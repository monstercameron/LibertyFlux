// original: 0x00bbad30 TASK_USE_MOBILE_PHONE
// TASK_USE_MOBILE_PHONE: engine(arg0, quirk(arg1 != 0)). No return value.
// The flag dword is also left in the incoming argument slot, as the original does.
export!(cdecl, rw_00bbad30(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        let flag = quirk_bool(ctx, *a.add(1) != 0);
        *arg_slot_of(ctx) = flag;
        callee_cdecl!(1, u32, *a, flag)
    }
});
