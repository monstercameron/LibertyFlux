// original: 0x008C77C0 stream_opt_store_b
/// Publish the second streaming option set and clear its pending slot.
///
/// Stores `first` and `second` to their globals and clears the pending
/// slot. Returns nothing. Original: cdecl, two stack words.
lf_checker_rt::export!(cdecl, rw_008c77c0(first: u32, second: u32) -> u32 {
    unsafe {
        *lf_checker_rt::global::<u32>(0x1031bb0) = first;
        *lf_checker_rt::global::<u32>(0x1173228) = second;
        *lf_checker_rt::global::<u32>(0x117322c) = 0;
        0
    }
});
