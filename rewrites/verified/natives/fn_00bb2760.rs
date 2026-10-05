// original: 0x00bb2760 MAKE_ROOM_IN_PLAYER_GANG_FOR_MISSION_PEDS
/// Script native `MAKE_ROOM_IN_PLAYER_GANG_FOR_MISSION_PEDS` (hash 0x210E265E).
///
/// Forwards one script argument to the engine. No return slot is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00bb2760(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
