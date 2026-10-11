// original: 0X00B4C2B0 nested_signed_byte_at_4e6

/// Read the signed byte at nested-record offset 0x4E6. The outer record's
/// pointer field is at byte offset 0x228; the outer pointer arrives as a
/// stack argument. The result is sign-extended to EAX.
lf_checker_rt::export!(cdecl, rw_00B4C2B0(outer: u32) -> u32 {
    const NESTED_PTR: u32 = 0x228;
    const SIGNED_VALUE: u32 = 0x4E6;
    // SAFETY: the outer object points to a nested record covering this byte.
    let nested = unsafe { (outer.wrapping_add(NESTED_PTR) as *const u32).read_unaligned() };
    // SAFETY: the contract supplies the nested record and its signed byte.
    let value = unsafe { (nested.wrapping_add(SIGNED_VALUE) as *const i8).read() };
    i32::from(value) as u32
});
