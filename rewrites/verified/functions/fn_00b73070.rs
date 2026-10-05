// original: 0x00b73070 CTaskSimpleSetPedAsAutoDriver::vf17

/// Task update: switch the ped's vehicle on with an auto-driver flag of 0.
///
/// Loads the vehicle pointer from `this+0x14` and calls the engine-on callee
/// (thiscall on the vehicle, one stack word: 0). Always reports done: the
/// low return byte is 1 and the upper 24 bits repeat the callee's answer
/// (the original sets only al).
///
/// Original: 0x00b73070 (thiscall, one stack word, unread).
lf_checker_rt::export!(thiscall, rw_00b73070(this: u32, _ped: u32) -> u32 {
    unsafe {
        const VEHICLE_OFF: u32 = 0x14;
        const ENGINE_ON_CALLEE: u32 = 1;
        const DONE: u32 = 1;
        let vehicle = ((this + VEHICLE_OFF) as *const u32).read_unaligned();
        let ans: u32 = lf_checker_rt::callee_thiscall!(ENGINE_ON_CALLEE, u32, vehicle, 0);
        (ans & 0xffff_ff00) | DONE
    }
});
