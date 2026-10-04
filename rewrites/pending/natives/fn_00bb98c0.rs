// original: 0x00bb98c0 TASK_FOLLOW_NAV_MESH_TO_COORD
/// Script native `TASK_FOLLOW_NAV_MESH_TO_COORD` (hash 0x1B31390E).
///
/// Orders a character to follow the navigation mesh: forwards a character
/// handle, three float bit-patterns (target coordinates), two more words
/// and one more float bit-pattern to the engine. No return slot is written.
export!(cdecl, rw_00bb98c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args.add(5),
            *args.add(6),
        )
    }
});
