// original: 0x00bba8e0 TASK_SIT_DOWN_ON_SEAT
/// Script native `TASK_SIT_DOWN_ON_SEAT` (hash 0x2CBE4DAF).
///
/// Forwards three integers, four float bit-patterns and one integer (seat task parameters) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bba8e0(ctx: *const u8) -> u32 {
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
            *args.add(7),
        )
    }
});
