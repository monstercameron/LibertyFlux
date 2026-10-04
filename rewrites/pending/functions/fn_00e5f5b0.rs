// original: 0x00E5F5B0 timing_entry_init_10f0f4
// Initialize the 16-byte timing entry at 0x0110F0F4.
//
// Stores the first callback pointer, a zero word, and the second
// callback pointer. The entry's second word is padding that the
// original fills by copying uninitialized stack (a two-register
// block store over a half-written temporary); it is reproduced
// here as 0, matching the contract's defined stack fill.
// The original leaves EAX untouched, so nothing is returned.
lf_checker_rt::export!(cdecl, rw_00e5f5b0() -> u32 {
    unsafe {
        let e = lf_checker_rt::global::<u32>(0x0110F0F4);
        *e = lf_checker_rt::relocated(0x0043EA90);
        *e.add(1) = 0;
        *e.add(2) = 0;
        *e.add(3) = lf_checker_rt::relocated(0x00409610);
        0
    }
});
