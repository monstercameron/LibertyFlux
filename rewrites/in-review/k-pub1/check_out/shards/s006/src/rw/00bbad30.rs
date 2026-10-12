// original: 0x00bbad30 TASK_USE_MOBILE_PHONE
// TASK_USE_MOBILE_PHONE: engine(arg0, flag(arg1 != 0)). No return value.
// The flag dword is also left in the incoming argument slot, as the original does.
export!(cdecl, rw_00bbad30(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        let flag = u32::from(*a.add(1) != 0);
        *arg_slot_of(ctx) = quirk_bool(ctx, flag != 0);
        __lf_callee_cdecl!(1, u32, *a, flag)
    }
});

/// Argument array of a native call context. (Carried from r-n09 via a-01.)
#[inline(always)]
pub unsafe fn args_of(ctx: u32) -> *const u32 {
    *((ctx as *const u32).add(2)) as *const u32
}

/// Address of this call's incoming argument slot, passed by the harness in
/// `ctx+0xC`. (Carried from r-n09 via a-01.)
#[inline(always)]
pub unsafe fn arg_slot_of(ctx: u32) -> *mut u32 {
    *((ctx as *const u32).add(3)) as *mut u32
}

/// The stack-slot boolean quirk: low byte of `ctx` replaced by the flag.
/// (Carried from r-n09 via a-01.)
#[inline(always)]
pub fn quirk_bool(ctx: u32, cond: bool) -> u32 {
    (ctx & 0xFFFF_FF00) | (cond as u32)
}
