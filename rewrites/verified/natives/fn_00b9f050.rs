// original: 0x00b9f050 GET_CHAR_SWIM_STATE
/// Script native `GET_CHAR_SWIM_STATE` (hash 0x34460DD7).
///
/// Forwards two script arguments (a character handle and an out-pointer) to the engine. No return slot is written by the handler itself.
export!(cdecl, rw_00b9f050(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
