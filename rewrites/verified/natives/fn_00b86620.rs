// original: 0x00b86620 ADD_CAM_SPLINE_NODE
/// Script native `ADD_CAM_SPLINE_NODE` (hash 0x3B4F1EBA).
///
/// Forwards two script arguments (a camera handle and a node index) to the
/// engine. No return slot is written; the engine answer is the exit value.
export!(cdecl, rw_00b86620(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
