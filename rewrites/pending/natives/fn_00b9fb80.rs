// original: 0x00b9fb80 IS_CHAR_IN_MELEE_COMBAT
//
// Forwards the character handle; the engine answers in the low byte and the
// handler zero-extends it into the return slot.
export!(cdecl, rw_00b9fb80(ctx: *mut u8) -> u32 {
    unsafe {
        let args = args_of(ctx);
        let result = callee_cdecl!(1, u32, *args);
        *retslot_of(ctx) = result & 0xFF;
        result
    }
});
