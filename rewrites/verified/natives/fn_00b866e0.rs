// original: 0x00b866e0 ATTACH_CAM_TO_VIEWPORT
/// Script native `ATTACH_CAM_TO_VIEWPORT` (hash 0x21A3110A).
///
/// Forwards two script arguments (a camera handle and a viewport id) to the
/// engine. No return slot is written.
export!(cdecl, rw_00b866e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
