// original: 0x00CD7920 CTaskComplexFollowPedFootsteps::vf1

/// Resolve the associated ped through the global intelligence context. When
/// that ped exists, request its follow-footsteps task with the object's
/// stored handle at byte offset `0x14`; otherwise return zero. The delegated
/// helper's result is returned unchanged.
///
/// Calling convention: thiscall with no incoming stack arguments. The
/// helper receives one 32-bit stack argument.
lf_checker_rt::export!(thiscall, rw_00cd7920(this: u32) -> u32 {
    const INTELLIGENCE_CONTEXT: u32 = 0x0167_E2A0;
    const FOLLOW_HANDLE: u32 = 0x14;
    const FIND_PED: u32 = 1;
    const CREATE_TASK: u32 = 2;

    unsafe {
        let context = lf_checker_rt::global::<u32>(INTELLIGENCE_CONTEXT).read();
        let ped = lf_checker_rt::callee_thiscall!(FIND_PED, u32, context);
        if ped == 0 {
            0
        } else {
            let handle = ((this + FOLLOW_HANDLE) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(CREATE_TASK, u32, ped, handle)
        }
    }
});
