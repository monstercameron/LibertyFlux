// original: 0x00a009e0 DOES_OBJECT_OF_TYPE_EXIST_AT_COORDS
/// Script native `DOES_OBJECT_OF_TYPE_EXIST_AT_COORDS` (hash 0x1F881A88).
///
/// Forwards five script arguments to the engine: three float bit-patterns
/// (coordinates), a float radius, and an object-type handle; then stores
/// the low byte of the engine answer (zero-extended) into the return slot.
/// Float arguments are copied as raw bits, so forwarding is bit-exact.
export!(cdecl, rw_00a009e0(ctx: *const u8) -> u32 {
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
