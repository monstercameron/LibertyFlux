// original: 0x00DC3FE0 ped_task_update_target (proposed)

/// Refresh the tracked-target vector for a ped task, gating on status flags.
///
/// `a0` points at status flags and `a1` at the task object: when bit 13 of
/// the flags word is clear the task word's bit 13 must also be clear,
/// otherwise the function returns 2 at once. Callee 1 (thiscall on the
/// manager at `MANAGER`, the global at `G_STATE` and `a3`) must then answer
/// nonzero, else it returns 2 again.
///
/// The switch at `G_MODE` selects the query shape: nonzero also reads the
/// enable byte at `G_ENABLE` into flag A (1 only when the mode is on and
/// the byte is zero) and derives flag B from the inverted low bit of the
/// task word, while zero forces flag A to 0. Both shapes call callee 2
/// (cdecl, `a2` then `a1`) and then callee 3 (thiscall on the manager with
/// eleven stack words: the task, four fixed globals, a pointer built as
/// `[a2+0x64] + ([a1+4] & 0x1FFFF) * 8`, `a4`/`a5`/`a6`, and the two flags),
/// which fills the three reference floats at `G_REF`. The zero-mode shape
/// passes a literal 0 for flag B and a constant 1 for the status byte,
/// while the nonzero-mode shape passes both flags and keeps callee 3's
/// answer byte as the status.
///
/// Finally, unless `[a6]` is -1 (which zeroes the output vector instead),
/// the three floats at `a4` are differenced against the reference, the
/// differences land at `G_OUT`, their squared length lands (square-rooted)
/// at `G_LEN`, and when the x/y part is strictly positive callee 4 (cdecl,
/// the x/y bits) scales all three outputs. The entry stack slot the
/// original loads its seed from is never written, so under a zero stack
/// fill the seed at `G_SEED` is always +0.0. Returns 1 when the status byte
/// is zero, else 0 (2 for the two early exits).
///
/// Original: 0x00DC3FE0 (cdecl, seven stack words).
lf_checker_rt::export!(cdecl, rw_00DC3FE0(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    unsafe {
        const FLAG_BIT: u32 = 13;
        const MANAGER: u32 = 0x016C6830;
        const G_STATE: u32 = 0x016C7448;
        const G_MODE: u32 = 0x0104827C;
        const G_ENABLE: u32 = 0x016B6B23;
        const G_SLOT_A: u32 = 0x017A1DD0;
        const G_REF: u32 = 0x017A22E0;
        const G_SLOT_C: u32 = 0x017A2310;
        const G_SLOT_D: u32 = 0x017A2320;
        const G_OUT: u32 = 0x017A2300;
        const G_SEED: u32 = 0x017A230C;
        const G_LEN: u32 = 0x017A3390;
        const OFF_KEY: u32 = 0x64;
        const KEY_MASK: u32 = 0x1FFFF;
        const ROW_STRIDE: u32 = 8;

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

        if (rd32(a0) >> FLAG_BIT) & 1 == 0 && (rd32(a1) >> FLAG_BIT) & 1 != 0 {
            return 2;
        }
        let mgr = lf_checker_rt::relocated(MANAGER);
        let gate: u32 = lf_checker_rt::callee_thiscall!(1, u32, mgr, rd32(lf_checker_rt::relocated(G_STATE)), a3);
        if gate as u8 == 0 {
            return 2;
        }
        let mode = rd32(lf_checker_rt::relocated(G_MODE));
        let flag_a: u32 = if mode != 0 && rd8(lf_checker_rt::relocated(G_ENABLE)) == 0 { 1 } else { 0 };
        // Row pointer shared by both query shapes.
        let row = rd32(a2.wrapping_add(OFF_KEY))
            .wrapping_add((rd32(a1.wrapping_add(4)) & KEY_MASK).wrapping_mul(ROW_STRIDE));
        let g_ref = lf_checker_rt::relocated(G_REF);
        let status: u8 = if mode != 0 {
            let flag_b = (!rd8(a1)) & 1;
            let _: u32 = lf_checker_rt::callee_cdecl!(2, u32, a2, a1);
            lf_checker_rt::callee_thiscall!(3, u32, mgr, a1,
                lf_checker_rt::relocated(G_SLOT_A), row, g_ref,
                lf_checker_rt::relocated(G_SLOT_C), lf_checker_rt::relocated(G_SLOT_D),
                a4, a5, a6, flag_a, flag_b as u32) as u8
        } else {
            let _: u32 = lf_checker_rt::callee_cdecl!(2, u32, a2, a1);
            let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, mgr, a1,
                lf_checker_rt::relocated(G_SLOT_A), row, g_ref,
                lf_checker_rt::relocated(G_SLOT_C), lf_checker_rt::relocated(G_SLOT_D),
                a4, a5, a6, flag_a, 0);
            1
        };
        // The original loads this from a stack slot it never wrote; the
        // contract fills unread stack with zero, so the seed is +0.0.
        let seed = 0.0f32;
        let g_out = lf_checker_rt::relocated(G_OUT);
        if rd32(a6) == 0xFFFF_FFFF {
            wr32(g_out, 0);
            wr32(g_out.wrapping_add(4), 0);
            wr32(g_out.wrapping_add(8), 0);
            wrf(lf_checker_rt::relocated(G_SEED), seed);
            wr32(lf_checker_rt::relocated(G_LEN), 0);
        } else {
            let dx = sub(rdf(a4), rdf(g_ref));
            let dy = sub(rdf(a4.wrapping_add(4)), rdf(g_ref.wrapping_add(4)));
            let dz = sub(rdf(a4.wrapping_add(8)), rdf(g_ref.wrapping_add(8)));
            wrf(lf_checker_rt::relocated(G_SEED), seed);
            wrf(g_out, dx);
            wrf(g_out.wrapping_add(4), dy);
            let planar = add(mul(dx, dx), mul(dy, dy));
            wr32(g_out.wrapping_add(8), 0);
            wrf(lf_checker_rt::relocated(G_LEN), add(mul(dz, dz), planar).sqrt());
            if planar > 0.0 {
                let f: f32 = lf_checker_rt::callee_cdecl!(4, f32, planar.to_bits());
                wrf(g_out, mul(rdf(g_out), f));
                wrf(g_out.wrapping_add(4), mul(rdf(g_out.wrapping_add(4)), f));
                wrf(g_out.wrapping_add(8), mul(f, rdf(g_out.wrapping_add(8))));
            }
        }
        if status == 0 { 1 } else { 0 }
    }
});

