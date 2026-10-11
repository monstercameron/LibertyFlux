// original: 0x00AE5EC0 streaming_copy_scanner_origin

/// Copy three global single-precision bit patterns to the caller's buffer.
///
/// The cdecl output pointer receives the exact 32-bit words from three
/// consecutive global values. The function performs no floating-point
/// arithmetic, so signed zero, infinities, and NaN payloads are preserved.
/// EAX returns the same pointer value supplied by the caller.
export!(cdecl, rw_00ae5ec0(output: *mut u32) -> u32 {
    unsafe {
        const FIRST_VALUE_FILE_VA: u32 = 0x015c3bc0;
        const SECOND_VALUE_FILE_VA: u32 = 0x015c3bc4;
        const THIRD_VALUE_FILE_VA: u32 = 0x015c3bc8;

        let first_bits = core::ptr::read_unaligned(global::<u32>(FIRST_VALUE_FILE_VA));
        let second_bits = core::ptr::read_unaligned(global::<u32>(SECOND_VALUE_FILE_VA));
        let third_bits = core::ptr::read_unaligned(global::<u32>(THIRD_VALUE_FILE_VA));
        core::ptr::write_unaligned(output, first_bits);
        core::ptr::write_unaligned(output.add(1), second_bits);
        core::ptr::write_unaligned(output.add(2), third_bits);
        output as u32
    }
});
