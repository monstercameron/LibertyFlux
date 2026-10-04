// original: 0x00ba0c10 MP_SET_PREFERENCE_VALUE
/// Script native `MP_SET_PREFERENCE_VALUE` (hash 0x216804D3).
///
/// Forwards two script arguments (a preference index and its value) to the
/// engine. No return slot is written.
export!(cdecl, rw_00ba0c10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
