// original: 0x00d85e50 guided_point_update (proposed)

/// Steer a guided point from three table-vector rows through gate checks and
/// a direction solve, updating counter and state bytes on the target object.
///
/// `obj` carries three 16-bit table-index pairs at `+0xDE0`/`+0xDE4`/`+0xDE8`
/// (low word selects a row base from the pointer table, high word a 32-byte
/// row), a weight byte at `+0xE6F`, a position block pointer at `+0x20`, a
/// signed key at `+0x2E`, and an object table pointer at `+0x0` whose slot
/// `+0xEC` is called when the solved point is far. `a1`/`a2`/`a3` are
/// out-words zeroed on the fail path, `a4` is passed to the report callee,
/// and `a5` is the target object (counter at `+0x27`, state at `+0x26`,
/// flags at `+0x2B`).
///
/// Behaviour: index `0xFFFF` or a null table entry for either of the first
/// two pairs takes the fail path (target flag bit, out-words zeroed). The
/// third pair is optional (missing stays null). Callee 1 resolves a table
/// entry; the first two rows' signed 16-bit pairs at `+0x14/+0x16`, scaled
/// by 0.125, form a direction that is normalised unless it is zero. Unless
/// the target flag bit 3 is set, each present row must answer 2 from callee
/// 2/3/4 or the batch is handed to callee 5 and the function returns; then
/// the resolved entry's flag bits either skip the energy test or run it
/// (callee 6 fills a triple, callee 7 gates on the key; within 1600 of the
/// position point and a positive cross product clears target flag bit 4).
/// Callee 8 solves a blend factor from the entry's packed fields, callee 9
/// fills the projection pair, and the projected point is pulled toward the
/// target by a factor capped below at 4.0. The counter clamps to 10, and to
/// 5 when the projection error is below 2.0. Callee 10 reports the steered
/// point with the six incoming arguments. A projection error of 1.0 or more
/// (or NaN) falls back to the object-table slot: its returned pair with
/// length below 1.0, like an error below 1.0, sets target state 5 and the
/// flag bit. NaN distances always take the far branch.
///
/// Original: 0x00d85e50 (cdecl, six stack words, no return value).
lf_checker_rt::export!(cdecl, rw_00d85e50(obj: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x0117_8284;
        const TABLE2: u32 = 0x0117_8384;
        const ROW_STRIDE: u32 = 32;
        const SCALE: f32 = f32::from_bits(0x3e00_0000); // 0.125
        const ONE: f32 = 1.0;
        const TWO: f32 = f32::from_bits(0x4000_0000); // 2.0
        const FOUR: f32 = f32::from_bits(0x4080_0000); // 4.0
        const SIX: f32 = f32::from_bits(0x40c0_0000); // 6.0
        const GATE2: f32 = f32::from_bits(0x44c8_0000); // 1600.0
        const PULL: f32 = f32::from_bits(0x4013_3333); // 2.3
        const SIGN: u32 = 0x8000_0000;
        const CAL_RESOLVE: u32 = 1;
        const CAL_ROW_A: u32 = 2;
        const CAL_ROW_B: u32 = 3;
        const CAL_ROW_C: u32 = 4;
        const CAL_BATCH: u32 = 5;
        const CAL_FILL: u32 = 6;
        const CAL_KEY: u32 = 7;
        const CAL_BLEND: u32 = 8;
        const CAL_PROJ: u32 = 9;
        const CAL_REPORT: u32 = 10;
        const CAL_FALLBACK: u32 = 11;
        const VT_SLOT: u32 = 0xec;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd16s(a: u32) -> f32 {
            unsafe { (a as *const i16).read_unaligned() as i32 as f32 }
        }
        #[inline(always)]
        unsafe fn rd16sx(a: u32) -> i32 {
            unsafe { (a as *const i16).read_unaligned() as i32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn neg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ SIGN)
        }

        let table = lf_checker_rt::relocated(TABLE);
        let table2 = lf_checker_rt::relocated(TABLE2);
        let full_a = rd32(obj + 0x0de0);
        let full_b = rd32(obj + 0x0de4);
        let full_c = rd32(obj + 0x0de8);
        let lo_a = full_a & 0xffff;
        let lo_b = full_b & 0xffff;
        if lo_a == 0xffff || lo_b == 0xffff {
            wr8(a5 + 0x2b, rd8(a5 + 0x2b) | 4);
            wr32(a3, 0);
            wr32(a2, 0);
            wr32(a1, 0);
            return 0;
        }
        let base_a = rd32(table.wrapping_add(lo_a.wrapping_mul(4)));
        if base_a == 0 {
            wr8(a5 + 0x2b, rd8(a5 + 0x2b) | 4);
            wr32(a3, 0);
            wr32(a2, 0);
            wr32(a1, 0);
            return 0;
        }
        let base_b = rd32(table.wrapping_add(lo_b.wrapping_mul(4)));
        if base_b == 0 {
            wr8(a5 + 0x2b, rd8(a5 + 0x2b) | 4);
            wr32(a3, 0);
            wr32(a2, 0);
            wr32(a1, 0);
            return 0;
        }
        let hub = lf_checker_rt::relocated(0x0117_7a80);
        let idx: u32 = lf_checker_rt::callee_thiscall!(CAL_RESOLVE, u32, hub, full_a, full_b);
        let row_a = base_a.wrapping_add((full_a >> 16).wrapping_mul(ROW_STRIDE));
        let row_b = base_b.wrapping_add((full_b >> 16).wrapping_mul(ROW_STRIDE));
        let entry_base = rd32(table2.wrapping_add(lo_a.wrapping_mul(4)));
        let row_c = {
            let lo_c = full_c & 0xffff;
            if lo_c == 0xffff {
                0
            } else {
                let base_c = rd32(table.wrapping_add(lo_c.wrapping_mul(4)));
                if base_c == 0 {
                    0
                } else {
                    base_c.wrapping_add((full_c >> 16).wrapping_mul(ROW_STRIDE))
                }
            }
        };
        let ax = mul(rd16s(row_a + 0x14), SCALE);
        let ay = mul(rd16s(row_a + 0x16), SCALE);
        let bx = mul(rd16s(row_b + 0x14), SCALE);
        let by = mul(rd16s(row_b + 0x16), SCALE);
        let dx = sub(bx, ax);
        let dy = sub(by, ay);
        let len2 = add(mul(dx, dx), mul(dy, dy));
        // Zero length stays zero; anything else (including NaN) normalises.
        let inv = if len2 == 0.0 { 0.0 } else { div(ONE, len2.sqrt()) };
        let nx = mul(inv, dx);
        let ny = mul(inv, dy);
        if rd8(a5 + 0x2b) & 8 == 0 {
            let ra: u32 = lf_checker_rt::callee_thiscall!(CAL_ROW_A, u32, row_a);
            if ra != 2 {
                return lf_checker_rt::callee_cdecl!(CAL_BATCH, u32, obj, a1, a2, a3, a4, a5);
            }
            let rb: u32 = lf_checker_rt::callee_thiscall!(CAL_ROW_B, u32, row_b);
            if rb != 2 {
                return lf_checker_rt::callee_cdecl!(CAL_BATCH, u32, obj, a1, a2, a3, a4, a5);
            }
            if row_c != 0 {
                let rc: u32 = lf_checker_rt::callee_thiscall!(CAL_ROW_C, u32, row_c);
                if rc != 2 {
                    return lf_checker_rt::callee_cdecl!(CAL_BATCH, u32, obj, a1, a2, a3, a4, a5);
                }
            }
            wr8(a5 + 0x2b, rd8(a5 + 0x2b) | 0x18);
            let entry = entry_base.wrapping_add(idx.wrapping_mul(8));
            if rd8(entry + 5) & 0x38 == 0 {
                let mut triple = [0.0f32; 3];
                lf_checker_rt::callee_cdecl!(CAL_FILL, u32, triple.as_mut_ptr() as u32);
                let key = rd16sx(obj + 0x2e);
                let gate: u32 = lf_checker_rt::callee_cdecl!(CAL_KEY, u32, key as u32, 0x18);
                if (gate as u8) != 0 {
                    let pos = rd32(obj + 0x20);
                    let ex = sub(triple[0], rdf(pos + 0x30));
                    let ey = sub(triple[1], rdf(pos + 0x34));
                    let ez = sub(triple[2], rdf(pos + 0x38));
                    let d2 = add(add(mul(ey, ey), mul(ex, ex)), mul(ez, ez));
                    if GATE2 > d2 {
                        let cross = sub(mul(sub(triple[1], ay), nx), mul(sub(triple[0], ax), ny));
                        if cross > 0.0 {
                            wr8(a5 + 0x2b, rd8(a5 + 0x2b) & 0xef);
                        }
                    } else {
                        let pos = rd32(obj + 0x20);
                        let cross = sub(
                            mul(sub(rdf(pos + 0x34), ay), nx),
                            mul(sub(rdf(pos + 0x30), ax), ny),
                        );
                        if cross > 0.0 {
                            wr8(a5 + 0x2b, rd8(a5 + 0x2b) & 0xef);
                        }
                    }
                } else {
                    let pos = rd32(obj + 0x20);
                    let cross = sub(
                        mul(sub(rdf(pos + 0x34), ay), nx),
                        mul(sub(rdf(pos + 0x30), ax), ny),
                    );
                    if cross > 0.0 {
                        wr8(a5 + 0x2b, rd8(a5 + 0x2b) & 0xef);
                    }
                }
            }
        }
        let entry = entry_base.wrapping_add(idx.wrapping_mul(8));
        let f5 = rd8(entry + 5);
        let f6 = rd8(entry + 6);
        let mut bf = [0.0f32; 2];
        lf_checker_rt::callee_cdecl!(
            CAL_BLEND, u32, (f5 & 7) as u32, ((f5 >> 3) & 7) as u32,
            ((f6 & 0x0f) as i32 as f32).to_bits(),
            bf.as_mut_ptr().wrapping_add(1) as u32, bf.as_mut_ptr() as u32,
            (f6 >> 7) as u32
        );
        let sel = if rd8(a5 + 0x2b) & 0x10 != 0 { bf[0] } else { neg(bf[1]) };
        let t = add(mul(ny, sel), ax);
        // Note: u overwrites the ay slot and the solve below uses nx, not u,
        // as the x multiplier (the two pushes shift the esp-relative slots).
        let u = sub(ay, mul(nx, sel));
        let pos = rd32(obj + 0x20);
        let mut proj = [0.0f32; 2];
        lf_checker_rt::callee_thiscall!(CAL_PROJ, u32, proj.as_mut_ptr() as u32, pos.wrapping_add(0x30), 0);
        let v0 = sub(proj[0], t);
        let v1 = sub(proj[1], u);
        let s = add(mul(v0, nx), mul(v1, ny));
        let px = add(mul(s, nx), t);
        let py = add(mul(s, ny), u);
        let ex = sub(proj[0], px);
        let ey = sub(proj[1], py);
        let d = add(mul(ey, ey), mul(ex, ex)).sqrt();
        let pulled = mul(d, PULL);
        let m = if FOUR > pulled { FOUR } else { pulled };
        let rx = add(mul(m, nx), px);
        let ry = add(mul(m, ny), py);
        let count = rd8(a5 + 0x27);
        wr8(a5 + 0x27, if count < 10 { count } else { 10 });
        if TWO > d {
            wr8(a5 + 0x27, if count < 5 { count } else { 5 });
        }
        let fe = rd8(obj + 0x0e6f) as i32 as f32;
        lf_checker_rt::callee_cdecl!(
            CAL_REPORT, u32, obj, 0, rx.to_bits(), ry.to_bits(),
            a1, a2, a3, a4, a5, fe.to_bits()
        );
        if ONE > d {
            wr8(a5 + 0x26, 5);
            wr8(a5 + 0x2b, rd8(a5 + 0x2b) | 4);
            return 0;
        }
        let slot: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(obj) + VT_SLOT) as usize);
        let scratch = [0u32; 8];
        let rp = slot(obj, scratch.as_ptr() as u32);
        let n2 = add(mul(rdf(rp), rdf(rp)), mul(rdf(rp + 4), rdf(rp + 4)));
        if ONE > n2.sqrt() {
            wr8(a5 + 0x26, 5);
            wr8(a5 + 0x2b, rd8(a5 + 0x2b) | 4);
        }
        0
    }
});
