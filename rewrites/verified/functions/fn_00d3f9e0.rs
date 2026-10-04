// original: 0x00d3f9e0 jump_forward_tagged_build (proposed)

/// Forward a tagged build request to the jump manager (tag 0xfe).
///
/// `this` is the task; `tag` (arg0) must equal `WANT_TAG` (0xfe) or 0 is
/// returned. The manager pointer from the global at `MGR_SLOT` is resolved
/// through the lookup callee; a null manager returns 0. Otherwise the
/// create callee runs (thiscall on the manager, five stack words): arg4,
/// arg2, arg3 carried as raw float bits, bit 0 of the flag byte at
/// `FLAGS` (+0x3d), and arg5. (Two pushed values, the manager pointer and
/// a scratch word, are overwritten by the float before the call; they are
/// dead.) Returns the create callee's answer. Arg1 is unread.
///
/// Original: 0x00d3f9e0 (thiscall, six stack words).
lf_checker_rt::export!(thiscall, rw_00d3f9e0(
    this: u32, tag: u32, _u: u32, p2: u32, f3: u32, a4: u32, a5: u32,
) -> u32 {
    unsafe {
        const MGR_SLOT: u32 = 0x0167_e2a0;
        const LOOKUP: u32 = 1;
        const CREATE: u32 = 2;
        const WANT_TAG: u32 = 0xfe;
        const FLAGS: u32 = 0x3d;

        if tag != WANT_TAG {
            return 0;
        }
        let mgr_src = lf_checker_rt::global::<u32>(MGR_SLOT).read_unaligned();
        let mgr: u32 = lf_checker_rt::callee_thiscall!(LOOKUP, u32, mgr_src);
        if mgr == 0 {
            return 0;
        }
        let bit = (((this + FLAGS) as *const u8).read() & 1) as u32;
        lf_checker_rt::callee_thiscall!(CREATE, u32, mgr, a4, p2, f3, bit, a5)
    }
});
