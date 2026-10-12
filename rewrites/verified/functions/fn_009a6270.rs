// original: 0x009a6270 script_sound_triple_find
/// Finds a live script-audio request with three matching identifiers.
///
/// The request table has 32 rows with a 20-byte stride. A row whose first
/// word is the unused sentinel ends the search. For a live row, its third
/// word is compared with the second cdecl argument, its first word with the
/// third argument, and its second word with the first argument. The result
/// is returned in AL.
const REQUEST_TABLE_VA: u32 = 0x0128_47D8;
const REQUEST_TABLE_END_VA: u32 = 0x0128_4A58;
const REQUEST_STRIDE: u32 = 0x14;
const REQUEST_ID: u32 = 0;
const REQUEST_FLAGS: u32 = 4;
const REQUEST_OWNER: u32 = 8;
const UNUSED_REQUEST: u32 = u32::MAX;

export!(stdcall, rw_009a6270(first: u32, second: u32, third: u32) -> u8 {
    let mut row = lf_checker_rt::relocated(REQUEST_TABLE_VA);
    let table_end = lf_checker_rt::relocated(REQUEST_TABLE_END_VA);
    while row < table_end {
        let stored_owner = unsafe {
            (row.wrapping_add(REQUEST_OWNER) as *const u32).read()
        };
        let owner_matches = stored_owner == second;
        let request_matches = owner_matches && unsafe {
            (row.wrapping_add(REQUEST_ID) as *const u32).read() == third
        };
        let flags_match = request_matches && unsafe {
            (row.wrapping_add(REQUEST_FLAGS) as *const u32).read() == first
        };
        if flags_match {
            return 1;
        }
        if unsafe { (row.wrapping_add(REQUEST_ID) as *const u32).read() } == UNUSED_REQUEST {
            break;
        }
        row = row.wrapping_add(REQUEST_STRIDE);
    }
    0
});
