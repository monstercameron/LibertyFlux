// original: 0x00bb9bd0 TASK_GO_TO_COORD_WHILE_SHOOTING
// TASK_GO_TO_COORD_WHILE_SHOOTING: engine(arg0..arg7, flag(arg8 != 0)).
// The flag dword is also left in the incoming argument slot, as the original does.
export!(cdecl, rw_00bb9bd0(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        let flag = u32::from(*a.add(8) != 0);
        *arg_slot_of(ctx) = quirk_bool(ctx, flag != 0);
        callee_cdecl!(1, u32, *a, *a.add(1), *a.add(2), *a.add(3), *a.add(4),
            *a.add(5), *a.add(6), *a.add(7), flag)
    }
});
