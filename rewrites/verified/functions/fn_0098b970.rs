// original: 0x0098b970 audio_handle_release
/// Releases one reference to a script-audio handle.
///
/// The cdecl argument selects a 16-byte handle record. A nonzero reference
/// count is decremented and the current global time is stored beside it. After
/// that conditional update, the live-handle count is decremented whenever the
/// record's resulting count is zero, including when the initial count was
/// already zero. All decrements wrap as 32-bit unsigned arithmetic.
const HANDLE_TABLE_VA: u32 = 0x0128_32C0;
const HANDLE_COUNT_OFFSET: u32 = 0;
const HANDLE_TIME_OFFSET: u32 = 4;
const HANDLE_STRIDE: u32 = 0x10;
const CURRENT_TIME_VA: u32 = 0x0117_35B4;
const LIVE_HANDLE_COUNT_VA: u32 = 0x0128_32A0;

export!(cdecl, rw_0098b970(handle_index: u32) -> () {
    let record = lf_checker_rt::relocated(HANDLE_TABLE_VA)
        .wrapping_add(handle_index.wrapping_mul(HANDLE_STRIDE));
    let reference_count = unsafe {
        (record.wrapping_add(HANDLE_COUNT_OFFSET) as *const u32).read()
    };
    if reference_count != 0 {
        let new_reference_count = reference_count.wrapping_sub(1);
        unsafe {
            (record.wrapping_add(HANDLE_COUNT_OFFSET) as *mut u32)
                .write(new_reference_count);
            let current_time = lf_checker_rt::global::<u32>(CURRENT_TIME_VA).read();
            (record.wrapping_add(HANDLE_TIME_OFFSET) as *mut u32).write(current_time);
        }
    }
    let resulting_count = unsafe {
        (record.wrapping_add(HANDLE_COUNT_OFFSET) as *const u32).read()
    };
    if resulting_count == 0 {
        unsafe {
            let live_count = lf_checker_rt::global::<u32>(LIVE_HANDLE_COUNT_VA).read();
            lf_checker_rt::global::<u32>(LIVE_HANDLE_COUNT_VA)
                .write(live_count.wrapping_sub(1));
        }
    }
});
