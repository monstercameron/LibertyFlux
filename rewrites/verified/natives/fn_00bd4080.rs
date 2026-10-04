// original: 0x00bd4080 GET_TEXTURE_RESOLUTION
/// Script native `GET_TEXTURE_RESOLUTION` (hash 0x01A75F0C).
///
/// Forwards three script arguments to the engine texture query. No return
/// slot is written.
export!(cdecl, rw_00bd4080(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
