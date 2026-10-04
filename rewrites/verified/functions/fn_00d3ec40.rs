// original: 0x00d3ec40 CTaskSimpleJumpLand::vf1

/// Build the control subtask for a jump-land task (slot vf1).
///
/// `this` is the task. The manager pointer is read from the global at
/// `MGR_SLOT` and resolved through the lookup callee (thiscall, no stack
/// words); a null manager returns 0. Otherwise the create callee runs
/// (thiscall on the manager) with the dwords at `P0` (+0x18) and `P1`
/// (+0x1c). Returns the create callee's answer.
///
/// Original: 0x00d3ec40 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00d3ec40(this: u32) -> u32 {
    unsafe {
        const MGR_SLOT: u32 = 0x0167_e2a0;
        const LOOKUP: u32 = 1;
        const CREATE: u32 = 2;
        const P0: u32 = 0x18;
        const P1: u32 = 0x1c;

        let mgr_src = lf_checker_rt::global::<u32>(MGR_SLOT).read_unaligned();
        let mgr: u32 = lf_checker_rt::callee_thiscall!(LOOKUP, u32, mgr_src);
        if mgr == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(
            CREATE, u32, mgr,
            ((this + P0) as *const u32).read_unaligned(),
            ((this + P1) as *const u32).read_unaligned()
        )
    }
});
