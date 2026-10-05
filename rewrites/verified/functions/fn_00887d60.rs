// original: 0x00887D60 stream_chain_find (proposed)

/// Find a tagged node in a hashed chain table and return its payload.
///
/// The low word of the argument picks bucket `word % count` (count from a
/// global; a zero count finds nothing) of the table from a global; the
/// chain (link at node `+8`) is walked for the first node whose word at
/// `+0` equals the argument word. A match returns the word at node `+4`;
/// an empty bucket or a full walk without a match returns 0.
///
/// Original: 0x00887D60 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00887D60(arg: u32) -> u32 {
    unsafe {
        const COUNT_GLOBAL: u32 = 0x0115_a518;
        const TABLE_GLOBAL: u32 = 0x0115_a514;
        const TAG: u32 = 0x00;
        const PAYLOAD: u32 = 0x04;
        const NEXT: u32 = 0x08;
        let count =
            (lf_checker_rt::relocated(COUNT_GLOBAL) as *const u16).read_unaligned()
                as u32;
        if count == 0 {
            return 0;
        }
        let word = arg as u16 as u32;
        let table =
            (lf_checker_rt::relocated(TABLE_GLOBAL) as *const u32).read_unaligned();
        let mut e = ((table.wrapping_add((word % count).wrapping_mul(4)))
            as *const u32)
            .read_unaligned();
        loop {
            if e == 0 {
                return 0;
            }
            let tag = ((e + TAG) as *const u16).read_unaligned() as u32;
            if tag == word {
                let p = e.wrapping_add(PAYLOAD);
                if p == 0 {
                    return 0;
                }
                return (p as *const u32).read_unaligned();
            }
            e = ((e + NEXT) as *const u32).read_unaligned();
        }
    }
});
