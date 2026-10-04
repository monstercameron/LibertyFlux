// original: 0x005B23A0 menu_fade_blend_tick
/// Blend one tick of a menu fade level and sink it per row.
///
/// Seeds a base level from two parameter pairs, biases it with a small
/// constant (plus an optional second bias), then walks a row count derived
/// from shared state (13 or 16 rows). Each row converts a parameter to an
/// integer selector, clamps a second parameter into `[0, bound]` where the
/// bound comes from the shared flag object (or the selector when the flag
/// is clear), folds the clamped byte into the row accumulator, and on every
/// other row resolves four selector words through the installed indirect
/// hook and sinks the accumulator plus a scaled level into the row sink.
/// Returns nothing meaningful.
lf_checker_rt::export!(cdecl, rb126_fn2() -> u32 {
    // Intercepted callees (ids match the contract's `callees` table).
    const C_SEED_A: u32 = 1; // float pair, cdecl/2 (out, 0x67)
    const C_SEED_B: u32 = 2; // float pair, cdecl/2 (out, 0x67)
    const C_TAG42: u32 = 3; // tag consume, cdecl/2 (out, 0x42)
    const C_BASE: u32 = 4; // base level, cdecl/1 (out)
    const C_BIAS2: u32 = 5; // second bias, cdecl/1 (out)
    const C_GAIN: u32 = 6; // gain pair, cdecl/2 (out, 0x11)
    const C_APPLY2: u32 = 7; // parameter apply, cdecl/4 (2, ptr, 0, 0)
    const C_APPLY3: u32 = 8; // parameter apply, cdecl/4 (3, ptr, 0, 0)
    const C_SELECT: u32 = 9; // row selector, cdecl/2 (out, 0x36)
    const C_FLAG_FN: u32 = 10; // flag resolve, thiscall/0, al result
    const C_CLAMPIN: u32 = 11; // clamp input, cdecl/2 (out, 0x36)
    // 12 is the planted indirect hook, called through the global below.
    const C_SINK: u32 = 13; // row sink, cdecl/2 (accum, scaled)
    const C_ACCUM: u32 = 14; // accumulator addend, cdecl/2 (out, 0x67)

    // Globals (file VAs; resolved through the worker's image base).
    const G_BIAS_FLAG_VA: u32 = 0x01160C32; // second-bias gate byte
    const G_COUNT_VA: u32 = 0x01160C40; // row-count select (15 -> 12 else 15)
    const G_FLAG_VA: u32 = 0x01161548; // flag source byte
    const G_EXPECT_VA: u32 = 0x0110DD14; // hook-result discriminator
    const G_SELA_VA: u32 = 0x0105C880; // selector word A
    const G_SELB_VA: u32 = 0x0105C87C; // selector word B
    const G_SELC_VA: u32 = 0x0105C884; // selector word C
    const G_SELD_VA: u32 = 0x0105C888; // selector word D
    const G_HOOK_VA: u32 = 0x00E731AC; // installed indirect hook
    const F_STEP_VA: u32 = 0x00FE86EC; // small level step constant

    /// `cvttss2si` with exact x86 semantics: truncation toward zero, and the
    /// integer-indefinite value for NaN and out-of-range inputs (Rust's `as`
    /// saturates instead, which differs exactly there).
    #[inline(always)]
    fn cvt_trunc(f: f32) -> u32 {
        if f.is_nan() || f >= 2147483648.0 || f < -2147483648.0 {
            0x8000_0000
        } else {
            f as i32 as u32
        }
    }

    /// The original's two-`comiss` clamp, including unordered (NaN) inputs:
    /// a negative bound selects zero, otherwise the value is limited above
    /// by the bound, with a NaN value collapsing onto the bound.
    #[inline(always)]
    fn clamp_duel(a: f32, b: f32) -> f32 {
        if b < 0.0 {
            0.0
        } else if !(a <= b) {
            b
        } else {
            a
        }
    }

    #[inline(always)]
    unsafe fn pair_word(pair: u32, word: u32) -> u32 {
        *((pair.wrapping_add(word.wrapping_mul(4))) as *const u32)
    }

    unsafe {
        let mut out = [0u32; 2];
        let out_ptr = out.as_mut_ptr() as u32;
        let pa = lf_checker_rt::callee_cdecl!(C_SEED_A, u32, out_ptr, 0x67);
        let const_add = f32::from_bits(pair_word(pa, 0));
        let pb = lf_checker_rt::callee_cdecl!(C_SEED_B, u32, out_ptr, 0x67);
        let mut accum = f32::from_bits(pair_word(pb, 1));
        lf_checker_rt::callee_cdecl!(C_TAG42, u32, out_ptr, 0x42);
        let pc = lf_checker_rt::callee_cdecl!(C_BASE, u32, out_ptr);
        let step = *lf_checker_rt::global::<f32>(F_STEP_VA);
        let mut level = f32::from_bits(pair_word(pc, 0)) - step;
        if *lf_checker_rt::global::<u8>(G_BIAS_FLAG_VA) == 0 {
            let pd = lf_checker_rt::callee_cdecl!(C_BIAS2, u32, out_ptr);
            level += f32::from_bits(pair_word(pd, 0));
        }
        let pe = lf_checker_rt::callee_cdecl!(C_GAIN, u32, out_ptr, 0x11);
        let gain = f32::from_bits(pair_word(pe, 0)) + step;
        lf_checker_rt::callee_cdecl!(C_APPLY2, u32, 2, out_ptr, 0, 0);
        lf_checker_rt::callee_cdecl!(C_APPLY3, u32, 3, out_ptr, 0, 0);
        let count = *lf_checker_rt::global::<u32>(G_COUNT_VA);
        let rows = (if count == 15 { 12 } else { 15 }) + 1;
        // The accumulator word starts as the frame slot's own fill (zero
        // under this contract); the toggle byte and its slot start clear.
        let mut acc_word = 0u32;
        let mut toggle = false;
        let mut toggle_slot = 0u8;
        let mut hook = *(lf_checker_rt::global::<u32>(G_HOOK_VA) as *const u32);
        let expect = *lf_checker_rt::global::<u32>(G_EXPECT_VA);
        let mut row = 0u32;
        while row < rows {
            let ps = lf_checker_rt::callee_cdecl!(C_SELECT, u32, out_ptr, 0x36);
            let sel = cvt_trunc(f32::from_bits(pair_word(ps, 0)));
            let bound: u32 = if *lf_checker_rt::global::<u8>(G_FLAG_VA) == 0 {
                sel & 0xFF
            } else {
                lf_checker_rt::callee_thiscall!(C_FLAG_FN, u32, lf_checker_rt::relocated(G_FLAG_VA))
                    & 0xFF
            };
            let pi = lf_checker_rt::callee_cdecl!(C_CLAMPIN, u32, out_ptr, 0x36);
            let clamped = clamp_duel(
                f32::from_bits(pair_word(pi, 0)),
                (bound as i32) as f32,
            );
            let folded = cvt_trunc(clamped) & 0xFF;
            acc_word = (acc_word & 0x00FF_FFFF) | (folded << 24);
            if toggle {
                let pick = |hit: bool, lo: u32, hi: u32| -> u32 {
                    if hit {
                        hi
                    } else {
                        lo
                    }
                };
                let sa = *lf_checker_rt::global::<u32>(G_SELA_VA);
                let sb = *lf_checker_rt::global::<u32>(G_SELB_VA);
                let sc = *lf_checker_rt::global::<u32>(G_SELC_VA);
                let sd = *lf_checker_rt::global::<u32>(G_SELD_VA);
                let f: extern "cdecl" fn() -> u32 =
                    core::mem::transmute(hook as usize);
                let w0 = pick(f() == expect, sa, sb);
                let w1 = pick(f() == expect, sc, sd);
                let w2 = pick(f() == expect, sa, sb);
                let w3 = pick(f() == expect, sc, sd);
                // The sink's first argument points at five consecutive words
                // (compared via snapshot): the scaled level plus three
                // combined chains the original stores beside it, and the
                // frame fill word below them.
                let scaled = (w3 as i32) as f32 * level;
                let chain_esi = (w2 as i32) as f32 * accum;
                let tmp = accum + const_add;
                let chain_edi = (w1 as i32) as f32 * gain;
                let chain_ebx = tmp * (w0 as i32) as f32;
                let mut sink_acc = acc_word;
                let mut sink_block = [
                    scaled.to_bits(),
                    chain_ebx.to_bits(),
                    chain_edi.to_bits(),
                    chain_esi.to_bits(),
                    0u32,
                ];
                lf_checker_rt::callee_cdecl!(
                    C_SINK,
                    u32,
                    sink_block.as_mut_ptr() as u32,
                    &mut sink_acc as *mut u32 as u32
                );
                // Re-read of the toggle slot (provably a no-op: the slot
                // always holds this iteration's toggle value) and hook
                // reload, both mirrored for shape fidelity.
                toggle = toggle_slot != 0;
                hook = *(lf_checker_rt::global::<u32>(G_HOOK_VA) as *const u32);
            }
            let pj = lf_checker_rt::callee_cdecl!(C_ACCUM, u32, out_ptr, 0x67);
            accum = f32::from_bits(pair_word(pj, 0)) + accum;
            toggle = !toggle;
            toggle_slot = toggle as u8;
            row += 1;
        }
        0
    }
});
