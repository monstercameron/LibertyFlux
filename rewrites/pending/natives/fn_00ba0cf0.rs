// original: 0x00ba0cf0 REMOVE_CHAR_FROM_CAR_MAINTAIN_POSITION
/// Script native `REMOVE_CHAR_FROM_CAR_MAINTAIN_POSITION` (hash 0x3DA4533F).
///
/// Forwards two script arguments (a character handle and a vehicle handle)
/// to the engine. No return slot is written.
export!(cdecl, rw_00ba0cf0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
