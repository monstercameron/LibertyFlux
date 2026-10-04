// original: 0x00bc6040 GET_NUM_CAR_LIVERIES
// GET_NUM_CAR_LIVERIES: call the engine with (arg0, arg1). No return slot.
export!(cdecl, rw_00bc6040(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        callee_cdecl!(1, u32, *a, *a.add(1))
    }
});
