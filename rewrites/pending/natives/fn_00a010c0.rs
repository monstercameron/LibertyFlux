// original: 0x00a010c0 HAS_CLOSEST_OBJECT_OF_TYPE_BEEN_DAMAGED_BY_CHAR
/// Script native `HAS_CLOSEST_OBJECT_OF_TYPE_BEEN_DAMAGED_BY_CHAR`
/// (hash 0x1FC90C7C).
///
/// Forwards six script arguments to the engine (four float bit-patterns, a
/// model hash and a character handle) and stores the low byte of its answer
/// (zero-extended) into the return slot.
export!(cdecl, rw_00a010c0(ctx: *const u8) -> u32 {
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
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
