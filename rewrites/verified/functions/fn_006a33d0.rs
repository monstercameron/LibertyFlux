// original: 0x006A33D0 clear_pending_stream_flag
/// Returns whether a pending stream flag was set and clears it when set.
///
/// The flag is a single byte in the image's shared state. A zero byte leaves
/// memory untouched and returns zero; any nonzero byte is cleared and returns
/// one in AL. The global helper derives the relocated pointer from the mapped
/// image base so the implementation does not depend on a fixed load address.
const PENDING_STREAM_FLAG: u32 = 0x019F_13EF;

lf_checker_rt::export!(cdecl, rw_006a33d0() -> u8 {
    let flag = unsafe {
        core::ptr::read_volatile(lf_checker_rt::global::<u8>(PENDING_STREAM_FLAG))
    };
    if flag == 0 {
        0
    } else {
        unsafe {
            core::ptr::write_volatile(
                lf_checker_rt::global::<u8>(PENDING_STREAM_FLAG),
                0,
            );
        }
        1
    }
});
