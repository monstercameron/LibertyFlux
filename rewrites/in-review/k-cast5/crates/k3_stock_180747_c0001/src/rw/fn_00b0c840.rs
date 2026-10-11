// original: 0x00B0C840 net_event_dispatch (proposed)

/// Dispatch a network event id to its handler.
///
/// Compares the id against five known ids in order and runs the first
/// match; an unknown id returns 0. Every path returns through the low
/// byte (`(an instruction of the original)`), so only `al` is significant.
///
/// Paths: `ID0` converts the word at `a2+W_FLOAT` to float and forwards
/// it with `a1`; `ID1` and `ID3` only notify with their tag; `ID2` polls
/// the sub-object twice (skipping the second poll when the first answers
/// 0) and forwards the result; `ID4` walks the `a2+O_NEPH` chain (bailing
/// out with 1 when the entry or the chain is missing, when the gate byte
/// is clear, when the flag bit is clear, or when the masked mode equals
/// `MODE_SKIP`), calls back through the `a1` vtable (slot `VT_F32`, whose
/// x87 single result is subtracted from the word at `a2+W_NUM`), converts
/// the difference to a 64-bit integer with chop rounding exactly like the
/// original's `fistp` (out-of-range and NaN yield `i64::MIN`), and finishes
/// through vtable slot `VT_DONE`.
///
/// All callees are intercepted and scripted, including the two that live
/// in the encrypted region (their call sites are patched, so they never
/// execute). The vtable callees are reached through the planted object
/// exactly like the original reaches them.
///
/// Original: 0x00B0C840 (cdecl, three stack words, returns `al`).
lf_checker_rt::export!(cdecl, rw_00B0C840(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const ID0: u32 = 0x1632BF0;
        const ID1: u32 = 0x161564C;
        const ID2: u32 = 0x1615640;
        const ID3: u32 = 0x1632BE4;
        const ID4: u32 = 0x12FA3E0;
        const SINK: u32 = 0x1176888;
        const GATE: u32 = 0x10400BC;
        const W_FLOAT: u32 = 0x558;
        const W_NUM: u32 = 0x556;
        const W_CLEAR: u32 = 0x568;
        const O_SUB: u32 = 0x228;
        const O_NEPH: u32 = 0x598;
        const N_FLAG: u32 = 0x26C;
        const N_NEXT: u32 = 0xB30;
        const N_MODE: u32 = 0x28;
        const MODE_MASK: u32 = 0x7C00;
        const MODE_SKIP: u32 = 0x0C00;
        const VT_F32: u32 = 0xFC;
        const VT_DONE: u32 = 0xF4;
        const ONE_F: u32 = 0x3F800000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
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
        unsafe fn g32(file_va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(file_va)) }
        }
        #[inline(always)]
        unsafe fn g8(file_va: u32) -> u8 {
            unsafe { rd8(lf_checker_rt::relocated(file_va)) }
        }

        /// The original's `fistp qword` under chop rounding: truncate toward
        /// zero when representable, else the indefinite value (this is also
        /// what NaN and infinities produce). Rust's `as` truncates but
        /// saturates, so the range is checked first.
        #[inline(always)]
        fn fistp_chop(f: f32) -> u64 {
            const LO: f32 = -9223372036854775808.0; // -2^63, exact
            const HI: f32 = 9223372036854775808.0; // +2^63, exact
            if f.is_finite() && f >= LO && f < HI {
                (f as i64) as u64
            } else {
                0x8000000000000000
            }
        }

        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        let sink = lf_checker_rt::relocated(SINK);
        if a0 == g32(ID0) {
            let bits = (rd16(a2 + W_FLOAT) as f32).to_bits();
            lf_checker_rt::callee_thiscall!(1, u32, a1, bits);
            lf_checker_rt::callee_thiscall!(2, u32, sink, 3u32);
            return 1;
        }
        if a0 == g32(ID1) {
            lf_checker_rt::callee_thiscall!(2, u32, sink, 5u32);
            return 1;
        }
        if a0 == g32(ID2) {
            let p = rd32(a1 + O_SUB);
            let ecx = if p == 0 { 0 } else { p.wrapping_add(0x70) };
            let r: u32 = lf_checker_rt::callee_thiscall!(3, u32, ecx);
            let v = r.wrapping_sub(1);
            let v = if (v as i32) < 0 {
                0
            } else {
                let p2 = rd32(a1 + O_SUB);
                let ecx2 = if p2 == 0 { 0 } else { p2.wrapping_add(0x70) };
                let r2: u32 = lf_checker_rt::callee_thiscall!(3, u32, ecx2);
                r2.wrapping_sub(1)
            };
            lf_checker_rt::callee_thiscall!(4, u32, a1, v, 0u32);
            lf_checker_rt::callee_cdecl!(5, u32, 0x1C0u32, ONE_F);
            lf_checker_rt::callee_thiscall!(2, u32, sink, 2u32);
            return 1;
        }
        if a0 == g32(ID3) {
            lf_checker_rt::callee_thiscall!(2, u32, sink, 4u32);
            return 1;
        }
        if a0 != g32(ID4) {
            return 0;
        }
        if a2 == 0 {
            return 1;
        }
        if g8(GATE) != 0 {
            let n1 = rd32(a2 + O_NEPH);
            if rd8(n1 + N_FLAG) & 4 != 0 && rd32(n1 + N_NEXT) != 0 {
                let t: u32 = lf_checker_rt::callee_cdecl!(6, u32,);
                if (t as u8) != 0 {
                    let n2 = rd32(n1 + N_NEXT);
                    if rd32(n2 + N_MODE) & MODE_MASK == MODE_SKIP {
                        return 1;
                    }
                }
                lf_checker_rt::callee_cdecl!(7, u32, rd32(n1 + N_NEXT));
            }
        }
        let num = rd16(a2 + W_NUM) as f32;
        let vt = rd32(a1);
        let f32slot: extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute(rd32(vt + VT_F32) as usize);
        let got = f32slot(a1);
        let diff = sub(num, got);
        let q = fistp_chop(diff);
        lf_checker_rt::callee_cdecl!(9, u32, (q & 0xFFFF_FFFF) as u32);
        lf_checker_rt::callee_cdecl!(5, u32, 0x16Au32, ONE_F);
        let num2 = rd16(a2 + W_NUM) as f32;
        let doneslot: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(vt + VT_DONE) as usize);
        doneslot(a1, num2.to_bits(), 0);
        wr32(a2 + W_CLEAR, 0);
        lf_checker_rt::callee_thiscall!(2, u32, sink, 7u32);
        1
    }
});
