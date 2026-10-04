// original: 0x00bd7200 GET_HOURS_OF_DAY
// GET_HOURS_OF_DAY: call the engine with no args, store eax in return slot.
export!(cdecl, rw_00bd7200(ctx: u32) -> u32 {
    unsafe {
        let r = callee_cdecl!(1, u32,);
        *ret_of(ctx) = r;
        r
    }
});
