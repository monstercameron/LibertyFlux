// original: 0x00a00f90 GET_SAFE_PICKUP_COORDS
/// Script native `GET_SAFE_PICKUP_COORDS` (hash 0x1AE44443).
///
/// Forwards six script arguments to the engine: three float bit-patterns
/// (coordinates) followed by three integers. The original stages the
/// floats through a stack temporary, but the callee receives every word
/// in script order, so the forward is a plain copy. No return slot is
/// written.
export!(cdecl, rw_00a00f90(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
        1,
        u32,
        *args,
        *args.add(1),
        *args.add(2),
        *args.add(3),
        *args.add(4),
        *args.add(5),
        )
    }
});
