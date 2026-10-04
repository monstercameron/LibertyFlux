// original: 0x00bd76d0 CALCULATE_CHECKSUM
/// Script native `CALCULATE_CHECKSUM` (hash 0x18A302CD).
///
/// Forwards two script arguments to the engine checksum routine and stores
/// the full 32-bit answer into the return slot.
export!(cdecl, rw_00bd76d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let slot = *(ctx as *const u32) as *mut u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        *slot = answer;
        answer
    }
});
