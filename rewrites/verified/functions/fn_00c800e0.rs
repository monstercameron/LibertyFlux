// original: 0x00c800e0 CTaskComplexWanderingScenario::vf7

/// Attach the wandering slot: resolve the owner, then link record field `+0x14`.
///
/// The owner comes from `c1`, called thiscall-style with the shared handle
/// kept in a global; a null owner yields 0. Otherwise `c2(owner, 1, [this+0x14])`
/// runs (thiscall, two stack words) and its result is returned. The one stack
/// word is popped but never read. EAX is defined on every path.
///
/// Original: thiscall, one stack word (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_00c800e0(this: u32, _u: u32) -> u32 {
    unsafe {
        const SHARED_HANDLE: u32 = 0x171faf4;
        const FIELD_OFF: u32 = 0x14;
        const C1: u32 = 1;
        const C2: u32 = 2;
        let handle = lf_checker_rt::global::<u32>(SHARED_HANDLE).read_unaligned();
        let owner: u32 = lf_checker_rt::callee_thiscall!(C1, u32, handle);
        if owner == 0 {
            return 0;
        }
        let field = ((this + FIELD_OFF) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(C2, u32, owner, 1u32, field)
    }
});
