// original: 0x00DB6CA0 input_control_state_initialize

/// Initialize a control-state object with its caller-supplied source word,
/// then clear the value-valid byte and the remaining event fields at +0x04,
/// +0x0C, +0x10 and +0x18. The source word remains at offset zero and is also
/// the value left in EAX; thiscall passes it as one stack argument.
lf_checker_rt::export!(thiscall, rw_00db6ca0(this: u32, source_word: u32) -> u32 {
    unsafe {
        const SOURCE_WORD: u32 = 0;
        const VALUE: u32 = 0x04;
        const VALUE_VALID: u32 = 0x0c;
        const EVENT_CODE: u32 = 0x10;
        const EVENT_PENDING: u32 = 0x18;

        ((this + SOURCE_WORD) as *mut u32).write_unaligned(source_word);
        ((this + VALUE) as *mut u32).write_unaligned(0);
        ((this + VALUE_VALID) as *mut u8).write(0);
        ((this + EVENT_CODE) as *mut u32).write_unaligned(0);
        ((this + EVENT_PENDING) as *mut u8).write(0);
    }
    source_word
});
