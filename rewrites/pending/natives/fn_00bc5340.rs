// original: 0x00bc5340 CONTROL_CAR_DOOR
/// Script native `CONTROL_CAR_DOOR` (hash 0x194F76D4).
///
/// Forwards four script arguments (a vehicle handle, two integers and one
/// float bit-pattern) to the engine. No return slot is written.
export!(cdecl, rw_00bc5340(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
