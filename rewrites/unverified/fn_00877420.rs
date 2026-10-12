// original: 0x00877420 rage::crmfAngle::vf1

/// Copy the scalar payload from another angle object.
///
/// The source and destination are object pointers. The payload is one raw
/// 32-bit word at offset `+0x04`; copying the word preserves every integer
/// and floating-point bit pattern. The copied word is also returned in EAX.
///
/// Original calling convention: thiscall with one stack pointer argument.
lf_checker_rt::export!(thiscall, rw_00877420(this: u32, source: u32) -> u32 {
    const VALUE_OFFSET: u32 = 0x04;
    unsafe {
        let value = (source.wrapping_add(VALUE_OFFSET) as *const u32).read_unaligned();
        (this.wrapping_add(VALUE_OFFSET) as *mut u32).write_unaligned(value);
        value
    }
});
