// original: 0x00b8afe0 ADD_WIDGET_SLIDER
/// Script native `ADD_WIDGET_SLIDER` (hash 0x4A904476).
///
/// Forwards five script arguments (widget id, bounds and step values) to
/// the engine and stores the full 32-bit engine answer into the return slot.
export!(cdecl, rw_00b8afe0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
