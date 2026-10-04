// original: 0x00bbad30 TASK_USE_MOBILE_PHONE
// TASK_USE_MOBILE_PHONE: engine(arg0, flag(arg1 != 0)). No return value.
// The flag dword is also left in the incoming argument slot, as the original does.
export!(cdecl, rw_00bbad30(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        let flag = u32::from(*a.add(1) != 0);
        *arg_slot_of(ctx) = quirk_bool(ctx, flag != 0);
        callee_cdecl!(1, u32, *a, flag)
    }
});
