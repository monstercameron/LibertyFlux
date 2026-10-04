// original: 0x006617e0 rage::snDropGamersTask::vf7
/// Drop-gamers task teardown step: forward the two arguments, drop session.
///
/// thiscall/2, returns void. Forwards both arguments to the shared teardown
/// helper, then clears the session link.
export!(thiscall, rw_006617e0(this_ptr: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this_ptr, a0, a1);
        ((this_ptr + 0x60) as *mut u32).write(0);
        0
    }
});
