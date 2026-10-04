// original: 0x00bc5b00 GET_CAR_FORWARD_Y
/// Script native `GET_CAR_FORWARD_Y` (hash 0x3BDB4496).
///
/// Forwards two script arguments (a vehicle handle and an out-coordinate index) to the engine. No return slot is written.
export!(cdecl, rw_00bc5b00(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
