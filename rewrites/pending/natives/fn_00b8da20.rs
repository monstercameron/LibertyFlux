// original: 0x00b8da20 SET_TEXT_RIGHT_JUSTIFY
// rw_set_text_right_justify: native SET_TEXT_RIGHT_JUSTIFY (handler 0x00B8DA20).
//
// Forwards one bool coerced from nonzero to 1 (low byte of the dead incoming-arg slot reused as the temp).
// NOTE: the original coerces the bool with `setne` into the low byte
// of its own dead incoming-argument stack slot and pushes that whole
// dword to the engine; the pushed value (high bytes = context address)
// is reproduced exactly, while the dead-slot store itself is not, so
// this contract checks `calls` but not `stack` (see report).
export!(cdecl, rw_set_text_right_justify(ctx: *mut u32) -> u32 {
    unsafe {
        let args = *(ctx.add(2) as *mut *const u32);
        let flag0 = ((*args.add(0) != 0) as u32) | ((ctx as u32) & !0xFF);
        let ans = callee_cdecl!(1, u32, flag0);
        ans
    }
});
