// original: 0x00b87170 PROSTITUTE_CAM_ACTIVATE
// PROSTITUTE_CAM_ACTIVATE: coerce arg0 to bool (stack-slot quirk), engine(flag).
// The flag dword is also left in the incoming argument slot, as the original does.
export!(cdecl, rw_00b87170(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        let flag = u32::from(*a != 0);
        *arg_slot_of(ctx) = quirk_bool(ctx, flag != 0);
        callee_cdecl!(1, u32, flag)
    }
});
