// original: 0x00AF6390 lookup_value_in_60_word_table_or_fallback

/// Find a value in the 60-word global lookup table and return its zero-based
/// index, or the supplied fallback when no entry matches. The table is read
/// from relocated image data; the function does not modify memory.
///
/// The two `u32` arguments are ordinary values on the cdecl stack. No caller
/// or helper behavior is part of this routine, and comparisons are exact
/// unsigned word equality.
lf_checker_rt::export!(cdecl, rw_00AF6390(needle: u32, fallback: u32) -> u32 {
    const TABLE_VA: u32 = 0x015F_CB80;
    const ENTRY_COUNT: u32 = 60;

    unsafe {
        let table = lf_checker_rt::global::<u32>(TABLE_VA);
        let mut index = 0u32;
        while index < ENTRY_COUNT {
            if table.add(index as usize).read_volatile() == needle {
                return index;
            }
            index += 1;
        }
        fallback
    }
});
