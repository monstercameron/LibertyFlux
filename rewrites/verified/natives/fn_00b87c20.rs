// original: 0x00b87c20 SET_ROOM_FOR_VIEWPORT_BY_NAME
/// Set a viewport's room by name.
///
/// Forwards the viewport id and room-name pointer (arguments 0-1) to the
/// engine implementation. Returns whatever the engine call returned.
export!(cdecl, rw_00b87c20(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
