// original: 0x00bd4130 RELEASE_TEXTURE
/// Script native `RELEASE_TEXTURE` (hash 0x58524B04).
///
/// Forwards one script argument (a texture id) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bd4130(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
