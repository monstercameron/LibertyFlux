// original: 0x00a01180 HAS_OBJECT_BEEN_DAMAGED_BY_CAR
/// Script native `HAS_OBJECT_BEEN_DAMAGED_BY_CAR` (hash 0x50801274).
///
/// Forwards an object handle and a car handle to the engine, then stores
/// the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00a01180(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let slot = *(ctx as *const u32) as *mut u32;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
        );
        *slot = answer & 0xFF;
        slot as u32
    }
});
