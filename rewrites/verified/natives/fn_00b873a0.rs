// original: 0x00b873a0 SET_CAM_INHERIT_ROLL_OBJECT
/// Script native `SET_CAM_INHERIT_ROLL_OBJECT` (hash 0x208B4A6A).
///
/// Forwards arg0, arg1 to the engine.
/// No return slot is written.
export!(cdecl, rw_00b873a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let ans = callee_cdecl!(1, u32, *args.add(0), *args.add(1));
        ans
    }
});
