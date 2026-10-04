// original: 0x00ba0d30 REMOVE_FAKE_NETWORK_NAME_FROM_PED
/// Script native `REMOVE_FAKE_NETWORK_NAME_FROM_PED` (hash 0x37A86FBD).
///
/// Forwards one script argument to the engine.
/// No return slot is written.
export!(cdecl, rw_00ba0d30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
