// original: 0x00b9efd0 GET_CHAR_PROP_INDEX
/// Script native `GET_CHAR_PROP_INDEX` (hash 0x3AC85DB1).
///
/// Forwards three script arguments (a character handle, a prop slot and an
/// out-pointer) to the engine. No return slot is written by the handler
/// itself.
export!(cdecl, rw_00b9efd0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
