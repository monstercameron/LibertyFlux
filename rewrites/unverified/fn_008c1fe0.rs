// original: 0x008C1FE0 game_state_is_stable

/// Decide whether the game state counts as stable.
///
/// First half: when the phase word `PHASE` is zero, the generation words
/// disagree, or the phase is the loading phase 0x12, the state goes to the
/// tail check directly. Phases 2 and 7 additionally require the guard byte
/// `GUARD` to be clear, otherwise the active predicate is called and must
/// agree while the hold byte `HOLD` stays clear. Tail check: the sub-phase
/// word `SUB` must be none of 5, 0xD, 6 or 0xE. Returns 1 for stable, 0
/// otherwise, in the low byte only; the upper 24 bits of EAX keep whatever
/// the caller left there. Leaf apart from the predicate call, cdecl, no
/// arguments.
lf_checker_rt::export!(cdecl, rw_008C1FE0() -> u32 {
    unsafe {
        const PHASE: u32 = 0x0103_7720;
        const GEN_B: u32 = 0x0120_88B4;
        const GEN_C: u32 = 0x00F1_C040;
        const LOADING_PHASE: u32 = 0x12;
        const GUARDED_A: u32 = 2;
        const GUARDED_B: u32 = 7;
        const GUARD: u32 = 0x011F_701E;
        const HOLD: u32 = 0x0103_76E8;
        const SUB: u32 = 0x0103_7724;
        let phase = (lf_checker_rt::relocated(PHASE) as *const u32).read_unaligned();
        if phase != 0 {
            let b = (lf_checker_rt::relocated(GEN_B) as *const u32).read_unaligned();
            let c = (lf_checker_rt::relocated(GEN_C) as *const u32).read_unaligned();
            if b == c && phase != LOADING_PHASE {
                let guarded_out = (phase == GUARDED_A || phase == GUARDED_B)
                    && (lf_checker_rt::relocated(GUARD) as *const u8).read_unaligned() != 0;
                if !guarded_out {
                    if lf_checker_rt::callee_cdecl!(1, u32,) as u8 == 0 {
                        return 0;
                    }
                    if (lf_checker_rt::relocated(HOLD) as *const u8).read_unaligned() != 0 {
                        return 0;
                    }
                }
            }
        }
        let sub = (lf_checker_rt::relocated(SUB) as *const u32).read_unaligned();
        u32::from(sub != 5 && sub != 0xD && sub != 6 && sub != 0xE)
    }
});
