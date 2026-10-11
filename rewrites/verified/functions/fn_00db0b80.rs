// original: 0x00DB0B80 mainloop_timing_initialize_triple
/// Initializes a three-word timing value. The `thiscall` receiver points to
/// the destination record, the stack argument is an `f32`, and the function
/// stores that value's exact bits followed by two zero words. It returns the
/// same destination pointer in EAX. No global state or callees are involved.
lf_checker_rt::export!(thiscall, rw_00DB0B80(destination: *mut u32, value: f32) -> *mut u32 {
    const FIRST_VALUE_WORD: usize = 0;
    const FIRST_ZERO_WORD: usize = 1;
    const SECOND_ZERO_WORD: usize = 2;

    let value_bits = value.to_bits();
    // SAFETY: the caller provides a writable record of at least three words.
    unsafe {
        destination.add(FIRST_VALUE_WORD).write(value_bits);
        destination.add(FIRST_ZERO_WORD).write(0);
        destination.add(SECOND_ZERO_WORD).write(0);
    }
    destination
});
