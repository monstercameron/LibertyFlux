// original: 0x00bb2e00 TAKE_REMOTE_CONTROL_OF_CAR
/// Script native `TAKE_REMOTE_CONTROL_OF_CAR` (hash 0x5DEA7140).
///
/// Forwards two script arguments (a character handle and a vehicle handle) to the engine. No return slot is written.
export!(cdecl, rw_00bb2e00(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
