// original: 0x00986C70 audStaticRadioEmitter::vf11

/// Static radio emitter virtual slot 11: stores two dwords into the object.
///
/// Writes `first` at `+FIRST_OFF` and `second` at `+SECOND_OFF` of `this`.
/// No meaningful return value.
/// Original: thiscall, two stack words, callee pops 8.
lf_checker_rt::export!(thiscall, rw_00986C70(this: u32, first: u32, second: u32) -> u32 {
    const FIRST_OFF: u32 = 0x38;
    const SECOND_OFF: u32 = 0x3c;
    unsafe {
        ((this + FIRST_OFF) as *mut u32).write_unaligned(first);
        ((this + SECOND_OFF) as *mut u32).write_unaligned(second);
    }
    0
});
