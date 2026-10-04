// original: 0x00bd1070 HAS_CHAR_BEEN_DAMAGED_BY_WEAPON
/// Script native `HAS_CHAR_BEEN_DAMAGED_BY_WEAPON` (hash 0x6DB26E07).
///
/// Forwards two script arguments (a character handle and a weapon id) to the engine and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bd1070(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args.add(0), *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
