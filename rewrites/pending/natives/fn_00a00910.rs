// original: 0x00a00910 DELETE_OBJECT
/// Script native `DELETE_OBJECT` (hash 0x62FE6290).
///
/// Forwards one script argument to the engine. No return slot is written.
export!(cdecl, rw_00a00910(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
