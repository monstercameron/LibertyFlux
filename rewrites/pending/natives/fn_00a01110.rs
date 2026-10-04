// original: 0x00a01110 HAS_FRAGMENT_ROOT_OF_CLOSEST_OBJECT_OF_TYPE_BEEN_DAMAGED
/// Script native `HAS_FRAGMENT_ROOT_OF_CLOSEST_OBJECT_OF_TYPE_BEEN_DAMAGED`.
///
/// Forwards five script arguments to the engine: four float bit-patterns
/// followed by one integer. Floats are copied as raw bits, so the forward
/// is bit-exact. Stores the low byte of the answer (zero-extended) into
/// the return slot. Returns the slot pointer.
export!(cdecl, rw_00a01110(ctx: *const u8) -> u32 {
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
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
