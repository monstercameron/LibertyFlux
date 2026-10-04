// original: 0x00bc8310 TURN_CAR_TO_FACE_COORD
/// Script native `TURN_CAR_TO_FACE_COORD` (hash 0x16184716).
///
/// Forwards three script arguments to the engine: a vehicle handle and two
/// float bit-patterns (coordinates). Floats are copied as raw bits, so the
/// forward is bit-exact. No return slot is written.
export!(cdecl, rw_00bc8310(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
