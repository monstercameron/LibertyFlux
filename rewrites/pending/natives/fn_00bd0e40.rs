// original: 0x00bd0e40 GET_CURRENT_CHAR_WEAPON
/// Script native `GET_CURRENT_CHAR_WEAPON` (hash 0x5AB8289F).
///
/// Forwards two script arguments (a character handle and an out-pointer)
/// to the engine and stores the low byte of its answer (zero-extended)
/// into the return slot.
export!(cdecl, rw_00bd0e40(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
