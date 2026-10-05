// original: 0x00B3F120 ped_task_probe_region_grid (proposed)

/// Collect grid slots overlapping a region, shuffle them, and probe each one.
///
/// `cell` points at four floats (x, y, z, w). `a1`..`a3` are squared into
/// parameter slots, `radius` is the search radius, `a5` is stored as-is,
/// `a6` is an opaque word stored alongside, and `out` receives four floats
/// when a probe hits.
///
/// Behaviour: a one-time initialisation (guarded by a flag global) sets the
/// table counter/limit word, allocates the slot table through callee 2 and
/// runs two setup callees. A lookup callee maps the table base plus the
/// counter. Four grid indices are derived from `cell` (x/radius and
/// y/radius pairs through a round-towards-floor bit trick, scaled by the
/// reciprocal of a size global and clamped to the size): the outer pair
/// bounds rows, the inner pair columns. Every covered row/column cell stores
/// `size*row+col` into the table while the counter is below the limit. The
/// filled prefix is then shuffled with a scripted random callee
/// (index = col + trunc(rand16 * 2^-15 * remaining)). Each shuffled id is
/// resolved through callee 6; nulls are skipped and the rest are probed
/// through callee 7 with the object, its word at +0x70 and the minus/plus
/// box corners; any nonzero low byte latches the found flag. A teardown
/// callee always runs; on found, four globals are copied to `out`. Returns
/// the found flag (0 or 1).
///
/// Float order is the original's throughout: the rounding trick is emulated
/// operation by operation (add then subtract the magic, compare not-less,
/// subtract 0 or 1), integer conversions truncate with the 0x80000000
/// out-of-range result, and the shuffle multiplies rand * 2^-15 first.
///
/// Original: 0x00B3F120 (cdecl, eight stack words, returns the flag in al).
lf_checker_rt::export!(cdecl, rw_00B3F120(cell: u32, a1b: u32, a2b: u32, a3b: u32, radius_bits: u32, a5b: u32, a6: u32, out: u32) -> u32 {
    unsafe {
        const INIT_CALLEE: u32 = 1;
        const ALLOC_CALLEE: u32 = 2;
        const SETUP_CALLEE: u32 = 3;
        const LOOKUP_CALLEE: u32 = 4;
        const RAND_CALLEE: u32 = 5;
        const OBJECT_CALLEE: u32 = 6;
        const PROBE_CALLEE: u32 = 7;
        const BEGIN_CALLEE: u32 = 8;
        const END_CALLEE: u32 = 9;
        const G_FLAG: u32 = 0x1665404;
        const G_COUNTLIM: u32 = 0x1665400;
        const G_TABLE: u32 = 0x16653FC;
        const G_DIVISOR: u32 = 0x10482AC;
        const G_SIZE: u32 = 0x10482B0;
        const G_BOX: u32 = 0x16C8560;
        const G_HITPOS: u32 = 0x16C8570;
        const G_PARAMS: u32 = 0x16B7C38;
        const INIT_ARG: u32 = 0x16C84D0;
        const SETUP_ARG: u32 = 0xE72620;
        const ROUND_MAGIC: u32 = 0x4B00_0000; // 2^23
        const SHUF_SCALE: f32 = f32::from_bits(0x3800_0000); // 2^-15
        const GRID_MUL: f32 = f32::from_bits(0x3CA3_D70A); // 0.02
        const GRID_ADD: f32 = 60.0;
        const SIGN_BIT: u32 = 0x8000_0000;

        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fdiv(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        /// Truncate toward zero with cvttss2si semantics: NaN, infinities
        /// and out-of-range values yield 0x80000000, not a saturation.
        #[inline(always)]
        fn cvtt(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                i32::MIN
            } else {
                x as i32
            }
        }
        /// The original's round-to-floor bit trick, operation by operation:
        /// round to nearest with the 2^23 magic, then subtract 1 unless the
        /// rounded value is below the input (an exact integer rounds to
        /// itself and still subtracts, matching the not-less comparison).
        #[inline(always)]
        fn round_trick(v: f32) -> f32 {
            let sign_bits = v.to_bits() & SIGN_BIT;
            let abs = f32::from_bits(v.to_bits() ^ sign_bits);
            let mask: u32 = if abs < f32::from_bits(ROUND_MAGIC) { 0xFFFF_FFFF } else { 0 };
            let magic = f32::from_bits((ROUND_MAGIC & mask) | sign_bits);
            let r = fsub(fadd(v, magic), magic);
            let e = fsub(r, v);
            let below = e < f32::from_bits(sign_bits);
            let adj = if !below { 1.0f32 } else { 0.0f32 };
            fsub(r, adj)
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }

        let mut scratch = 0u32;
        lf_checker_rt::callee_thiscall!(INIT_CALLEE, u32, (&mut scratch as *mut u32) as u32,
            lf_checker_rt::relocated(INIT_ARG));
        let flag = lf_checker_rt::global::<u32>(G_FLAG).read();
        if flag & 1 == 0 {
            lf_checker_rt::global::<u32>(G_FLAG).write(flag | 1);
            lf_checker_rt::global::<u32>(G_COUNTLIM).write(0x0040_0000);
            let tp = lf_checker_rt::callee_cdecl!(ALLOC_CALLEE, u32, 0x100u32);
            lf_checker_rt::global::<u32>(G_TABLE).write(tp);
            lf_checker_rt::callee_cdecl!(SETUP_CALLEE, u32, lf_checker_rt::relocated(SETUP_ARG));
        }
        let table = lf_checker_rt::global::<u32>(G_TABLE).read();
        let counter = lf_checker_rt::global::<u32>(G_COUNTLIM).read() & 0xFFFF;
        let cursor = table.wrapping_add(counter.wrapping_mul(4));
        lf_checker_rt::callee_thiscall!(LOOKUP_CALLEE, u32, lf_checker_rt::relocated(G_TABLE), table, cursor);

        let divisor = lf_checker_rt::global::<i32>(G_DIVISOR).read();
        let size = lf_checker_rt::global::<i32>(G_SIZE).read();
        let scale = fdiv(1.0, divisor as f32);
        let x = rdf(cell);
        let y = rdf(cell.wrapping_add(4));
        let radius = f32::from_bits(radius_bits);
        let lo1 = {
            let v = fadd(fmul(fsub(x, radius), GRID_MUL), GRID_ADD);
            let j = cvtt(fmul(cvtt(round_trick(v)) as f32, scale));
            if j < 0 { 0 } else { let m = size.wrapping_sub(1); if j > m { m } else { j } }
        };
        let hi1 = {
            let v = fadd(fmul(fadd(x, radius), GRID_MUL), GRID_ADD);
            let j = cvtt(fmul(cvtt(round_trick(v)) as f32, scale));
            if j < 0 { 0 } else { let m = size.wrapping_sub(1); if j > m { m } else { j } }
        };
        let lo2 = {
            let v = fadd(fmul(fsub(y, radius), GRID_MUL), GRID_ADD);
            let j = cvtt(fmul(cvtt(round_trick(v)) as f32, scale));
            if j < 0 { 0 } else { let m = size.wrapping_sub(1); if j > m { m } else { j } }
        };
        let hi2 = {
            let v = fadd(fmul(fadd(y, radius), GRID_MUL), GRID_ADD);
            let j = cvtt(fmul(cvtt(round_trick(v)) as f32, scale));
            if j < 0 { 0 } else { let m = size.wrapping_sub(1); if j > m { m } else { j } }
        };

        if lo2 <= hi2 {
            let mut row = lo2;
            loop {
                if lo1 <= hi1 {
                    let mut col = lo1;
                    loop {
                        let cl = lf_checker_rt::global::<u32>(G_COUNTLIM).read();
                        let dx = (cl & 0xFFFF) as u16;
                        let lim = (cl >> 16) as u16;
                        if dx < lim {
                            let v = (size as u32).wrapping_mul(row as u32).wrapping_add(col as u32);
                            lf_checker_rt::global::<u32>(G_COUNTLIM)
                                .write((cl & 0xFFFF_0000) | (dx.wrapping_add(1) as u32));
                            (table.wrapping_add((dx as u32).wrapping_mul(4)) as *mut u32).write(v);
                        }
                        if col == hi1 {
                            break;
                        }
                        col = col.wrapping_add(1);
                    }
                }
                if row == hi2 {
                    break;
                }
                row = row.wrapping_add(1);
            }
        }

        let count = lf_checker_rt::global::<u32>(G_COUNTLIM).read() & 0xFFFF;
        if count != 0 {
            let mut rem = count as i32;
            let mut di = 0u32;
            loop {
                let rv = lf_checker_rt::callee_cdecl!(RAND_CALLEE, u32,) & 0xFFFF;
                let f = fmul(fmul((rv as i32) as f32, SHUF_SCALE), rem as f32);
                let k = cvtt(f).wrapping_add(di as i32);
                let pa = table.wrapping_add(di.wrapping_mul(4));
                let pb = table.wrapping_add((k as u32).wrapping_mul(4));
                let ta = (pa as *const u32).read();
                let tb = (pb as *const u32).read();
                (pa as *mut u32).write(tb);
                (pb as *mut u32).write(ta);
                di = di.wrapping_add(1);
                rem = rem.wrapping_sub(1);
                if di >= count {
                    break;
                }
            }
        }

        let x0 = rdf(cell);
        let x1 = rdf(cell.wrapping_add(4));
        let x2 = rdf(cell.wrapping_add(8));
        let x3 = rdf(cell.wrapping_add(12));
        lf_checker_rt::global::<u32>(G_BOX).write(x0.to_bits());
        lf_checker_rt::global::<u32>(G_BOX).wrapping_add(1).write(x1.to_bits());
        lf_checker_rt::global::<u32>(G_BOX).wrapping_add(2).write(x2.to_bits());
        lf_checker_rt::global::<u32>(G_BOX).wrapping_add(3).write(x3.to_bits());
        let minus = [fsub(x0, radius), fsub(x1, radius), fsub(x2, radius)];
        let plus = [fadd(x0, radius), fadd(x1, radius), fadd(x2, radius)];
        let r2 = fmul(radius, radius);
        let a1 = f32::from_bits(a1b);
        let a2 = f32::from_bits(a2b);
        let a3 = f32::from_bits(a3b);
        let gp = lf_checker_rt::global::<u32>(G_PARAMS);
        gp.wrapping_add(3).write(r2.to_bits());
        gp.wrapping_add(5).write(a6);
        gp.wrapping_add(0).write(fmul(a1, a1).to_bits());
        gp.wrapping_add(1).write(fmul(a2, a2).to_bits());
        gp.wrapping_add(2).write(fmul(a3, a3).to_bits());
        gp.wrapping_add(4).write(a5b);

        let mut found = 0u32;
        let mut s = 0u32;
        while s < count {
            let id = (table.wrapping_add(s.wrapping_mul(4)) as *const u32).read();
            s += 1;
            let obj = lf_checker_rt::callee_cdecl!(OBJECT_CALLEE, u32, id);
            if obj == 0 {
                continue;
            }
            let w70 = ((obj.wrapping_add(0x70)) as *const u32).read();
            let hit = lf_checker_rt::callee_cdecl!(PROBE_CALLEE, u32, obj, w70,
                minus.as_ptr() as u32, plus.as_ptr() as u32);
            if hit & 0xFF != 0 {
                found = 1;
            }
        }
        lf_checker_rt::callee_thiscall!(BEGIN_CALLEE, u32, (&mut scratch as *mut u32) as u32);
        if found != 0 {
            let hp = lf_checker_rt::global::<u32>(G_HITPOS);
            (out as *mut u32).write_unaligned(hp.read());
            (out.wrapping_add(4) as *mut u32).write_unaligned(hp.wrapping_add(1).read());
            (out.wrapping_add(8) as *mut u32).write_unaligned(hp.wrapping_add(2).read());
            (out.wrapping_add(12) as *mut u32).write_unaligned(hp.wrapping_add(3).read());
            lf_checker_rt::callee_thiscall!(END_CALLEE, u32, (&mut scratch as *mut u32) as u32);
            1
        } else {
            lf_checker_rt::callee_thiscall!(END_CALLEE, u32, (&mut scratch as *mut u32) as u32);
            0
        }
    }
});
