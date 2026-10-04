// original: 0x00bb95d0 TASK_DESTROY_CAR
/// Script native `TASK_DESTROY_CAR` (hash 0x787A3D4C).
///
/// Forwards two script arguments (a character handle and a vehicle handle) to the engine. No return slot is written.
export!(cdecl, rw_00bb95d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args.add(0), *args.add(1))
    }
});
