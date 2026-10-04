// original: 0x00bd1050 HAS_CAR_BEEN_DAMAGED_BY_WEAPON
/// Script native `HAS_CAR_BEEN_DAMAGED_BY_WEAPON` (hash 0x0EE34390).
///
/// Forwards two script arguments (a vehicle handle and a weapon type) to
/// the engine and stores the low byte of its answer (zero-extended) into
/// the return slot.
export!(cdecl, rw_00bd1050(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
