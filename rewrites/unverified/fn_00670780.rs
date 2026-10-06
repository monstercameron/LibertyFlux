// original: 0x00670780 rage::fiTokenizer::ctor

/// Construct a tokenizer over a caller-supplied stream.
///
/// `this` points to the tokenizer, `first` and `stream` are stored at `+0x04`
/// and `+0x0c` (the stream object the sibling methods read through). The
/// tokenizer table is installed, the line number at `+0x08` starts at 1, the
/// read mode at `+0x14` starts at 2 with its limit at `+0x10` set to 0x20, and
/// the pushback count at `+0x18`, its first cell at `+0x1c`, the auxiliary
/// word at `+0x21c` and the indent level at `+0x220` are cleared. Returns
/// `this`. No calls, no globals.
///
/// Original: 0x00670780 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00670780(this: u32, first: u32, stream: u32) -> u32 {
    unsafe {
        const VTABLE_TOKENIZER: u32 = 0x00fe_36b4;
        const FIRST: u32 = 0x04;
        const LINE: u32 = 0x08;
        const STREAM: u32 = 0x0c;
        const LIMIT: u32 = 0x10;
        const MODE: u32 = 0x14;
        const PUSHBACK_COUNT: u32 = 0x18;
        const PUSHBACK_BUF: u32 = 0x1c;
        const AUX: u32 = 0x21c;
        const LEVEL: u32 = 0x220;
        const FIRST_LINE: u32 = 1;
        const LIMIT_INIT: u32 = 0x20;
        const MODE_INIT: u32 = 2;

        ((this + FIRST) as *mut u32).write_unaligned(first);
        ((this + STREAM) as *mut u32).write_unaligned(stream);
        ((this + LEVEL) as *mut u32).write_unaligned(0);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_TOKENIZER));
        ((this + LINE) as *mut u32).write_unaligned(FIRST_LINE);
        ((this + LIMIT) as *mut u32).write_unaligned(LIMIT_INIT);
        ((this + PUSHBACK_COUNT) as *mut u32).write_unaligned(0);
        ((this + PUSHBACK_BUF) as *mut u8).write(0);
        ((this + AUX) as *mut u32).write_unaligned(0);
        ((this + MODE) as *mut u32).write_unaligned(MODE_INIT);
        this
    }
});
