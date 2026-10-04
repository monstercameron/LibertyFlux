// original: 0x00bd41d0 REMOVE_TXD
/// Script native `REMOVE_TXD` (hash 0x44C27071).
///
/// Forwards one script argument (a texture-dictionary handle) to the
/// engine. No return slot is written.
export!(cdecl, rw_00bd41d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
