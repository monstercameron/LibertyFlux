// original: 0x00B8CBA0 GET_WIDTH_OF_SUBSTRING_GIVEN_TEXT_LABEL
/// Native handler `GET_WIDTH_OF_SUBSTRING_GIVEN_TEXT_LABEL`.
///
/// Forward a label, a coerced flag and three words; store the float width result.
///
/// Engine returns the width in ST0; the handler stores it through the return slot.
///
/// `ctx` is the native call context: word 0 points at the return
/// slot, word 2 points at the script argument array.
lf_k2_rt::export!(cdecl, rw_00b8cba0(ctx: u32) -> u32 {
    unsafe {
        let ctx_words = ctx as *const u32;
        let args = *ctx_words.add(2) as *const u32;
        let ret_slot = *ctx_words as *mut u32;
        let flag1 = u32::from(*args.add(1) != 0);
        let engine: extern "cdecl" fn(u32, u32, u32, u32, u32) -> f32 =
            core::mem::transmute(lf_k2_rt::callee_addr(1) as usize);
        let width = engine(*args.add(0), flag1, *args.add(2), *args.add(3), *args.add(4));
        *(ret_slot as *mut f32) = width;
        ret_slot as u32
    }
});
