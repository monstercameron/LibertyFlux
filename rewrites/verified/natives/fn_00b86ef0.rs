// original: 0x00B86EF0 IS_SCREEN_FADED_IN
//
// Calls the fade-state query with no arguments and stores its zero-extended
// low byte into the return slot.
export!(cdecl, rw_00b86ef0(ctx: *mut u8) -> u32 {
    unsafe {
        let result = callee_cdecl!(1, u32,);
        *retslot_of(ctx) = result & 0xFF;
        result
    }
});
