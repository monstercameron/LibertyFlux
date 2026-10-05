// original: 0x00DC3C50 ped_task_aim_or_track_update (proposed)

/// Update a ped task's aim or tracking state, then refresh the shared target.
///
/// `a0` points at status flags, `a1` at the task, `a2` at the query object,
/// `a3` at mode flags, `a4` at three floats (aim point), `a5` at the output
/// float and `a6` at the output dword. Callee 1 (thiscall on the manager at
/// `MANAGER`, the state object from `G_STATE` and `a3`) must answer nonzero,
/// else the function returns 2. When the state flag `0x40` is set and the
/// mode word carries both `0x6000` bits, the enable byte at `G_ENABLE` is
/// set to 1 if the high bit of `[a0+0x1C]` is set, else left alone.
///
/// The switch at `G_MODE` and the enable byte pick the worker path: the
/// near path (mode off, or enable on) or the far path (mode on, enable
/// off). The near path re-checks bit 12 of the task word (set rejoins the
/// far path); then either the fast shape (byte at `G_FAST` nonzero: callee
/// 2 on `a2`/`a1`/`a4`, squared distance from the aim point to the state's
/// `+0x50` offsets into `a5`, `0x7F` into `a6`) or the query shape (callee
/// 3, then callee 4 with seven stack words whose float result lands in
/// `a5`). A positive `a5` is replaced by 1 over callee 5's answer on its
/// bits; the near path always returns 0.
///
/// The far path calls callee 3, then continues inward only when `a1` equals
/// both TC slots `G_TC0`/`G_TC1` and callee 6 (three stack words: a bit
/// from the state word, state `+0x50`, state `+0x40`) answers zero: callee
/// 4 fills `a5` and the same scale-or-keep applies, returning 0. Otherwise
/// the outer shape runs: callee 7 (eight stack words, the inverted low
/// task bit, the row pointer, the reference globals at `G_REF`) fills the
/// reference and the state offsets, the squared distance from the aim
/// point lands in `a5` with the same scaling, and the return is 1 when
/// callee 7's answer byte is zero, else 0.
///
/// Every worker path ends at one tail: the aim point minus the reference
/// lands at `G_OUT`, a zero seed (the original loads it from a stack slot
/// it never wrote; the contract fills unread stack with zero) lands at
/// `G_SEED`, and `G_LEN` gets 1 over callee 5's answer on the squared
/// length, or 0 when it is not positive. Returns 2 for the early exit.
///
/// Original: 0x00DC3C50 (cdecl, seven stack words). True size 910 bytes;
/// the batch list's 725 cut off the shared tail.
lf_checker_rt::export!(cdecl, rw_00DC3C50(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    unsafe {
        const MANAGER: u32 = 0x016C6830;
        const G_STATE: u32 = 0x016C7448;
        const G_ENABLE: u32 = 0x016B6B23;
        const G_MODE: u32 = 0x0104827C;
        const G_FAST: u32 = 0x01048272;
        const G_SLOT_A: u32 = 0x017A1DD0;
        const G_REF: u32 = 0x017A22E0;
        const G_TC0: u32 = 0x017A2340;
        const G_TC1: u32 = 0x017A2344;
        const G_OUT: u32 = 0x017A2300;
        const G_SEED: u32 = 0x017A230C;
        const G_LEN: u32 = 0x017A3390;
        const STATE_FLAGS: u32 = 0x14;
        const STATE_VEC: u32 = 0x50;
        const STATE_ALT: u32 = 0x40;
        const READY_BITS: u32 = 0x6000;
        const READY_FLAG: u32 = 0x40;
        const OFF_KEY: u32 = 0x64;
        const KEY_MASK: u32 = 0x1FFFF;
        const ROW_STRIDE: u32 = 8;
        const ONE: f32 = 1.0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
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
        /// `[a2+0x64] + ([a1+4] & mask) * stride`, wrapping.
        #[inline(always)]
        unsafe fn row(a1: u32, a2: u32) -> u32 {
            unsafe {
                rd32(a2.wrapping_add(OFF_KEY)).wrapping_add(
                    (rd32(a1.wrapping_add(4)) & KEY_MASK).wrapping_mul(ROW_STRIDE),
                )
            }
        }
        /// Replace a positive slot by 1 over the scaler's answer on its bits.
        #[inline(always)]
        unsafe fn scale_slot(slot: u32) {
            unsafe {
                let v = rdf(slot);
                if v > 0.0 {
                    let f: f32 = lf_checker_rt::callee_cdecl!(5, f32, v.to_bits());
                    wrf(slot, div(ONE, f));
                }
            }
        }
        /// Far worker path. Returns 0 on the inner shape, `status == 0`
        /// on the outer shape.
        #[inline(always)]
        unsafe fn far_path(mgr: u32, state: u32, a1: u32, a2: u32, a4: u32, a5: u32, a6: u32) -> u32 {
            unsafe {
                let _: u32 = lf_checker_rt::callee_cdecl!(3, u32, a2, a1);
                let inner = a1 == rd32(lf_checker_rt::relocated(G_TC0))
                    && a1 == rd32(lf_checker_rt::relocated(G_TC1));
                if inner {
                    let bit = (rd32(state.wrapping_add(STATE_FLAGS)) >> 9) & 1;
                    let probe: u32 = lf_checker_rt::callee_thiscall!(6, u32, mgr,
                        state.wrapping_add(STATE_ALT), state.wrapping_add(STATE_VEC), bit);
                    if probe as u8 == 0 {
                        let q: f32 = lf_checker_rt::callee_thiscall!(4, f32,
                            mgr, a1, lf_checker_rt::relocated(G_SLOT_A), row(a1, a2),
                            state.wrapping_add(STATE_ALT), a4, a6, 0);
                        wrf(a5, q);
                        scale_slot(a5);
                        return 0;
                    }
                }
                // Outer shape.
                let fb = (!rd8(a1)) & 1;
                let status: u32 = lf_checker_rt::callee_thiscall!(7, u32, mgr, a1,
                    lf_checker_rt::relocated(G_SLOT_A), row(a1, a2),
                    lf_checker_rt::relocated(G_REF), state.wrapping_add(STATE_VEC),
                    a4, a6, fb as u32);
                let dx = sub(rdf(a4), rdf(state.wrapping_add(STATE_VEC)));
                let dy = sub(rdf(a4.wrapping_add(4)), rdf(state.wrapping_add(STATE_VEC + 4)));
                let dz = sub(rdf(a4.wrapping_add(8)), rdf(state.wrapping_add(STATE_VEC + 8)));
                wrf(a5, add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz)));
                scale_slot(a5);
                if status as u8 == 0 { 1 } else { 0 }
            }
        }

        let mgr = lf_checker_rt::relocated(MANAGER);
        let state = rd32(lf_checker_rt::relocated(G_STATE));
        let gate: u32 = lf_checker_rt::callee_thiscall!(1, u32, mgr, state, a3);
        if gate as u8 == 0 {
            return 2;
        }
        let g_en = lf_checker_rt::relocated(G_ENABLE);
        let mut en = rd8(g_en);
        if rd32(state.wrapping_add(STATE_FLAGS)) & READY_FLAG != 0
            && rd32(a3) & READY_BITS == READY_BITS
        {
            if rd8(a0.wrapping_add(0x1C)) >> 7 != 0 {
                en = 1;
            }
            wr8(g_en, en);
        }
        // Worker path. Near returns 0; far-outer returns status==0.
        let near = rd32(lf_checker_rt::relocated(G_MODE)) == 0 || en != 0;
        let result: u32 = if near && (rd32(a1) >> 12) & 1 == 0 {
            if rd8(lf_checker_rt::relocated(G_FAST)) != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, a2, a1, a4);
                let dx = sub(rdf(state.wrapping_add(STATE_VEC)), rdf(a4));
                let dy = sub(rdf(state.wrapping_add(STATE_VEC + 4)), rdf(a4.wrapping_add(4)));
                let dz = sub(rdf(state.wrapping_add(STATE_VEC + 8)), rdf(a4.wrapping_add(8)));
                wrf(a5, add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz)));
                wr32(a6, 0x7F);
            } else {
                let _: u32 = lf_checker_rt::callee_cdecl!(3, u32, a2, a1);
                let q: f32 = lf_checker_rt::callee_thiscall!(4, f32,
                    mgr, a1, lf_checker_rt::relocated(G_SLOT_A), row(a1, a2),
                    state.wrapping_add(STATE_VEC), a4, a6, 0);
                wrf(a5, q);
            }
            scale_slot(a5);
            0
        } else {
            far_path(mgr, state, a1, a2, a4, a5, a6)
        };
        // Shared tail.
        let g_ref = lf_checker_rt::relocated(G_REF);
        let g_out = lf_checker_rt::relocated(G_OUT);
        let tx = sub(rdf(a4), rdf(g_ref));
        let ty = sub(rdf(a4.wrapping_add(4)), rdf(g_ref.wrapping_add(4)));
        let tz = sub(rdf(a4.wrapping_add(8)), rdf(g_ref.wrapping_add(8)));
        wrf(g_out.wrapping_add(4), ty);
        wrf(g_out, tx);
        let total = add(add(mul(ty, ty), mul(tx, tx)), mul(tz, tz));
        wrf(g_out.wrapping_add(8), tz);
        // Zero seed: the original reads a stack slot it never wrote.
        wrf(lf_checker_rt::relocated(G_SEED), 0.0);
        if total > 0.0 {
            let f: f32 = lf_checker_rt::callee_cdecl!(5, f32, total.to_bits());
            wrf(lf_checker_rt::relocated(G_LEN), div(ONE, f));
        } else {
            wr32(lf_checker_rt::relocated(G_LEN), 0);
        }
        result
    }
});

