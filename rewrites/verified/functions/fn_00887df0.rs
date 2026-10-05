// original: 0x00887DF0 stream_lookup_word (proposed)

/// Look up a stream word by a key held in the caller's argument slot.
///
/// Passes the address of its own incoming argument slot to the table
/// lookup entry (callee 1) with the table constant; a hit returns the
/// word the hit points to, a miss returns 0.
///
/// Original: 0x00887DF0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00887DF0(arg: u32) -> u32 {
    unsafe {
        const TABLE_FILE_VA: u32 = 0x0115_a4c8;
        const LOOKUP: u32 = 1;
        let table = lf_checker_rt::relocated(TABLE_FILE_VA);
        // The original passes the address of its own argument slot; pass a
        // local holding the same value. The contract skips the pointer and
        // snapshots the word it points to at call time.
        let slot = arg;
        let ans = lf_checker_rt::callee_thiscall!(
            LOOKUP, u32, table, &slot as *const u32 as u32);
        if ans == 0 {
            0
        } else {
            (ans as *const u32).read_unaligned()
        }
    }
});
