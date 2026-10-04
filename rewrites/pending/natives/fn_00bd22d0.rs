// original: 0x00bd22d0 IS_EXPLOSION_IN_AREA
/// Script native `IS_EXPLOSION_IN_AREA` (hash 0x676B6BCA).
///
/// Forwards seven script arguments to the engine: an integer followed by
/// six float bit-patterns (the area corners). Floats are copied as raw
/// bits. Stores the low byte of the engine answer (zero-extended) into
/// the return slot.
export!(cdecl, rw_00bd22d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args.add(5),
            *args.add(6),
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
