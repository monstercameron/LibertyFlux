// original: 0x00670380 rage::fiTokenizer::vf34

/// Format the supplied value fields into a local buffer, then write that
/// buffer to the tokenizer's stream. The tokenizer write state at `+0x14`
/// is reset first. The format string and all value arguments are passed to
/// the formatter in their original order; the returned byte count is
/// compared with the stream writer's count and the result is a boolean in AL.
/// The output buffer is 64 bytes; the format references are relocated from
/// their executable virtual addresses. This function uses the stream pointer at
/// `+0x0C`.
///
/// Original: 0x00670380 (thiscall, one stack pointer to 2 binary32 values, 2 calls).
lf_checker_rt::export!(thiscall, rw_00670380(this: u32, values: u32) -> u32 {
    unsafe {
        let value0 = (values as *const u32).add(0).read_unaligned();
        let value1 = (values as *const u32).add(1).read_unaligned();
        const STATE: u32 = 0x14;
        const STREAM: u32 = 0x0C;
        const FORMAT_FILE_VA: u32 = 0xF9EA6C;
        const FORMAT_CALLEE: u32 = 1;
        const WRITE_CALLEE: u32 = 2;
        #[inline(always)]
        unsafe fn rd32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        ((this + STATE) as *mut u32).write_unaligned(0);
        let format = lf_checker_rt::relocated(FORMAT_FILE_VA);
        let mut buf = [0u32; 16];
        let length = lf_checker_rt::callee_cdecl!(FORMAT_CALLEE, u32, buf.as_mut_ptr() as u32, format, (f64::from(f32::from_bits(value0)).to_bits() as u32), ((f64::from(f32::from_bits(value0)).to_bits() >> 32) as u32), (f64::from(f32::from_bits(value1)).to_bits() as u32), ((f64::from(f32::from_bits(value1)).to_bits() >> 32) as u32));
        let stream = rd32(this + STREAM);
        let written = lf_checker_rt::callee_thiscall!(WRITE_CALLEE, u32, stream, buf.as_mut_ptr() as u32, length);
        (written == length) as u32
    }
});
