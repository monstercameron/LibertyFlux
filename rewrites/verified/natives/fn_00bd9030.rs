// original: 0x00bd9030 RESERVE_NETWORK_MISSION_PEDS
/// Script native `RESERVE_NETWORK_MISSION_PEDS` (hash 0x4B2F6DDC).
///
/// Forwards one script argument to the engine. No return slot is written. (The original cleans its one pushed argument with `(an instruction of the original)`; the effect on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00bd9030(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
