// original: 0x00bc6040 GET_NUM_CAR_LIVERIES
// GET_NUM_CAR_LIVERIES: call the engine with (arg0, arg1). No return slot.
export!(cdecl, rw_00bc6040(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        callee_cdecl!(1, u32, *a, *a.add(1))
    }
});

/// Argument array of a native call context. (Carried from r-n09 via a-01.)
#[inline(always)]
pub unsafe fn args_of(ctx: u32) -> *const u32 {
    *((ctx as *const u32).add(2)) as *const u32
}
