// original: 0x009a5800 script_audio_requests_reset
/// Resets the request metadata in the 32-row script audio request table.
///
/// Each row is twenty bytes wide. The first three words become the unused
/// sentinel, zero, and zero; later row fields are preserved. The method
/// returns the address immediately after the table.
const TABLE_VA: u32 = 0x0128_47D8;
const TABLE_END_VA: u32 = 0x0128_4A58;
const ROW_BYTES: u32 = 0x14;
const REQUEST_ID: u32 = 0;
const REQUEST_FLAGS: u32 = 4;
const REQUEST_OWNER: u32 = 8;

export!(cdecl, rw_009a5800() -> u32 {
    let mut row = lf_checker_rt::relocated(TABLE_VA);
    let table_end = lf_checker_rt::relocated(TABLE_END_VA);
    while row < table_end {
        unsafe {
            (row.wrapping_add(REQUEST_ID) as *mut u32).write(u32::MAX);
            (row.wrapping_add(REQUEST_FLAGS) as *mut u32).write(0);
            (row.wrapping_add(REQUEST_OWNER) as *mut u32).write(0);
        }
        row = row.wrapping_add(ROW_BYTES);
    }
    row
});
