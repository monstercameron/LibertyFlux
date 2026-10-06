// original: 0x00951250 sweep_and_free_entries (proposed)

/// Sweep the 1024-entry table, freeing live entries and counting them.
///
/// Each 0x14-byte entry at `TABLE` holds a handle at +0, a dead flag at
/// +16 and a live flag at +17. An entry with a null handle, a zero live
/// flag or a non-zero dead flag is skipped. Otherwise, when the LOW byte
/// of `f` is non-zero the handle is freed through callee 1 (one stack
/// word) and counted; then the entry is cleared (dword +0, word +4 = -1,
/// dwords +8 and +12, word +16 covering both flags). Returns the count.
///
/// Original: 0x00951250 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00951250(f: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x120F2B8;
        const SLOTS: u32 = 1024;
        const STRIDE: u32 = 0x14;
        const FREE: u32 = 1;
        const CLEARED_TAG: u16 = 0xFFFF;
        let mut count = 0u32;
        let mut i = 0u32;
        while i < SLOTS {
            let e = lf_checker_rt::relocated(TABLE).wrapping_add(i.wrapping_mul(STRIDE));
            let h = (e as *const u32).read();
            if h != 0 {
                let live = (e.wrapping_add(17) as *const u8).read();
                if live != 0 {
                    let dead = (e.wrapping_add(16) as *const u8).read();
                    if dead == 0 {
                        if (f & 0xFF) != 0 {
                            lf_checker_rt::callee_cdecl!(FREE, u32, h);
                            count += 1;
                        }
                        (e as *mut u32).write(0);
                        (e.wrapping_add(4) as *mut u16).write_unaligned(CLEARED_TAG);
                        (e.wrapping_add(8) as *mut u32).write(0);
                        (e.wrapping_add(12) as *mut u32).write(0);
                        (e.wrapping_add(16) as *mut u16).write_unaligned(0);
                    }
                }
            }
            i += 1;
        }
        count
    }
});
