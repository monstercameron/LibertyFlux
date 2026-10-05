// original: 0x00c3d9f0 train_free_slot_table (proposed)
/// Free every non-null pointer in a thirteen-slot global table.
///
/// Walks the thirteen dwords of the table, calling deallocator id 1
/// (cdecl, pointer) on each non-null entry, and clears every slot to
/// null. Returns nothing meaningful.
///
/// Original: 0x00c3d9f0 (cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_00c3d9f0() -> u32 {
    unsafe {
        const TABLE: u32 = 0x16d11c8;
        const SLOTS: u32 = 13;
        const FREE: u32 = 1;
        let mut p = lf_checker_rt::relocated(TABLE);
        let mut i = 0u32;
        while i < SLOTS {
            let e = (p as *const u32).read_unaligned();
            if e != 0 {
                let _: u32 = lf_checker_rt::callee_cdecl!(FREE, u32, e);
            }
            (p as *mut u32).write_unaligned(0);
            p = p.wrapping_add(4);
            i += 1;
        }
        0
    }
});
