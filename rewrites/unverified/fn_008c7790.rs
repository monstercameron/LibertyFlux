// original: 0x008C7790 stream_opt_store_a
/// Publish one streaming option set: handle, mode byte and a cleared slot.
///
/// Stores `handle` to the option handle global, the low byte of `mode` to
/// the mode global, raises the options-present flag and clears the pending
/// slot. Returns nothing. Original: cdecl, two stack words.
lf_checker_rt::export!(cdecl, rw_008c7790(handle: u32, mode: u32) -> u32 {
    unsafe {
        *lf_checker_rt::global::<u32>(0x1173220) = handle;
        *lf_checker_rt::global::<u8>(0x1031ba6) = 1;
        *lf_checker_rt::global::<u8>(0x1172f52) = mode as u8;
        *lf_checker_rt::global::<u32>(0x1173224) = 0;
        0
    }
});
