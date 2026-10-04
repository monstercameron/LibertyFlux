// original: 0x00bc7160 REMOVE_CARS_FROM_GENERATORS_IN_AREA
/// Script native `REMOVE_CARS_FROM_GENERATORS_IN_AREA` (hash 0x2BEE5F97).
///
/// Forwards six float script arguments (two corners of a box) to the
/// engine as raw bits. No return slot is written.
export!(cdecl, rw_00bc7160(ctx: *const u8) -> u32 {
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
