// original: 0x00bb9680 TASK_DRIVE_POINT_ROUTE_ADVANCED
/// Script native `TASK_DRIVE_POINT_ROUTE_ADVANCED` (hash 0x7A0A1063).
///
/// Forwards six script arguments to the engine: two leading words, one
/// float bit-pattern the handler moves through a vector register, and three
/// trailing words. The float is forwarded as raw bits, bit-exact by
/// construction. No return slot is written.
export!(cdecl, rw_00bb9680(ctx: *const u8) -> u32 {
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
