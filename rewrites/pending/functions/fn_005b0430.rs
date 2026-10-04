// original: 0x005B0430 MO_MARKET_NEW
/// Stage the market menu-overlay variant and publish its parameters.
///
/// Seeds three parameter slots (tags 7, 0 and 1), resolves a mode flag from
/// shared state (or `0xFF` when the flag source is clear), looks up a handle
/// and stages two pairs of float parameters through it, accumulates a biased
/// sum, then publishes one of two overlay name strings depending on a table
/// probe: the "new" variant only when the slot index is valid, its table
/// entry is live, the gate byte is clear and the entry probe accepts.
/// Returns nothing meaningful (the last callee's result is returned as-is).
lf_checker_rt::export!(cdecl, rb126_fn1() -> u32 {
    // Intercepted callees (ids match the contract's `callees` table).
    const C_SEED_TAG: u32 = 1; // seed slot, cdecl/1 (tag)
    const C_SEED_ZERO: u32 = 2; // seed slot, cdecl/1 (zero)
    const C_SEED_ONE: u32 = 3; // seed slot, cdecl/1 (one)
    const C_FLAG_FN: u32 = 4; // flag resolve, thiscall/0, al result
    const C_LOOKUP: u32 = 5; // handle lookup, cdecl/3 (out, kind, flag)
    const C_USE_HANDLE: u32 = 6; // handle consumer, cdecl/1
    const C_PAIR_A: u32 = 7; // float pair, cdecl/2 (out, 0x29)
    const C_PAIR_B: u32 = 8; // float pair, cdecl/2 (out, 0x11)
    const C_PAIR_C: u32 = 9; // float pair, cdecl/2 (out, 0x2a)
    const C_PAIR_D: u32 = 10; // float pair, cdecl/2 (out, 0x2b)
    const C_PAIR_E: u32 = 11; // float pair, cdecl/2 (out, 0x2d)
    const C_APPLY2: u32 = 12; // parameter apply, cdecl/4 (2, ptr, 0, 0)
    const C_APPLY3: u32 = 19; // parameter apply, cdecl/4 (3, ptr, 0, 0)
    const C_APPLY7: u32 = 20; // parameter apply, cdecl/4 (7, 0, ptr, 0)
    const C_STAGE_AB: u32 = 13; // stage pair A/B words, cdecl/2
    const C_STAGE_AC: u32 = 14; // stage pair A/C words, cdecl/2
    const C_STAGE_SUM: u32 = 15; // stage biased sum, cdecl/2 (1 live arg)
    const C_PROBE: u32 = 16; // entry probe, thiscall/0, al result
    const C_PUBLISH: u32 = 17; // overlay publish, thiscall/1 (name)
    const C_FINAL: u32 = 18; // final stage, cdecl/2

    // Globals (file VAs; resolved through the worker's image base).
    const G_FLAG_VA: u32 = 0x01161578; // flag source byte
    const G_INDEX_VA: u32 = 0x01036F14; // table slot index, -1 selects default
    const G_TABLE_VA: u32 = 0x011A8808; // table of entries, indexed by G_INDEX
    const G_GATE_VA: u32 = 0x017F5FC6; // gate byte, must be clear for "new"
    const F_BIAS_VA: u32 = 0x00FE8914; // bias added to the accumulated sum
    const S_NEW_VA: u32 = 0x00F8930C; // "MO_MARKET_NEW"
    const S_OLD_VA: u32 = 0x00F893E4; // "MO_MARKET_OL_PC"
    const CTX_VA: u32 = 0x0116BFF0; // publish context

    #[inline(always)]
    unsafe fn pair_word(pair: u32, word: u32) -> u32 {
        *((pair.wrapping_add(word.wrapping_mul(4))) as *const u32)
    }

    unsafe {
        lf_checker_rt::callee_cdecl!(C_SEED_TAG, u32, 7);
        lf_checker_rt::callee_cdecl!(C_SEED_ZERO, u32, 0);
        lf_checker_rt::callee_cdecl!(C_SEED_ONE, u32, 1);
        // Mode flag: the original pushes the flag byte as a full word whose
        // upper bytes are its own frame fill (zero under this contract), so a
        // plain zero-extended byte matches bit for bit.
        let flag: u32 = if *lf_checker_rt::global::<u8>(G_FLAG_VA) == 0 {
            0xFF
        } else {
            lf_checker_rt::callee_thiscall!(C_FLAG_FN, u32, lf_checker_rt::relocated(G_FLAG_VA))
                & 0xFF
        };
        // Scratch pair for out-pointer arguments (call addresses are skipped;
        // pointed-to words are scripted identically on both sides).
        let mut out = [0u32; 2];
        let out_ptr = out.as_mut_ptr() as u32;
        let handle = lf_checker_rt::callee_cdecl!(C_LOOKUP, u32, out_ptr, 0x44, flag);
        let handled = *(handle as *const u32);
        lf_checker_rt::callee_cdecl!(C_USE_HANDLE, u32, handled);
        let pa = lf_checker_rt::callee_cdecl!(C_PAIR_A, u32, out_ptr, 0x29);
        let a0 = pair_word(pa, 0);
        // The original also fetches pair A's second word into its frame, but
        // never reads it back; the fetch is mirrored and the value dropped.
        let _a1 = pair_word(pa, 1);
        lf_checker_rt::callee_cdecl!(C_APPLY2, u32, 2, out_ptr, 0, 0);
        let pb = lf_checker_rt::callee_cdecl!(C_PAIR_B, u32, out_ptr, 0x11);
        let b0 = pair_word(pb, 0);
        lf_checker_rt::callee_cdecl!(C_APPLY3, u32, 3, out_ptr, 0, 0);
        lf_checker_rt::callee_cdecl!(C_STAGE_AB, u32, a0, b0);
        lf_checker_rt::callee_cdecl!(C_PAIR_C, u32, out_ptr, 0x2A);
        lf_checker_rt::callee_cdecl!(C_APPLY7, u32, 7, 0, out_ptr, 0);
        // The callee wrote `out` through the pointer above; both words are
        // staged together.
        let c0 = out[0];
        let c1 = out[1];
        lf_checker_rt::callee_cdecl!(C_STAGE_AC, u32, c0, c1);
        lf_checker_rt::callee_cdecl!(C_PAIR_D, u32, out_ptr, 0x2B);
        let d0 = out[0];
        let d1 = out[1];
        let bias = *lf_checker_rt::global::<f32>(F_BIAS_VA);
        let sum = f32::from_bits(d0) + bias + f32::from_bits(d1);
        // The second word on the stack is the mode flag left in the frame slot.
        lf_checker_rt::callee_cdecl!(C_STAGE_SUM, u32, sum.to_bits(), flag);
        // Overlay selection: "new" only on the fully guarded path.
        let name = lf_checker_rt::relocated(S_OLD_VA);
        let index = *lf_checker_rt::global::<u32>(G_INDEX_VA);
        let mut name = name;
        if index != 0xFFFF_FFFF {
            let table = lf_checker_rt::relocated(G_TABLE_VA);
            let entry = *((table.wrapping_add(index.wrapping_mul(4))) as *const u32);
            if entry != 0 && *lf_checker_rt::global::<u8>(G_GATE_VA) == 0 {
                let ok = lf_checker_rt::callee_thiscall!(C_PROBE, u32, entry) & 0xFF;
                if ok != 0 {
                    name = lf_checker_rt::relocated(S_NEW_VA);
                }
            }
        }
        // The original pushes two constant words ahead of the name that the
        // callee never pops and nothing reads again; only the name is live.
        lf_checker_rt::callee_thiscall!(C_PUBLISH, u32, lf_checker_rt::relocated(CTX_VA), name);
        let pe = lf_checker_rt::callee_cdecl!(C_PAIR_E, u32, out_ptr, 0x2D);
        let e1 = pair_word(pe, 1);
        // The first final-stage argument is pair A's first word again, still
        // sitting in its frame slot.
        lf_checker_rt::callee_cdecl!(C_FINAL, u32, a0, e1)
    }
});
