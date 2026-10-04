// original: 0x00B8C4A0 DOES_TEXT_LABEL_EXIST
// DOES_TEXT_LABEL_EXIST: forward the label hash; store the answer's low
// byte through the return slot. Returns the slot.
export!(cdecl, rw_00B8C4A0(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        let found = callee_cdecl!(1, u32, *a) & 0xFF;
        let slot = ret_slot(ctx);
        *slot = found;
        slot as u32
    }
});
