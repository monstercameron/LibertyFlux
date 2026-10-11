// original: 0x00CD78C0 CTaskComplexFollowLeaderAnyMeans::vf1

/// Find the associated ped through the global intelligence context. If one is
/// available, ask it to create the next task using the two stored object
/// handles; otherwise return zero. The helper's result is returned unchanged.
///
/// Object fields are the first handle at byte offset `0x14` and the second at
/// `0x18`. The global slot stores the intelligence context pointer.
///
/// Calling convention: thiscall with no incoming stack arguments. The two
/// outgoing helper arguments are pushed in reverse order, so the helper
/// observes the first handle followed by the second.
lf_checker_rt::export!(thiscall, rw_00cd78c0(this: u32) -> u32 {
    const INTELLIGENCE_CONTEXT: u32 = 0x0167_E2A0;
    const FIRST_HANDLE: u32 = 0x14;
    const SECOND_HANDLE: u32 = 0x18;
    const FIND_PED: u32 = 1;
    const CREATE_TASK: u32 = 2;

    unsafe {
        let context = lf_checker_rt::global::<u32>(INTELLIGENCE_CONTEXT).read();
        let ped = lf_checker_rt::callee_thiscall!(FIND_PED, u32, context);
        if ped == 0 {
            0
        } else {
            let first = ((this + FIRST_HANDLE) as *const u32).read_unaligned();
            let second = ((this + SECOND_HANDLE) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(CREATE_TASK, u32, ped, first, second)
        }
    }
});
