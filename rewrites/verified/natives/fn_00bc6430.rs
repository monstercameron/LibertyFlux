// original: 0x00bc6430 GET_TRAIN_CABOOSE
/// Script native `GET_TRAIN_CABOOSE` (hash 0x3FB72D27).
///
/// Forwards two script arguments (a train handle and an index) to the
/// engine worker. No return slot is written.
export!(cdecl, rw_00bc6430(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
