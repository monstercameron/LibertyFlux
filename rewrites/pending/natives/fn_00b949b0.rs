// original: 0x00b949b0 IS_MEMORY_CARD_IN_USE
//
// Calls the save-system query with no arguments and stores its zero-extended
// low byte into the return slot.
export!(cdecl, rw_00b949b0(ctx: *mut u8) -> u32 {
    unsafe {
        let result = callee_cdecl!(1, u32,);
        *retslot_of(ctx) = result & 0xFF;
        result
    }
});
