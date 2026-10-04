// original: 0x00b93f60 CLEAR_ANGLED_AREA_OF_CARS
/// Script native `CLEAR_ANGLED_AREA_OF_CARS` (hash 0x7E2A7743).
///
/// Builds the engine argument block from seven float script arguments (two corner triples and one value) and calls the engine worker. No return slot is written.
export!(cdecl, rw_00b93f60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        let a2 = *args.add(2);
        let a3 = *args.add(3);
        let a4 = *args.add(4);
        let a5 = *args.add(5);
        let a6 = *args.add(6);
        callee_cdecl!(
            1,
            u32,
            a0,
            a1,
            a2,
            a3,
            a4,
            a5,
            a6,
            a3,
            a4,
            a5,
            a0,
            a1,
            a2,
        )
    }
});
