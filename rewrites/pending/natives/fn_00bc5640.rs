// original: 0x00bc5640 DOES_CAR_HAVE_HYDRAULICS
/// Script native `DOES_CAR_HAVE_HYDRAULICS` (hash 0x0F0956CA).
///
/// Forwards a vehicle handle to the engine, which looks it up in the
/// vehicle pool and tests the hydraulics flag. Stores the low byte of the
/// engine answer (zero-extended) into the return slot.
export!(cdecl, rw_00bc5640(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
