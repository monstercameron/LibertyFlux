// original: 0x00bd4040 GET_TEXTURE
/// Script native `GET_TEXTURE` (hash 0x0F5D1937).
///
/// Forwards two script arguments (a texture-dictionary and texture name) to the engine and stores its full 32-bit answer into the return slot.
export!(cdecl, rw_00bd4040(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
