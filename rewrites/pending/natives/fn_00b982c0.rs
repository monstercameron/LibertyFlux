// original: 0x00b982c0 GET_MOTION_CONTROLS_ENABLED
// rw_get_motion_controls_enabled: native GET_MOTION_CONTROLS_ENABLED (handler 0x00B982C0).
//
// Forwards two out-pointer args to the motion-controls query; the engine writes through them. No return slot.
export!(cdecl, rw_get_motion_controls_enabled(ctx: *mut u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *mut *const u32);
        let ans = callee_cdecl!(1, u32, *args.add(0), *args.add(1));
        ans
    }
});
