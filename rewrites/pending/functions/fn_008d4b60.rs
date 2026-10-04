// original: 0x008D4B60 state_refresh_helper
/// State refresh helper.
///
/// Returns immediately when the skip flag is set (returning the incoming
/// EAX untouched — the contract pins it; all known callers ignore the
/// result). Otherwise it derives a size pair: either by polling a helper
/// twice and selecting from two global pairs on the low byte of each
/// answer, or — when the context pointer is set — by scaling two integers
/// with two floats and truncating, exactly like `cvttss2si` (out-of-range
/// and NaN yield `0x80000000`). When the pair and the version word match
/// the cached triple and `flag` is zero it returns the stashed high half
/// combined with the version word; otherwise it stores the new triple,
/// runs a prepare call and an apply call fed with two quotients, and
/// returns the apply call's answer. Two scratch stores of the original
/// (the second quotient and two constants) are dead — written below the
/// frame pointer and never read — and are omitted.
lf_checker_rt::export!(cdecl, rb92_fn6(flag: u32) -> u32 {
    // Intercepted callees (ids match the contract's `callees` table).
    const CAL_POLL: u32 = 1; // polled helper (cdecl/0)
    const CAL_PREP: u32 = 2; // prepare (thiscall/0, scratch word)
    const CAL_APPLY: u32 = 3; // apply (thiscall/2)

    const G_SKIP: u32 = 0x0117_36F4; // nonzero = return immediately
    const G_CTX: u32 = 0x017F_583C; // context pointer, or null for poll mode
    const G_SZ0: u32 = 0x0105_C884; // size pair, first poll low byte clear
    const G_SZ1: u32 = 0x0105_C888; // size pair, first poll low byte set
    const G_SZ2: u32 = 0x0105_C880; // size pair, second poll low byte clear
    const G_SZ3: u32 = 0x0105_C87C; // size pair, second poll low byte set
    const G_VER: u32 = 0x011A_8908; // version word source (low 16 bits)
    const G_TVER: u32 = 0x0117_371C; // cached version (low 16 bits)
    const G_TS0: u32 = 0x0117_3720; // cached size pair
    const G_TS1: u32 = 0x0117_3724;
    const G_C1: u32 = 0x00FE_8A24; // first quotient numerator (pristine)
    const G_C2: u32 = 0x00FE_8DB0; // second quotient numerator (pristine)
    const G_THIS: u32 = 0x0117_36C4; // apply call receiver
    const G_MODE: u32 = 0x0117_36E4; // apply call first argument
    // Pinned incoming EAX, returned on the skip path (see doc comment).
    const IN_EAX: u32 = 0xA5A5_A5A5;

    #[inline(always)]
    fn g32(file_va: u32) -> u32 {
        unsafe { *(lf_checker_rt::global::<u32>(file_va) as *const u32) }
    }

    #[inline(always)]
    fn set32(file_va: u32, v: u32) {
        unsafe {
            *(lf_checker_rt::global::<u32>(file_va) as *mut u32) = v;
        }
    }

    /// `cvttss2si` semantics: truncate toward zero; NaN, infinities and
    /// out-of-range values yield `0x80000000` (Rust's `as` saturates
    /// instead, so the range check is explicit).
    #[inline(always)]
    fn cvt(x: f32) -> i32 {
        if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
            0x80000000u32 as i32
        } else {
            x as i32
        }
    }

    unsafe {
        let skip = *(lf_checker_rt::global::<u8>(G_SKIP) as *const u8);
        if skip != 0 {
            return IN_EAX;
        }
        let ctx = g32(G_CTX);
        // Third element feeds the early-return value's high half: the
        // second poll answer in poll mode, the context pointer otherwise.
        let (esi, edi, retbase): (u32, u32, u32) = if ctx == 0 {
            let a1 = lf_checker_rt::callee_cdecl!(CAL_POLL, u32,);
            let s = if (a1 & 0xFF) != 0 { g32(G_SZ1) } else { g32(G_SZ0) };
            let a2 = lf_checker_rt::callee_cdecl!(CAL_POLL, u32,);
            let d = if (a2 & 0xFF) != 0 { g32(G_SZ3) } else { g32(G_SZ2) };
            (s, d, a2)
        } else {
            let i0 = *(ctx.wrapping_add(0x2B0) as *const i32);
            let f0 = *(ctx.wrapping_add(0x288) as *const f32);
            let i1 = *(ctx.wrapping_add(0x2B4) as *const i32);
            let f1 = *(ctx.wrapping_add(0x28C) as *const f32);
            (
                cvt((i0 as f32) * f0) as u32,
                cvt((i1 as f32) * f1) as u32,
                ctx,
            )
        };
        let ver = g32(G_VER) & 0xFFFF;
        if ver == (g32(G_TVER) & 0xFFFF) && esi == g32(G_TS0) && edi == g32(G_TS1) && (flag & 0xFF) == 0 {
            return (retbase & 0xFFFF0000) | ver;
        }
        set32(G_TVER, (g32(G_TVER) & 0xFFFF0000) | ver);
        set32(G_TS0, esi);
        set32(G_TS1, edi);
        // The original passes an uninitialized scratch word here (defined
        // to zero by the contract's stack fill); an explicit zero is the
        // same observable value.
        let zero = 0u32;
        lf_checker_rt::callee_thiscall!(CAL_PREP, u32, &zero as *const u32 as u32);
        let c1 = *(lf_checker_rt::global::<f32>(G_C1) as *const f32);
        let q1 = c1 / ((esi as i32) as f32);
        let mut slot = q1;
        let this = g32(G_THIS);
        let mode = g32(G_MODE);
        lf_checker_rt::callee_thiscall!(CAL_APPLY, u32, this, mode, &mut slot as *mut f32 as u32)
    }
});
