// original: 0x00b9f240 GET_GROUP_FORMATION
/// Script native `GET_GROUP_FORMATION` (hash 0x596174E5).
///
/// Forwards arg0, arg1 to the engine.
/// No return slot is written.
export!(cdecl, rw_00b9f240(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let ans = callee_cdecl!(1, u32, *args.add(0), *args.add(1));
        ans
    }
});
