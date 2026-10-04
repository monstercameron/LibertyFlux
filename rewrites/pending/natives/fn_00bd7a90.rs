// original: 0x00bd7a90 GET_NETWORK_ID_FROM_OBJECT
/// Script native `GET_NETWORK_ID_FROM_OBJECT` (hash 0x50424095).
///
/// Forwards two script arguments to the engine. No return slot is written.
export!(cdecl, rw_00bd7a90(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
