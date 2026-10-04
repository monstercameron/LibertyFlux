// original: 0x00b86300 ALLOCATE_SCRIPT_TO_RANDOM_PED
// ALLOCATE_SCRIPT_TO_RANDOM_PED: coerce arg3 to bool (stack-slot quirk),
// call the engine with (arg0, arg1, arg2, flag). No return value. The flag
// dword is also left in the incoming argument slot, as the original does.
export!(cdecl, rw_00b86300(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        let flag = quirk_bool(ctx, *a.add(3) != 0);
        *arg_slot_of(ctx) = flag;
        callee_cdecl!(1, u32, *a, *a.add(1), *a.add(2), flag)
    }
});
