// original: 0x00b02d10 clear_word_pair
/// Zero a pair of adjacent state words. No inputs, no return.
export!(cdecl, rw_00b02d10() -> u32 {
    unsafe {
        *global::<u32>(0x16010a4) = 0;
        *global::<u32>(0x16010a8) = 0;
    }
    0 // unchecked: contract compares no return channel
});
