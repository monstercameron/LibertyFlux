// original: 0x006703F0 rage::fiTokenizer::vf33

/// Format the supplied value fields into a local buffer, then write that
/// buffer to the tokenizer's stream. The tokenizer write state at `+0x14`
/// is reset first. The format string and all value arguments are passed to
/// the formatter in their original order; the returned byte count is
/// compared with the stream writer's count and the result is a boolean in AL.
/// The output buffer is 64 bytes and the original format strings are image
/// relative references. This function uses the tokenizer stream pointer at
/// `+0x0C`.
///
/// Original: 0x006703F0 (thiscall, 3 stack argument(s), 2 calls).
lf_checker_rt::export!(thiscall, rw_006703f0(this: u32, value0: u32, value1: u32, value2: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0x14;
        const STREAM: u32 = 0x0C;
        const FORMAT_RVA: u32 = 0xB9EA90;
        const FORMAT_CALLEE: u32 = 1;
        const WRITE_CALLEE: u32 = 2;
        #[inline(always)]
        unsafe fn rd32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        ((this + STATE) as *mut u32).write_unaligned(0);
        let format = lf_checker_rt::relocated(FORMAT_RVA);
        let mut buf = [0u32; 16];
        let length = lf_checker_rt::callee_cdecl!(FORMAT_CALLEE, u32, buf.as_mut_ptr() as u32, format, (f64::from(f32::from_bits(value0: u32)).to_bits() as u32), ((f64::from(f32::from_bits(value0: u32)).to_bits() >> 32) as u32), (f64::from(f32::from_bits(value1: u32)).to_bits() as u32), ((f64::from(f32::from_bits(value1: u32)).to_bits() >> 32) as u32), (f64::from(f32::from_bits(value2: u32)).to_bits() as u32), ((f64::from(f32::from_bits(value2: u32)).to_bits() >> 32) as u32));
        let stream = rd32(this + STREAM);
        let written = lf_checker_rt::callee_thiscall!(WRITE_CALLEE, u32, stream, buf.as_mut_ptr() as u32, length);
        (written == length) as u32
    }
});
