// original: 0x00b87340 SET_CAM_FAR_CLIP
/// Script native `SET_CAM_FAR_CLIP` (hash 0x181F6B00).
///
/// Forwards two script arguments (a camera handle and a float distance, copied as raw bits) to the engine. The original stages the float through a scratch stack slot; the pushed values are exactly the two argument words. No return slot is written.
export!(cdecl, rw_00b87340(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
