// original: 0x00D68C60 store_sign_extended_byte

/// Stores the signed low byte of `value` as a 16-bit value at `destination` and returns the destination pointer.
lf_checker_rt::export!(cdecl, rw_00d68c60(destination: u32, value: u32) -> u32 {
    unsafe {
        let signed = value as u8 as i8 as i16 as u16;
        (destination as *mut u16).write_unaligned(signed);
        destination
    }
});
