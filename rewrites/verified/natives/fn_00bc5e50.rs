// original: 0x00bc5e50 GET_DRIVER_OF_CAR
/// Script native `GET_DRIVER_OF_CAR` (hash 0x22457083).
///
/// Forwards two script arguments (a vehicle handle and a second word)
/// to the engine. No return slot is written.
export!(cdecl, rw_00bc5e50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
