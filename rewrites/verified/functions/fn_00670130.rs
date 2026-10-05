// original: 0x00670130 rage::fiTokenizer::vf30

/// End the current line unless it is already ended.
///
/// `this` points to the tokenizer. When the writer state at `+0x14` is not
/// 2, a carriage return and a line feed are put through the character callee
/// with the stream object at `+0x0C`. The state becomes 2 either way. The
/// return register is untouched, so the contract does not compare it.
///
/// Original: 0x00670130 (thiscall, no stack arguments, up to 2 calls).
lf_checker_rt::export!(thiscall, rw_00670130(this: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0x14;
        const STREAM: u32 = 0x0c;
        const PUTC_CALLEE: u32 = 1;

        if ((this + STATE) as *const u32).read_unaligned() != 2 {
            let stream = ((this + STREAM) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(PUTC_CALLEE, u32, stream, 0x0d);
            lf_checker_rt::callee_thiscall!(PUTC_CALLEE, u32, stream, 0x0a);
        }
        ((this + STATE) as *mut u32).write_unaligned(2);
        0
    }
});
