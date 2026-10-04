// original: 0x00BD3F20 ENABLE_SHADOWS
// ENABLE_SHADOWS: coerces the script word to a bool and forwards it. The
// original builds the argument in its own dead incoming-argument slot, so
// the pushed value keeps the context pointer's high bytes with the low byte
// replaced by the bool; that shape is reproduced exactly. No return.
export!(cdecl, rw_fn_bd3f20(ctx: *mut u8) -> () {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let arg = ((*args != 0) as u32);
        let _ans: u32 = callee_cdecl!(1, u32, arg);
    }
});
