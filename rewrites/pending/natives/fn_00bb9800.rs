// original: 0x00bb9800 TASK_FOLLOW_NAV_MESH_AND_SLIDE_TO_COORD
// rw_task_follow_nav_mesh_and_slide_to_coord: native TASK_FOLLOW_NAV_MESH_AND_SLIDE_TO_COORD (handler 0x00BB9800).
//
// Forwards a char handle, three coord floats, two ints and two floats to the navmesh-slide task assigner. No return slot.
export!(cdecl, rw_task_follow_nav_mesh_and_slide_to_coord(ctx: *mut u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *mut *const u32);
        let ans = callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5), *args.add(6), *args.add(7));
        ans
    }
});
