// original: 0x00888B70 stream_insert_word (proposed)

/// Insert a stream word through the table insert entry.
///
/// Passes the address of its own incoming argument slot plus the argument
/// plus 0x42 to the table insert entry (callee 1) with the table constant;
/// its answer is the answer of this function.
///
/// Original: 0x00888B70 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00888B70(arg: u32) -> u32 {
    unsafe {
        const TABLE_FILE_VA: u32 = 0x0115_a514;
        const KEY_BIAS: u32 = 0x42;
        const INSERT: u32 = 1;
        let table = lf_checker_rt::relocated(TABLE_FILE_VA);
        // The original passes the address of its own argument slot; pass a
        // local holding the same value. The contract skips the pointer and
        // snapshots the word it points to at call time.
        let slot = arg;
        lf_checker_rt::callee_thiscall!(INSERT, u32, table,
            arg.wrapping_add(KEY_BIAS), &slot as *const u32 as u32)
    }
});
