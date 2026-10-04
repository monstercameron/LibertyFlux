// original: 0x00a01350 IS_NON_FRAG_OBJECT_SMASHED
/// Script native `IS_NON_FRAG_OBJECT_SMASHED` (hash 0x5C723F31).
///
/// Forwards four float bit-patterns (a position and radius) and an object
/// handle to the engine, then stores the low byte of its answer
/// (zero-extended) into the return slot.
export!(cdecl, rw_00a01350(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let slot = *(ctx as *const u32) as *mut u32;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
        );
        *slot = answer & 0xFF;
        slot as u32
    }
});
