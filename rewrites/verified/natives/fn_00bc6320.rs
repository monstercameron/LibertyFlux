// original: 0x00bc6320 GET_RANDOM_CAR_OF_TYPE_IN_ANGLED_AREA_NO_SAVE
/// Script native `GET_RANDOM_CAR_OF_TYPE_IN_ANGLED_AREA_NO_SAVE` (hash 0x6D4746D8).
///
/// Forwards seven script arguments to the engine: five float bit-patterns (area corners and heading) followed by two integers. Floats are copied as raw bits, so the forward is bit-exact. No return slot is written.
export!(cdecl, rw_00bc6320(ctx: *const u8) -> u32 {
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
            *args.add(6),
        )
    }
});
