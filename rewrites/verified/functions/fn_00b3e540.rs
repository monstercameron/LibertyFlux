// original: 0x00B3E540 worklist_scan_and_record (proposed)

/// Work-list-driven scan over scope/member pairs that records the nearest
/// passing candidate into the shared result cells.
///
/// Arguments (cdecl, no stack words read): the two seed words are taken
/// from below the incoming stack pointer, where the caller leaves frame
/// scratch; the contract pins the stack fill to zero so they read 0, and
/// the rewrite uses 0 with that noted as a narrowing, since the checker
/// offers no transport for below-stack inputs. Returns leftover register
/// state that no caller reads; the contract does not compare it.
///
/// The shared count cell seeds a stack of scope/slot pairs (at most 256).
/// Each iteration pops one pair and, unless a flag-gated bit test rejects
/// the slot, walks the slot's sub-range through a callee and asks a mixer
/// callee how many table rows to scan. Every row whose position is within
/// the stored radius, passes the optional quantized-bounds callee and the
/// per-entry distance gates, and improves the stored best distance,
/// overwrites the result slot, result vector, result index and best
/// distance. Afterwards the slot's neighbour list is merged into two
/// accumulators (seeded by the entry seeds on the first iteration); each
/// merged-in scope id that resolves, is unstamped, passes the flag gates
/// and the stored 16-bit bounds, and fits the stack is pushed back. The
/// scan ends when the count reaches zero.
///
/// One mixer argument carries three uninitialized frame bytes above a flag
/// byte; the pinned zero fill makes it exactly the flag, which the rewrite
/// passes. Float order is the original's throughout.
///
/// Original: 0x00B3E540 (cdecl; reads no incoming stack words).
lf_checker_rt::export!(cdecl, rw_00B3E540() -> u32 {
    unsafe {
        const COUNT: u32 = 0x16646B0;
        const WORK_PTR: u32 = 0x16646B8;
        const WORK_SLOT: u32 = 0x16646BC;
        const WORK_MAX: i32 = 0x100;
        const FLAG0: u32 = 0x16646A0;
        const FLAG1: u32 = 0x16646A1;
        const FLAG2: u32 = 0x16646A2;
        const FLAG3: u32 = 0x16646A3;
        const FLAG4: u32 = 0x16646A4;
        const SUB_TABLE: u32 = 0x1664EC0;
        const ROW_TABLE: u32 = 0x1664FC0;
        const ROW_FIRST: u32 = 0x1664FC4;
        const ROW_STRIDE: u32 = 0x10;
        const QUERY_X: u32 = 0x16653D0;
        const QUERY_Y: u32 = 0x16653D4;
        const QUERY_Z: u32 = 0x16653D8;
        const QUERY_RAD2: u32 = 0x1664688;
        const AUX_COUNT: u32 = 0x16646A8;
        const AUX_BASE: u32 = 0x16646AC;
        const BEST: u32 = 0x166468C;
        const RESULT_SLOT: u32 = 0x1664698;
        const RESULT_ID12: u32 = 0x1664690;
        const RESULT_X: u32 = 0x16653E0;
        const RESULT_Y: u32 = 0x16653E4;
        const RESULT_Z: u32 = 0x16653E8;
        const RESULT_W: u32 = 0x16653EC;
        const RESULT_IDX: u32 = 0x1664694;
        const BOUND_LO_X: u32 = 0x16653F0;
        const BOUND_HI_X: u32 = 0x16653F2;
        const BOUND_LO_Y: u32 = 0x16653F4;
        const BOUND_HI_Y: u32 = 0x16653F6;
        const BOUND_LO_Z: u32 = 0x16653F8;
        const BOUND_HI_Z: u32 = 0x16653FA;
        const STAMP: u32 = 0x16C7472;
        const QUANT_XY: u32 = 0xFE87E4;
        const QUANT_Z: u32 = 0xFE8960;
        const QUANT_GAIN: u32 = 0xFE8AFc;
        const ENTRY_GAP: u32 = 0xFE8A94;
        const SCOPE_SUBS: u32 = 0x60;
        const SCOPE_MEMBERS: u32 = 0x6C;
        const SCOPE_LINKS: u32 = 0x64;
        const MEMBER_STRIDE: u32 = 40;
        const MEMBER_BITS: u32 = 0x1C;
        const NO_ID12: u32 = 0xFFF;
        const WALK_SUB: u32 = 1;
        const MIX_ROWS: u32 = 2;
        const BOUNDS_OK: u32 = 3;
        const LOOKUP_SCOPE: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u32 {
            unsafe { (a as *const u8).read() as u32 }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn g8(va: u32) -> u32 {
            unsafe { rd8(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn g16(va: u32) -> u32 {
            unsafe { rd16(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits(g32(va)) }
        }
        #[inline(always)]
        unsafe fn gw32(va: u32, v: u32) {
            unsafe { (lf_checker_rt::relocated(va) as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn gwf(va: u32, v: f32) {
            unsafe { gw32(va, v.to_bits()) }
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn cvtt(v: f32) -> i32 {
            if v.is_nan() || v >= 2147483648.0 || v < -2147483648.0 {
                i32::MIN
            } else {
                v as i32
            }
        }
        #[inline(always)]
        fn above(a: f32, b: f32) -> bool {
            core::hint::black_box(a) > core::hint::black_box(b)
        }
        #[inline(always)]
        fn below_eq(a: f32, b: f32) -> bool {
            !(core::hint::black_box(a) > core::hint::black_box(b))
        }
        #[inline(always)]
        fn div40(v: u32) -> u32 {
            let q = (v as i32).wrapping_mul(0x66666667) >> 4;
            q.wrapping_add((q as u32 >> 31) as i32) as u32
        }

        // Below-stack seeds; pinned to zero by the stack fill.
        let mut acc_x = 0u32;
        let mut acc_y = 0u32;
        loop {
            if (g32(COUNT) as i32) <= 0 {
                break;
            }
            let c = g32(COUNT).wrapping_sub(1);
            let ptr = rd32(
                lf_checker_rt::relocated(WORK_PTR).wrapping_add(c.wrapping_mul(8)),
            );
            let slot = rd32(
                lf_checker_rt::relocated(WORK_SLOT).wrapping_add(c.wrapping_mul(8)),
            );
            gw32(COUNT, c);
            let mut gated = false;
            if g8(FLAG0) != 0 && (rd32(slot) >> 2) & 1 == 0 {
                gated = true;
            }
            if !gated && g8(FLAG1) != 0 && (rd8(slot.wrapping_add(MEMBER_BITS)) >> 7) & 1 != 0
            {
                gated = true;
            }
            if !gated && g8(FLAG2) != 0 && (rd8(slot.wrapping_add(MEMBER_BITS)) >> 6) & 1 != 0
            {
                gated = true;
            }
            if !gated && g8(FLAG3) != 0 && (rd32(slot) >> 7) & 1 != 0 {
                gated = true;
            }
            if !gated {
                let sub_count = (rd32(slot) >> 0x15) & 0xf;
                if (sub_count as i32) > 0 {
                    let mut j = 0u32;
                    while j < sub_count {
                        let tp = lf_checker_rt::relocated(SUB_TABLE)
                            .wrapping_add(j.wrapping_mul(ROW_STRIDE));
                        let cc = (rd32(slot.wrapping_add(4)) & 0x1ffff).wrapping_add(j);
                        let subs = rd32(ptr.wrapping_add(SCOPE_SUBS));
                        let w = rd16(subs.wrapping_add(cc.wrapping_mul(2)));
                        lf_checker_rt::callee_thiscall!(WALK_SUB, u32, ptr, w, tp);
                        j += 1;
                    }
                }
                let flag_bit = (rd32(slot) >> 1) & 1;
                let links = rd32(ptr.wrapping_add(SCOPE_LINKS));
                let lc = rd32(slot.wrapping_add(4)) & 0x1ffff;
                let rows = lf_checker_rt::callee_cdecl!(
                    MIX_ROWS,
                    u32,
                    slot,
                    lf_checker_rt::relocated(SUB_TABLE),
                    links.wrapping_add(lc.wrapping_mul(8)),
                    0,
                    flag_bit,
                    lf_checker_rt::relocated(ROW_TABLE)
                );
                if (rows as i32) > 0 {
                    let mut t = 0u32;
                    while t < rows {
                        let vp = lf_checker_rt::relocated(ROW_FIRST)
                            .wrapping_add(t.wrapping_mul(ROW_STRIDE));
                        let vx = f32::from_bits(rd32(vp.wrapping_sub(4)));
                        let vy = f32::from_bits(rd32(vp));
                        let vz = f32::from_bits(rd32(vp.wrapping_add(4)));
                        let dx = fsub(vx, gf(QUERY_X));
                        let dy = fsub(vy, gf(QUERY_Y));
                        let dz = fsub(vz, gf(QUERY_Z));
                        let d = fadd(
                            fadd(fmul(dx, dx), fmul(dy, dy)),
                            fmul(dz, dz),
                        );
                        if above(d, gf(QUERY_RAD2)) {
                            t += 1;
                            continue;
                        }
                        if g8(FLAG4) != 0 {
                            let r0 = gf(QUANT_XY);
                            let r1 = gf(QUANT_Z);
                            let gain = gf(QUANT_GAIN);
                            let mut words = [0u16; 6];
                            words[0] = cvtt(fmul(fsub(vx, r0), gain)) as u16;
                            words[1] = cvtt(fmul(fadd(vx, r0), gain)) as u16;
                            words[2] = cvtt(fmul(fsub(vy, r0), gain)) as u16;
                            words[3] = cvtt(fmul(fadd(vy, r0), gain)) as u16;
                            words[4] = cvtt(fmul(fsub(vz, r1), gain)) as u16;
                            words[5] = cvtt(fmul(fadd(vz, r1), gain)) as u16;
                            let ok = lf_checker_rt::callee_stdcall!(
                                BOUNDS_OK,
                                u32,
                                words.as_mut_ptr() as u32
                            );
                            if ok & 0xff != 0 {
                                t += 1;
                                continue;
                            }
                        }
                        let limit = g32(AUX_COUNT);
                        let mut jj = 0u32;
                        if (limit as i32) > 0 {
                            let base = g32(AUX_BASE);
                            while jj < limit {
                                let ap = base
                                    .wrapping_add(0x10)
                                    .wrapping_add(jj.wrapping_mul(0x20));
                                let q0 = f32::from_bits(rd32(ap));
                                let q1 = f32::from_bits(rd32(ap.wrapping_sub(0x10)));
                                let q2 = f32::from_bits(rd32(ap.wrapping_sub(0xc)));
                                let dd = fadd(
                                    fmul(fsub(vy, q2), fsub(vy, q2)),
                                    fmul(fsub(vx, q1), fsub(vx, q1)),
                                );
                                if below_eq(fmul(q0, q0), dd) {
                                    jj += 1;
                                    continue;
                                }
                                let tz = f32::from_bits(rd32(ap.wrapping_sub(8)));
                                let gap = (fsub(vz, tz)).abs();
                                if above(gf(ENTRY_GAP), gap) {
                                    break;
                                }
                                jj += 1;
                            }
                        }
                        if jj != limit {
                            t += 1;
                            continue;
                        }
                        if below_eq(gf(BEST), d) {
                            t += 1;
                            continue;
                        }
                        gw32(RESULT_SLOT, slot);
                        gwf(BEST, d);
                        gw32(RESULT_ID12, (rd32(slot.wrapping_add(4)) >> 0x11) & 0xfff);
                        gwf(RESULT_X, vx);
                        gwf(RESULT_Y, vy);
                        gwf(RESULT_Z, vz);
                        gwf(RESULT_W, f32::from_bits(rd32(vp.wrapping_add(8))));
                        let base = rd32(ptr.wrapping_add(SCOPE_MEMBERS));
                        gw32(RESULT_IDX, div40(slot.wrapping_sub(base)));
                        t += 1;
                    }
                }
            }
            // Neighbour merge; always runs, even for a gated slot.
            let mut low = acc_y | 0xffff0fff;
            let mut high = (acc_x | 0x0fffffff) & 0xefffffff;
            acc_x = high;
            let merged_loop_ran = rd32(slot) & 0x1e00000 != 0;
            if merged_loop_ran {
                let iters = (rd32(slot) >> 0x15) & 0xf;
                let mut i = 0u32;
                while i < iters {
                    let links = rd32(ptr.wrapping_add(SCOPE_LINKS));
                    let s = (rd32(slot.wrapping_add(4)) & 0x1ffff).wrapping_add(i);
                    let w0 = rd32(links.wrapping_add(s.wrapping_mul(8)));
                    // Bit-by-bit merge of the low 16 bits from the link word.
                    low = (low & !0xfff) | (w0 & 0xfff);
                    low = (low & !0x1000) | (w0 & 0x1000);
                    low = (low & !0x6000) | (w0 & 0x6000);
                    low = (low & !0x8000) | (w0 & 0x8000);
                    low = (low & !0xffff) | (w0 & 0xffff);
                    let w1 = rd32(links.wrapping_add(s.wrapping_mul(8)).wrapping_add(4));
                    high = (high & !0xfff) | (w1 & 0xfff);
                    high = (high & !0xffff000) | (w1 & 0xffff000);
                    high = (high & !0x10000000) | (w1 & 0x10000000);
                    acc_x = high;
                    let id2 = high & 0xfff;
                    if id2 != NO_ID12 {
                        let scope2 = lf_checker_rt::callee_cdecl!(LOOKUP_SCOPE, u32, id2);
                        if scope2 != 0 {
                            let idx2 = (high >> 0xc) & 0xffff;
                            let m2 = rd32(scope2.wrapping_add(SCOPE_MEMBERS));
                            let slot2 = m2.wrapping_add(idx2.wrapping_mul(MEMBER_STRIDE));
                            let stamp = g16(STAMP);
                            if rd16(slot2.wrapping_add(0xa)) != stamp {
                                (slot2.wrapping_add(0xa) as *mut u16)
                                    .write_unaligned(stamp as u16);
                                let mut blocked = false;
                                if g8(FLAG2) != 0
                                    && (rd8(slot2.wrapping_add(MEMBER_BITS)) >> 6) & 1 != 0
                                {
                                    blocked = true;
                                }
                                if !blocked
                                    && g8(FLAG3) != 0
                                    && (rd32(slot2) >> 7) & 1 != 0
                                {
                                    blocked = true;
                                }
                                if !blocked {
                                    let in_hi = (rd16(slot2.wrapping_add(0x10)) as u16 as i16)
                                        <= (g16(BOUND_HI_X) as u16 as i16)
                                        && (rd16(slot2.wrapping_add(0x14)) as u16 as i16)
                                            <= (g16(BOUND_HI_Y) as u16 as i16)
                                        && (rd16(slot2.wrapping_add(0x18)) as u16 as i16)
                                            <= (g16(BOUND_HI_Z) as u16 as i16);
                                    let in_lo = (rd16(slot2.wrapping_add(0x12)) as u16 as i16)
                                        >= (g16(BOUND_LO_X) as u16 as i16)
                                        && (rd16(slot2.wrapping_add(0x16)) as u16 as i16)
                                            >= (g16(BOUND_LO_Y) as u16 as i16)
                                        && (rd16(slot2.wrapping_add(0x1a)) as u16 as i16)
                                            >= (g16(BOUND_LO_Z) as u16 as i16);
                                    if in_hi && in_lo {
                                        let n = g32(COUNT);
                                        if (n as i32) < WORK_MAX {
                                            gw32(COUNT, n.wrapping_add(1));
                                            gw32(
                                                WORK_PTR.wrapping_add(n.wrapping_mul(8)),
                                                scope2,
                                            );
                                            gw32(
                                                WORK_SLOT.wrapping_add(n.wrapping_mul(8)),
                                                slot2,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                    i += 1;
                }
                acc_y = low;
            }
        }
        0
    }
});
