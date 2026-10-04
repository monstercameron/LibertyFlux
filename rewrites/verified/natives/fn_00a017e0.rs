// original: 0x00a017e0 PLACE_OBJECT_RELATIVE_TO_CAR
/// Script native `PLACE_OBJECT_RELATIVE_TO_CAR` (hash 0x21DE7496).
///
/// Forwards five script arguments (an object handle, a vehicle handle and
/// three float words, bit-for-bit) to the engine. No return slot is
/// written.
export!(cdecl, rw_00a017e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4)
        )
    }
});
