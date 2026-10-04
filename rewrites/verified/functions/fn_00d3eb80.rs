// original: 0x00d3eb80 CTaskComplexJump::vf1

/// Build the jump subtask for a complex jump task (slot vf1).
///
/// `this` is the task. The manager pointer is read from the global at
/// `MGR_SLOT` and resolved through the lookup callee (thiscall, no stack
/// words); a null manager returns 0. Otherwise the create callee runs
/// (thiscall on the manager) with the weight word at `WEIGHT` (+0x3c) and
/// an options pointer: `this`+`OPT_DATA` (+0x20) when bit 3 of `OPT_FLAG`
/// (+0x5c) is set, else null. Returns the create callee's answer.
///
/// Original: 0x00d3eb80 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00d3eb80(this: u32) -> u32 {
    unsafe {
        const MGR_SLOT: u32 = 0x0167_e2a0;
        const LOOKUP: u32 = 1;
        const CREATE: u32 = 2;
        const WEIGHT: u32 = 0x3c;
        const OPT_FLAG: u32 = 0x5c;
        const OPT_BIT: u8 = 8;
        const OPT_DATA: u32 = 0x20;

        let mgr_src = lf_checker_rt::global::<u32>(MGR_SLOT).read_unaligned();
        let mgr: u32 = lf_checker_rt::callee_thiscall!(LOOKUP, u32, mgr_src);
        if mgr == 0 {
            return 0;
        }
        let weight = ((this + WEIGHT) as *const u16).read_unaligned() as u32;
        let opt = if ((this + OPT_FLAG) as *const u8).read() & OPT_BIT != 0 {
            this + OPT_DATA
        } else {
            0
        };
        lf_checker_rt::callee_thiscall!(CREATE, u32, mgr, weight, opt)
    }
});
