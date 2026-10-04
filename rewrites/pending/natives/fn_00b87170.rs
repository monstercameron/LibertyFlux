// original: 0x00b87170 PROSTITUTE_CAM_ACTIVATE
// PROSTITUTE_CAM_ACTIVATE: coerce arg0 to bool (stack-slot quirk), engine(flag).
// The flag dword is also left in the incoming argument slot, as the original does.
export!(cdecl, rw_00b87170(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        let flag = quirk_bool(ctx, *a != 0);
        *arg_slot_of(ctx) = flag;
        callee_cdecl!(1, u32, flag)
    }
});
