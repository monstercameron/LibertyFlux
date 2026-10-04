// original: 0x00bb9660 TASK_DRIVE_POINT_ROUTE
/// TASK_DRIVE_POINT_ROUTE: drive-task along a point route.
///
/// Native handler. Forwards ped, route id and a float parameter to the task engine. The float is bit-copied.
export!(cdecl, rw_00bb9660(ctx: u32) -> u32 {
    unsafe {
        // Native call context: +0 = return-slot pointer, +8 = arg array.
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        let a2 = *args.add(2); // bit-copied f32
        callee_cdecl!(1, u32, a0, a1, a2)
    }
});
