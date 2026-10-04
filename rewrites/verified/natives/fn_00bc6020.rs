// original: 0x00BC6020 GET_NUM_CAR_COLOURS
//
// Script native handler: forwards the vehicle handle and an out-pointer to
// one engine function (cdecl/2), which reports through the pointer. The
// handler itself stores nothing to the return slot.
export!(cdecl, rw_00bc6020(ctx: *const u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *const *const u32);
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
