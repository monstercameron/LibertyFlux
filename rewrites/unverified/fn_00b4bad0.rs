// original: 0x00B4BAD0 object_field_10_not_200

/// Return one when the object's 32-bit field at byte offset 0x10 is not 200.
/// The object arrives in ECX; the result is zero-extended from the low byte.
lf_checker_rt::export!(thiscall, rw_00B4BAD0(object: u32) -> u32 {
    const FIELD_VALUE: u32 = 0x10;
    const EMPTY_SENTINEL: u32 = 200;

    // SAFETY: the checker contract supplies a readable object through ECX.
    let field = unsafe { ((object + FIELD_VALUE) as *const u32).read_unaligned() };
    u32::from(field != EMPTY_SENTINEL)
});
