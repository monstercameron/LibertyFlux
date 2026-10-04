// original: 0x00bd4190 REMOVE_PTFX_FROM_OBJECT
/// Script native `REMOVE_PTFX_FROM_OBJECT` (hash 0x4D7775BA).
///
/// Forwards one script argument to the engine. No return slot is written.
export!(cdecl, rw_00bd4190(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
