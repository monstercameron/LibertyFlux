// original: 0x00b0b590 net_probe_rows_around (proposed)

/// Probe around the given position and attach rows found nearby.
///
/// Runs up to seven attempts (fewer for small `count`: the attempt count is
/// the magic-division estimate capped at seven, and non-positive counts do
/// nothing). Each attempt jitters the input position with two scripted
/// random draws shaped by the shaping callees, picks one of two height
/// biases, and offers the point to the probe callee, which answers a height
/// and a found flag. Attempts the probe rejects end there. Two wrinkles
/// shape later iterations: the probe's flag store overlaps the stacked
/// register save and zeroes it, and attempts that end early skip the
/// register restores, so the keep test and the make argument after the
/// first attempt see stale register values, tracked exactly here.
///
/// An accepted point with a live session goes through the make/use callee
/// pair, yielding a table row; the row's anchor triple seeds the inner
/// search, which scans downward in fixed steps for a live handle and links
/// the first one found, recording the finish callee's answer in the row.
/// Rows whose link flag stays clear are left alone.
///
/// Original: 0x00b0b590 (cdecl, two stack words).
export!(cdecl, rw_00b0b590(pos: u32, count: u32) -> u32 {
    unsafe {
        const MAGIC: i64 = 0x66666667;
        const MAX_ATTEMPTS: i32 = 7;
        const ROW_STRIDE: u32 = 80;
        const TABLE: u32 = 0x1615660;
        const OFF_A: u32 = 0x18;
        const OFF_B: u32 = 0x1c;
        const OFF_C: u32 = 0x20;
        const OFF_LINKFLAG: u32 = 0x08;
        const OFF_FINISH: u32 = 0x0c;
        const SESSION: u32 = 0x12fa500;
        const MODEFLAG: u32 = 0x1bb5624;
        const JITTER_SCALE: u32 = 0xeaabbc;
        const JITTER_GAIN: u32 = 0xfe8960;
        const HEIGHT_A: u32 = 0xfe8a24;
        const HEIGHT_B: u32 = 0xfe8ad8;
        const PROBE_BIAS: u32 = 0xfe876c;
        const KEEP_REF: u32 = 0xfe87e8;
        const KEEP_SCALE: u32 = 0xfe8684;
        const FIND_ARG: u32 = 0x3dcccccd;
        const INNER_ARG: u32 = 0x40000000;
        const INNER_STEP: u32 = 0xfe8a24;
        const U64_TABLE: u32 = 0xfe8f50;
        const FIND_HANDLE_OFF: u32 = 0x73;
        const C_RAND1: u32 = 1;
        const C_FX: u32 = 2;
        const C_RAND2: u32 = 3;
        const C_FY: u32 = 4;
        const C_FLAGA: u32 = 5;
        const C_PROBE: u32 = 6;
        const C_RAND3: u32 = 7;
        const C_RAND4: u32 = 8;
        const C_MK: u32 = 9;
        const C_USE: u32 = 10;
        const C_FLAGB: u32 = 11;
        const C_COND: u32 = 12;
        const C_FIND: u32 = 13;
        const C_INNER: u32 = 14;
        const C_LINK: u32 = 15;
        const C_FIN: u32 = 16;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn g32(va: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(va) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        fn gf(va: u32) -> f32 {
            f32::from_bits(g32(va))
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        // Attempt count estimate: magic-division step, capped at seven.
        let x = count as i32;
        let hi = ((x as i64).wrapping_mul(MAGIC) >> 32) as i32;
        let old = hi >> 3;
        let est = ((old as u32) >> 31) as i32 + old.wrapping_add(1);
        let mut attempts = MAX_ATTEMPTS;
        if est < attempts {
            attempts = est;
        }
        if attempts <= 0 {
            return 0;
        }
        let per = x.wrapping_div(attempts);
        let mut edi_cur = attempts;
        let mut edi_save = attempts;
        let mut esi_cur = per;
        let px = rdf(pos);
        let py = rdf(pos + 4);
        let pz = rdf(pos + 8);
        let mut left = attempts;
        while left > 0 {
            left -= 1;
            let r1: u32 = lf_checker_rt::callee_cdecl!(C_RAND1, u32,);
            let jx = mul((r1 & 0xff) as f32, gf(JITTER_SCALE));
            let fxr: u32 = lf_checker_rt::callee_cdecl!(C_FX, u32, jx.to_bits());
            let jxp = add(mul(f32::from_bits(fxr), gf(JITTER_GAIN)), px);
            let r2: u32 = lf_checker_rt::callee_cdecl!(C_RAND2, u32,);
            let jy = mul((r2 & 0xff) as f32, gf(JITTER_SCALE));
            let fyr: u32 = lf_checker_rt::callee_cdecl!(C_FY, u32, jy.to_bits());
            let jyp = add(mul(f32::from_bits(fyr), gf(JITTER_GAIN)), py);
            let fa: u32 = lf_checker_rt::callee_cdecl!(C_FLAGA, u32,);
            let href = if (fa as u8) != 0 { gf(HEIGHT_A) } else { gf(HEIGHT_B) };
            let jzp = add(core::hint::black_box(href), core::hint::black_box(pz));
            let mut pslot = [0u32; 1];
            let pf: f32 = lf_checker_rt::callee_cdecl!(
                C_PROBE, f32, jxp.to_bits(), jyp.to_bits(), jzp.to_bits(),
                pslot.as_mut_ptr() as u32, 0, 4);
            let _ph = add(core::hint::black_box(pf), core::hint::black_box(gf(PROBE_BIAS)));
            edi_save = 0; // flag word overlaps and zeroes the stacked edi save
            if (pslot.as_mut_ptr() as *const u8).read() == 0 {
                continue;
            }
            if g32(SESSION) == 0xffffffff {
                continue;
            }
            let mut keep = 0u8;
            if edi_cur < 5 {
                keep = 1;
            } else {
                let r3: u32 = lf_checker_rt::callee_cdecl!(C_RAND3, u32,);
                let t = mul((r3 as i32) as f32, gf(KEEP_SCALE));
                if gf(KEEP_REF) > t {
                    keep = 1;
                }
            }
            core::hint::black_box(keep);
            let r4: u32 = lf_checker_rt::callee_cdecl!(C_RAND4, u32,);
            let mut slot2 = [0u32; 1];
            let mut slot1 = [0u32; 3];
            let made: u32 = lf_checker_rt::callee_cdecl!(
                C_MK, u32, slot2.as_mut_ptr() as u32, slot1.as_mut_ptr() as u32,
                g32(SESSION), 8, 0, esi_cur.wrapping_add((r4 & 3) as i32) as u32,
                0, 0, 0, 0, 0, keep as u32, 0);
            let row: u32 = lf_checker_rt::callee_cdecl!(C_USE, u32, made);
            esi_cur = row as i32;
            let fb: u32 = lf_checker_rt::callee_cdecl!(C_FLAGB, u32,);
            let mut inner_n = 1u32;
            edi_cur = 1;
            if (fb as u8) != 0 {
                let cr: u32 = lf_checker_rt::callee_thiscall!(C_COND, u32, g32(MODEFLAG), 2);
                if (cr as u8) != 0 {
                    inner_n = 10;
                    edi_cur = 10;
                }
            }
            let rowptr = lf_checker_rt::relocated(TABLE).wrapping_add(row.wrapping_mul(ROW_STRIDE));
            let mut fslot = [0u32; 3];
            fslot[0] = rd32(rowptr + OFF_A);
            fslot[1] = rd32(rowptr + OFF_B);
            fslot[2] = rd32(rowptr + OFF_C);
            let found: u32 = lf_checker_rt::callee_cdecl!(
                C_FIND, u32, fslot.as_mut_ptr() as u32, FIND_ARG);
            if found == 0 {
                continue;
            }
            if ((found + FIND_HANDLE_OFF) as *const u8).read() == 0 {
                continue;
            }
            esi_cur = 0;
            slot2[0] = 0; // inner setup zeroes the scratch slot (wipes any make out-word)
            if inner_n == 0 {
                continue;
            }
            let mut i = 0u32;
            let mut handle = 0u32;
            while i < inner_n {
                let base = fslot[2];
                let sel = (i >> 31) as usize;
                let dbo = (lf_checker_rt::global::<u64>(U64_TABLE) as *const u64)
                    .wrapping_add(sel).read_unaligned();
                let ef = ((i as f64) + f64::from_bits(dbo)) as f32;
                let arg = sub(f32::from_bits(base), mul(ef, gf(INNER_STEP)));
                let mut islot = [fslot[0], fslot[1], arg.to_bits()];
                let mut zslot = [0u32; 1];
                let h: u32 = lf_checker_rt::callee_cdecl!(
                    C_INNER, u32, islot.as_mut_ptr() as u32, zslot.as_mut_ptr() as u32, INNER_ARG);
                handle = h;
                if h != 0 {
                    break;
                }
                i += 1;
            }
            esi_cur = i as i32;
            if handle == 0 {
                continue;
            }
            esi_cur = row.wrapping_mul(ROW_STRIDE) as i32;
            let z0 = slot2[0];
            let _: u32 = lf_checker_rt::callee_thiscall!(C_LINK, u32, rowptr, handle, z0);
            if rd32(rowptr + OFF_LINKFLAG) == 0 {
                continue;
            }
            let fin: u32 = lf_checker_rt::callee_thiscall!(C_FIN, u32, handle, z0);
            wr32(rowptr + OFF_FINISH, fin);
            esi_cur = per;
            edi_cur = edi_save;
        }
        0
    }
});
