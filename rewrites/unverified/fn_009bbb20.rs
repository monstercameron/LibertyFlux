// original: 0x009bbb20 input_flag_is_set (proposed)

/// Test whether the linked input record carries its flag byte.
///
/// `this` points to a holder whose first word is a possibly-null pointer to
/// the record. Returns 1 when the record exists and its byte at `+0x61` is
/// non-zero, else 0. Only the low byte of the result is set.
///
/// Edge cases: a null link returns 0 without dereferencing it; any
/// non-zero flag value counts as set.
///
/// Original: thiscall, `this` in ECX, no stack arguments, `al` result.
lf_checker_rt::export!(thiscall, rw_009bbb20(this: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0x61;
        let rec = (this as *const u32).read_unaligned();
        if rec == 0 {
            return 0;
        }
        if ((rec + FLAG_OFF) as *const u8).read() == 0 {
            return 0;
        }
        1
    }
});
