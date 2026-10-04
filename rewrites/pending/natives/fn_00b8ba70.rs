// original: 0x00b8ba70 ADD_BLIP_FOR_GANG_TERRITORY
/// Script native `ADD_BLIP_FOR_GANG_TERRITORY` (hash 0x2C1B52CE).
///
/// Forwards six script arguments to the engine: four float bit-patterns (a rectangle) followed by two integers. No return slot is written.
export!(cdecl, rw_00b8ba70(ctx: *const u8) -> u32 {
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
