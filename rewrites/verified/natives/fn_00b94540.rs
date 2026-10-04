// original: 0x00b94540 GET_HEADING_FROM_VECTOR_2D
/// Script native `GET_HEADING_FROM_VECTOR_2D` (hash 0x09DD61E1).
///
/// Forwards three script arguments to the engine: two float vector
/// components as raw bits and one integer. No return slot is written.
export!(cdecl, rw_00b94540(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
