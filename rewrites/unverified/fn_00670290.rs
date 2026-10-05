// original: 0X00670290 rage::fiTokenizer::vf40

/// Format an integer into the stream, reporting full success.
///
/// `this` points to the tokenizer and `value` is the integer. The writer
/// state at `+0x14` becomes 0, the format callee renders the value into a
/// 0x40-byte frame buffer (its format string lives at an unrelocated game
/// address, so the contract skips that argument and the value runs on a small
/// cycle with true outputs), then the frame bytes go through the write callee
/// with the stream object at `+0x0C`. Only the low byte of the return value
/// is meaningful: 1 when the write callee's answer equals the formatted
/// length, else 0.
///
/// Original: 0X00670290 (thiscall, one stack argument, 2 calls).
lf_checker_rt::export!(thiscall, rw_00670290(this: u32, value: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0x14;
        const STREAM: u32 = 0x0c;
        const FORMAT_CALLEE: u32 = 1;
        const WRITE_CALLEE: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        ((this + STATE) as *mut u32).write_unaligned(0);
        let mut buf = [0u32; 16];
        // NOTE: the format-string argument is skipped by the contract (it is
        // an unrelocated game address); any value keeps the call shape.
        let len = lf_checker_rt::callee_cdecl!(FORMAT_CALLEE, u32,
                                               buf.as_mut_ptr() as u32, 0, value);
        let stream = rd32(this + STREAM);
        let wrote = lf_checker_rt::callee_thiscall!(WRITE_CALLEE, u32, stream,
                                                    buf.as_mut_ptr() as u32, len);
        (wrote == len) as u32
    }
});
