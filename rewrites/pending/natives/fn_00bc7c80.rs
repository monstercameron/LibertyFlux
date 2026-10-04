// original: 0x00bc7c80 SET_ROCKET_LAUNCHER_FREEBIE_IN_HELI
/// Toggle the rocket-launcher freebie in helicopters.
///
/// Forwards the flag (argument 0, normalised to 0/1) to the engine. The pushed
/// flag dword carries the context pointer's high bytes (see `rw_00b9ebf0`),
/// reproduced exactly. Returns whatever the engine returned.
export!(cdecl, rw_00bc7c80(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let flag = u32::from(*args != 0);
        let pushed = (ctx & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, pushed)
    }
});
