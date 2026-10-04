// original: 0x00bc6560 HAS_CAR_BEEN_DAMAGED_BY_CHAR
/// Script native `HAS_CAR_BEEN_DAMAGED_BY_CHAR` (hash 0x61487DBF).
///
/// Forwards two script arguments (a vehicle handle and a character handle)
/// to the engine and stores the low byte of its answer (zero-extended)
/// into the return slot.
export!(cdecl, rw_00bc6560(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
