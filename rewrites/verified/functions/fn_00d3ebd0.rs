// original: 0x00d3ebd0 CTaskSimpleJumpInAir::vf1

/// Build the control subtask for a jump-in-air task (slot vf1).
///
/// `this` is the task. The manager pointer is read from the global at
/// `MGR_SLOT` and resolved through the lookup callee (thiscall, no stack
/// words); a null manager returns 0. Otherwise the create callee runs
/// (thiscall on the manager, six stack words): the dwords at `P0` (+0x18)
/// and `P1` (+0x1c), bit 1 of the flag byte at `FLAGS` (+0x70) converted
/// to float (0.0/1.0, matching cvtdq2ps), bit 2, whether bit 3 is set
/// (the options pointer `this`+0x60 is formed and null-tested, then
/// overwritten by the float before the call), and a trailing 0. Returns
/// the create callee's answer.
///
/// Original: 0x00d3ebd0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00d3ebd0(this: u32) -> u32 {
    unsafe {
        const MGR_SLOT: u32 = 0x0167_e2a0;
        const LOOKUP: u32 = 1;
        const CREATE: u32 = 2;
        const P0: u32 = 0x18;
        const P1: u32 = 0x1c;
        const FLAGS: u32 = 0x70;

        let mgr_src = lf_checker_rt::global::<u32>(MGR_SLOT).read_unaligned();
        let mgr: u32 = lf_checker_rt::callee_thiscall!(LOOKUP, u32, mgr_src);
        if mgr == 0 {
            return 0;
        }
        let flags = ((this + FLAGS) as *const u8).read();
        let has_opt = (flags & 8 != 0) as u32;
        let bit2 = ((flags >> 2) & 1) as u32;
        let bit1 = (flags >> 1) & 1;
        let as_float = (bit1 as f32).to_bits();
        lf_checker_rt::callee_thiscall!(
            CREATE, u32, mgr,
            ((this + P0) as *const u32).read_unaligned(),
            ((this + P1) as *const u32).read_unaligned(),
            as_float, bit2, has_opt, 0
        )
    }
});
