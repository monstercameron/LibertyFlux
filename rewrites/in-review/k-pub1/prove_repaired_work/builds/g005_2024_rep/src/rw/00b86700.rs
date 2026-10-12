// original: 0x00b86700 BEGIN_CAM_COMMANDS
// BEGIN_CAM_COMMANDS: call the engine with arg0. No return value.
export!(cdecl, rw_00b86700(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        callee_cdecl!(1, u32, *a)
    }
});

/// Argument array of a native call context. (Carried from r-n09 via a-01.)
#[inline(always)]
pub unsafe fn args_of(ctx: u32) -> *const u32 {
    *((ctx as *const u32).add(2)) as *const u32
}
