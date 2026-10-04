// original: 0x00b86300 ALLOCATE_SCRIPT_TO_RANDOM_PED
// ALLOCATE_SCRIPT_TO_RANDOM_PED: coerce arg3 to bool (stack-slot quirk),
// call the engine with (arg0, arg1, arg2, flag). No return value. The flag
// dword is also left in the incoming argument slot, as the original does.
export!(cdecl, rw_00b86300(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        let flag = u32::from(*a.add(3) != 0);
        *arg_slot_of(ctx) = quirk_bool(ctx, flag != 0);
        callee_cdecl!(1, u32, *a, *a.add(1), *a.add(2), flag)
    }
});
