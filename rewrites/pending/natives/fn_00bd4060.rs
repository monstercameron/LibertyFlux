// original: 0x00bd4060 GET_TEXTURE_FROM_STREAMED_TXD
/// Script native `GET_TEXTURE_FROM_STREAMED_TXD` (hash 0x32C24491).
///
/// Forwards two script arguments (a streamed texture-dictionary name and a texture name) to the engine and stores its full 32-bit answer into the return slot. Unlike the boolean natives, this handler keeps the whole answer (`mov`, not `movzx`).
export!(cdecl, rw_00bd4060(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
