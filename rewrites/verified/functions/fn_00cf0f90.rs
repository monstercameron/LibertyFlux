// original: 0x00CF0F90 stat_ms_to_millis

/// Return the signed millisecond remainder of a signed elapsed-time word.
///
/// The single cdecl argument is interpreted as an `i32`. The original uses
/// signed division by one thousand and subtracts the quotient times one
/// thousand, so negative values retain Rust's signed remainder convention
/// (the remainder has the dividend's sign). No memory is read or written.
lf_checker_rt::export!(cdecl, rw_fn_00cf0f90(elapsed_milliseconds: i32) -> i32 {
    const MILLIS_PER_SECOND: i32 = 1_000;
    elapsed_milliseconds % MILLIS_PER_SECOND
});
