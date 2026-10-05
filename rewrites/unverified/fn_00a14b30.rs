// original: 0x00a14b30 CCamFollowVehicle::vf5
/// Publish the camera's mode byte and report success.
///
/// Copies the byte at `this + 0x22c` (the follow-vehicle camera's mode) into
/// the global camera-mode slot and returns 1. Only the low byte of the
/// return register is set; the upper bytes keep their entry value, so the
/// contract compares `al` only. Thiscall, no stack arguments.
export!(thiscall, rw_00a14b30(this: u32) -> u32 {
    unsafe {
        const MODE_OFF: u32 = 0x22c;
        const MODE_GLOBAL: u32 = 0x0103b924;
        let mode = (this + MODE_OFF) as *const u8;
        *global::<u8>(MODE_GLOBAL) = *mode;
        1
    }
});
