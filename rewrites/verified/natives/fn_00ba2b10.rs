// original: 0x00ba2b10 WARP_CHAR_INTO_CAR
/// Script native `WARP_CHAR_INTO_CAR` (hash 0x73D3504A).
///
/// Forwards two script arguments (a character handle and a vehicle handle)
/// to the engine. No return slot is written.
export!(cdecl, rw_00ba2b10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
