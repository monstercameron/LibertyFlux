// original: 0x00a01b10 SET_HEALTH_PICKUP_NETWORK_REGEN_TIME
/// Script native `SET_HEALTH_PICKUP_NETWORK_REGEN_TIME` (hash 0x072516B4).
///
/// Forwards one script argument (a time in milliseconds) to the engine. No
/// return slot is written. (The original cleans its one pushed argument
/// with `(an instruction of the original)`; the effect on the stack pointer is identical to the
/// plain cdecl return here.)
export!(cdecl, rw_00a01b10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
