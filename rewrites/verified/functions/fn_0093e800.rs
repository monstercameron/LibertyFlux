// original: 0x0093e800 stream_lists_reset (proposed)

/// Clear the back-pointers of every object in the streaming node lists.
///
/// First pass: for each 12-byte slot from `TABLE_A` up to (not including)
/// `TABLE_A_END`, treats words +0 and +4 as list heads and walks each
/// chain through +4, zeroing the word at +0x3C of the object each node
/// points at. Second pass: for each 20-byte slot from `TABLE_B` up to
/// `TABLE_B_END`, does the same for the heads at +0, +4 and +8.
///
/// Original: 0x0093e800 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_0093e800() -> u32 {
    const TABLE_A: u32 = 0x11A9D20;
    const TABLE_A_END: u32 = 0x11D4020;
    const SLOT_A: u32 = 0x0C;
    const TABLE_B: u32 = 0x11A8918;
    const TABLE_B_END: u32 = 0x11A9D18;
    const SLOT_B: u32 = 0x14;
    const OBJ_BACK: u32 = 0x3C;
    unsafe fn clear_chain(mut node: u32) {
        while node != 0 {
            let obj = (node as *const u32).read_unaligned();
            ((obj + OBJ_BACK) as *mut u32).write_unaligned(0);
            node = ((node + 4) as *const u32).read_unaligned();
        }
    }
    unsafe {
        let mut slot = lf_checker_rt::relocated(TABLE_A);
        let end = lf_checker_rt::relocated(TABLE_A_END);
        while slot < end {
            clear_chain((slot as *const u32).read_unaligned());
            clear_chain(((slot + 4) as *const u32).read_unaligned());
            slot += SLOT_A;
        }
        let mut slot = lf_checker_rt::relocated(TABLE_B);
        let end = lf_checker_rt::relocated(TABLE_B_END);
        while slot < end {
            clear_chain((slot as *const u32).read_unaligned());
            clear_chain(((slot + 4) as *const u32).read_unaligned());
            clear_chain(((slot + 8) as *const u32).read_unaligned());
            slot += SLOT_B;
        }
        0
    }
});
