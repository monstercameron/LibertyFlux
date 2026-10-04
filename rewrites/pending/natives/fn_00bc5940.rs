// original: 0x00bc5940 GET_CAR_ANIM_TOTAL_TIME
/// Script native `GET_CAR_ANIM_TOTAL_TIME` (hash 0x295C34B8).
///
/// Forwards four script arguments (a vehicle handle, animation names and an
/// out-pointer) to the engine. No return slot is written by the handler
/// itself.
export!(cdecl, rw_00bc5940(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
