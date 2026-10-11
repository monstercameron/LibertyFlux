// original: 0x00CD7950 CTaskComplexSeekEntityAiming::vf1

/// Resolve the ped through the shared intelligence context and, when one is
/// present, request an aiming task with the object's stored handle and two
/// float parameters. The float arguments are transported as their exact
/// 32-bit representations; this method performs no floating-point arithmetic.
/// A missing ped produces zero, otherwise the helper's result is returned.
///
/// Calling convention: thiscall with no incoming stack arguments. The helper
/// receives the handle, the first float at byte offset `0x18`, and the second
/// float at `0x1C` as three stack words.
lf_checker_rt::export!(thiscall, rw_00cd7950(this: u32) -> u32 {
    const INTELLIGENCE_CONTEXT: u32 = 0x0167_E2A0;
    const FOLLOW_HANDLE: u32 = 0x14;
    const FIRST_FLOAT: u32 = 0x18;
    const SECOND_FLOAT: u32 = 0x1C;
    const FIND_PED: u32 = 1;
    const CREATE_TASK: u32 = 2;

    unsafe {
        let context = lf_checker_rt::global::<u32>(INTELLIGENCE_CONTEXT).read();
        let ped = lf_checker_rt::callee_thiscall!(FIND_PED, u32, context);
        if ped == 0 {
            0
        } else {
            let handle = ((this + FOLLOW_HANDLE) as *const u32).read_unaligned();
            let first_float_bits = ((this + FIRST_FLOAT) as *const u32).read_unaligned();
            let second_float_bits = ((this + SECOND_FLOAT) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(
                CREATE_TASK,
                u32,
                ped,
                handle,
                first_float_bits,
                second_float_bits
            )
        }
    }
});
