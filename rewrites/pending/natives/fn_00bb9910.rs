// original: 0x00BB9910 TASK_FOLLOW_NAV_MESH_TO_COORD_NO_STOP
//
// Forwards the character handle, target coordinates, move state, time and
// radius to the engine task routine. Floats move bitwise. No return value.
export!(cdecl, rw_00bb9910(ctx: *mut u8) -> u32 {
    unsafe {
        let args = args_of(ctx);
        callee_cdecl!(
            1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4),
            *args.add(5), *args.add(6)
        )
    }
});
