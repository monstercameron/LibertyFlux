// original: 0x00A7F4A0 CDieInfo::vf6

/// Serialize the dword at `this + 0x14` through the output stream with a
/// fixed 32-bit mode value, then serialize the inherited task-info base.
/// The first helper also receives a zero word from the method's local stack
/// slot. The function returns the bitwise OR of the low bytes returned by the
/// two helpers. This is a 32-bit thiscall method with one stack argument.
lf_checker_rt::export!(thiscall, rw_00a7f4a0(this: u32, stream: u32) -> u32 {
    unsafe {
        const SERIALIZED_VALUE: u32 = 0x14;
        const WRITE_VALUE: u32 = 1;
        const WRITE_BASE: u32 = 2;
        const MODE: u32 = 0x20;
        const EXTRA_ZERO: u32 = 0;

        let value = (this.wrapping_add(SERIALIZED_VALUE) as *const u32).read_unaligned();
        let value_result = lf_checker_rt::callee_thiscall!(
            WRITE_VALUE,
            u32,
            stream,
            value,
            MODE,
            EXTRA_ZERO
        );
        let base_result = lf_checker_rt::callee_thiscall!(WRITE_BASE, u32, this, stream);
        (value_result & 0xff) | (base_result & 0xff)
    }
});
