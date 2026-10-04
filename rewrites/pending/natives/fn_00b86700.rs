// original: 0x00b86700 BEGIN_CAM_COMMANDS
// BEGIN_CAM_COMMANDS: call the engine with arg0. No return value.
export!(cdecl, rw_00b86700(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        callee_cdecl!(1, u32, *a)
    }
});
