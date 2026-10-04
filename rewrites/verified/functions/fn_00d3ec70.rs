// original: 0x00d3ec70 CTaskSimpleJumpLaunch::vf1

/// Build the control subtask for a jump-launch task (slot vf1).
///
/// `this` is the task. The manager pointer is read from the global at
/// `MGR_SLOT` and resolved through the lookup callee (thiscall, no stack
/// words); a null manager returns 0. Otherwise the create callee runs
/// (thiscall on the manager) with the id word at `ID` (+0x90) and a
/// trailing 0. Returns the create callee's answer.
///
/// Original: 0x00d3ec70 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00d3ec70(this: u32) -> u32 {
    unsafe {
        const MGR_SLOT: u32 = 0x0167_e2a0;
        const LOOKUP: u32 = 1;
        const CREATE: u32 = 2;
        const ID: u32 = 0x90;

        let mgr_src = lf_checker_rt::global::<u32>(MGR_SLOT).read_unaligned();
        let mgr: u32 = lf_checker_rt::callee_thiscall!(LOOKUP, u32, mgr_src);
        if mgr == 0 {
            return 0;
        }
        let id = ((this + ID) as *const u16).read_unaligned() as u32;
        lf_checker_rt::callee_thiscall!(CREATE, u32, mgr, id, 0)
    }
});
