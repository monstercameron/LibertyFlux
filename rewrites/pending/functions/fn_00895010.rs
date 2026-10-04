// original: 0x00895010 create_and_await_ready
/// Create a handle through a seven-argument factory, then wait for readiness.
///
/// The factory result is stored on the object. While the object's ready byte
/// is clear, a poll call runs until another thread sets it. Returns the
/// stored tag with its low byte forced to 1.
crate::rt::export!(thiscall, rs17_00895010(this: u32) -> u32 {
    const READY_OFF: u32 = 0x320;
    const TAG_OFF: u32 = 0x324;
    let callback = crate::rt::relocated(0x00895360);
    let params = crate::rt::relocated(0x00E78BA0);
    let handle = crate::rt::callee_cdecl!(5, u32, callback, 0, 0x10000, 0, params, 1, 2);
    unsafe { ((this + TAG_OFF) as *mut u32).write(handle) };
    let mut tag = handle;
    if unsafe { ((this + READY_OFF) as *const u8).read() } == 0 {
        loop {
            tag = crate::rt::callee_cdecl!(6, u32, 0x0A);
            if unsafe { ((this + READY_OFF) as *const u8).read() } != 0 {
                break;
            }
        }
    }
    (tag & !0xFF) | 1
});
