// original: 0x006A33F0 reset_stream_state_words
/// Clears the two shared stream state words and returns no value.
///
/// Each write targets its own four-byte global slot through the runtime's
/// relocated-image accessor. The function does not read its incoming
/// registers or stack and makes no outgoing calls.
const FIRST_STREAM_STATE: u32 = 0x018D_D968;
const SECOND_STREAM_STATE: u32 = 0x018D_DD78;

lf_checker_rt::export!(cdecl, rw_006a33f0() -> () {
    unsafe {
        core::ptr::write_volatile(lf_checker_rt::global::<u32>(FIRST_STREAM_STATE), 0);
        core::ptr::write_volatile(lf_checker_rt::global::<u32>(SECOND_STREAM_STATE), 0);
    }
});
