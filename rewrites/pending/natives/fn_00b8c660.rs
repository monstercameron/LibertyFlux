// original: 0x00b8c660 GET_BLIP_ALPHA
/// Script native `GET_BLIP_ALPHA` (hash 0x61497585).
///
/// Forwards two script arguments (a blip handle and an out-reference) to the engine. No return slot is written by the handler itself.
export!(cdecl, rw_00b8c660(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
