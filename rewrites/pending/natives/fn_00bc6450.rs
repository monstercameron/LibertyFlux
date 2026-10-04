// original: 0x00bc6450 GET_TRAIN_CARRIAGE
/// Script native `GET_TRAIN_CARRIAGE` (hash 0x7F861E46).
///
/// Forwards three script arguments (a train handle, a carriage index and an
/// out-pointer) to the engine. No return slot is written by the handler
/// itself.
export!(cdecl, rw_00bc6450(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
