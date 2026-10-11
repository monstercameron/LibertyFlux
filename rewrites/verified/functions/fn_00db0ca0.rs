// original: 0x00DB0CA0 mainloop_timing_cached_value_query
/// Returns the cached value at byte offset eight when it is nonzero. For a
/// zero cache word, it calls the cdecl query helper with the word at byte
/// offset four, stores the helper's EAX result as the new cache value, and
/// returns it. The equality test is unsigned-neutral; zero is the sentinel.
lf_checker_rt::export!(thiscall, rw_00DB0CA0(state: *mut u32) -> u32 {
    const QUERY_ARGUMENT_WORD: usize = 1;
    const CACHED_VALUE_WORD: usize = 2;

    // SAFETY: the receiver addresses a record with at least three words.
    let cached_value = unsafe { state.add(CACHED_VALUE_WORD).read() };
    if cached_value != 0 {
        return cached_value;
    }

    // SAFETY: the argument word is part of the same receiver record.
    let query_argument = unsafe { state.add(QUERY_ARGUMENT_WORD).read() };
    let queried_value = lf_checker_rt::callee_cdecl!(1, u32, query_argument);
    // SAFETY: the cache word is writable within the receiver record.
    unsafe { state.add(CACHED_VALUE_WORD).write(queried_value); }
    queried_value
});
