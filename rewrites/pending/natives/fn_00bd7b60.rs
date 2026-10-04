// original: 0x00bd7b60 GET_OBJECT_FROM_NETWORK_ID
/// Script native `GET_OBJECT_FROM_NETWORK_ID` (hash 0x7AA91131).
///
/// Forwards two script arguments (network ids) to the engine. No return slot is written.
export!(cdecl, rw_00bd7b60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
