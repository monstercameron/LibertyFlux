// original: 0x00BB6530 PLAYSTATS_MISSION_FAILED
// PLAYSTATS_MISSION_FAILED: forward the mission id to the engine call.
export!(cdecl, rw_00BB6530(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        callee_cdecl!(1, u32, *a)
    }
});
