// original: 0x00a00df0 GET_OBJECT_MODEL
/// Script native `GET_OBJECT_MODEL` (hash 0x5CC55619).
///
/// Forwards two script arguments (an object handle and an out-pointer) to
/// the engine. No return slot is written by the handler itself.
export!(cdecl, rw_00a00df0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
