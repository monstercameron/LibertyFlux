// original: 0x00b9f2c0 GET_GROUP_SIZE
/// Script native `GET_GROUP_SIZE` (hash 0x45EE4E9A).
///
/// Forwards three script arguments (a group handle and two out slots) to the engine. No return slot is written.
export!(cdecl, rw_00b9f2c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
