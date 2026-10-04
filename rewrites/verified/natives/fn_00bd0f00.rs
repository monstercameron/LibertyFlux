// original: 0x00bd0f00 GET_MAX_AMMO
/// Script native `GET_MAX_AMMO` (hash 0x7C6968F8).
///
/// Forwards three script arguments (a character handle, a weapon id and an
/// out-pointer) to the engine and stores the low byte of its answer
/// (zero-extended) into the return slot.
export!(cdecl, rw_00bd0f00(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
