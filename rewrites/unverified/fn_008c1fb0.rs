// original: 0x008C1FB0 game_state_is_active

/// Decide whether the game state counts as active.
///
/// Returns 0 only on the fully quiet path: the state word `STATE_A` is not
/// 1, the generation words `GEN_B` and `GEN_C` agree, and the phase word
/// `PHASE` is not 0x12. On every other path the answer is whether the ready
/// word `READY` is non-zero. Only the low byte of the return is set; the
/// upper 24 bits keep whatever the caller left in EAX. Leaf, cdecl, no
/// arguments.
lf_checker_rt::export!(cdecl, rw_008C1FB0() -> u32 {
    unsafe {
        const STATE_A: u32 = 0x011F_7060;
        const GEN_B: u32 = 0x0120_88B4;
        const GEN_C: u32 = 0x00F1_C040;
        const PHASE: u32 = 0x0103_7720;
        const READY: u32 = 0x011F_66A0;
        const ACTIVE_STATE: u32 = 1;
        const LIVE_PHASE: u32 = 0x12;
        let a = (lf_checker_rt::relocated(STATE_A) as *const u32).read_unaligned();
        if a != ACTIVE_STATE {
            let b = (lf_checker_rt::relocated(GEN_B) as *const u32).read_unaligned();
            let c = (lf_checker_rt::relocated(GEN_C) as *const u32).read_unaligned();
            if b == c {
                let d = (lf_checker_rt::relocated(PHASE) as *const u32).read_unaligned();
                if d != LIVE_PHASE {
                    return 0;
                }
            }
        }
        let e = (lf_checker_rt::relocated(READY) as *const u32).read_unaligned();
        u32::from(e != 0)
    }
});
