// original: 0x00b86680 ATTACH_CAM_TO_OBJECT
/// Script native `ATTACH_CAM_TO_OBJECT` (hash 0x2966710D).
///
/// Forwards two script arguments (a camera handle and an object handle) to
/// the engine. No return slot is written; the engine answer is left in EAX.
export!(cdecl, rw_00b86680(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
