// original: 0x00bb7540 REQUEST_ANIMS
/// Script native `REQUEST_ANIMS` (hash 0x65F874DE).
///
/// Forwards one script argument (an animation-set name) to the engine
/// streaming request. No return slot is written.
export!(cdecl, rw_00bb7540(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
