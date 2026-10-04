// original: 0x00a01050 GRAB_NEARBY_OBJECT_WITH_SPECIAL_ATTRIBUTE
/// Script native `GRAB_NEARBY_OBJECT_WITH_SPECIAL_ATTRIBUTE` (hash 0x256472F1).
///
/// Forwards two script arguments to the engine worker.
/// No return slot is written.
export!(cdecl, rw_00a01050(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
