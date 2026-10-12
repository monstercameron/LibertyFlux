// original: 0x0086F250 POW
//! Reads two single precision operands from the script context, calls the
//! engine power routine in that order, and stores its returned float bits.

lf_checker_rt::export!(cdecl, rw_fn_0086f250(context: u32) -> () {
    let context_words = context as *const u32;
    let output = unsafe { core::ptr::read_unaligned(context_words) };
    let inputs = unsafe { core::ptr::read_unaligned(context_words.add(2)) };
    let input_words = inputs as *const u32;
    let base = unsafe { core::ptr::read_unaligned(input_words) };
    let exponent = unsafe { core::ptr::read_unaligned(input_words.add(1)) };
    let result = lf_checker_rt::callee_cdecl!(1, u32, base, exponent);
    unsafe { core::ptr::write_unaligned(output as *mut u32, result) };
});
