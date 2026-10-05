// original: 0x00c22930 stream_entry_remove (proposed)
/// Remove every entry whose first word equals `key` from the global entry
/// array (base 0x016b9d84, 16 bytes per entry, `count` entries from
/// 0x010482cc, all counts signed): each match shifts the later entries
/// down one slot with 16-byte moves and drops the count, and the scan
/// restarts until a full pass finds nothing. A count of 1 or less (or
/// negative) takes no action. Always returns 1 (low byte; the contract
/// compares `al` since the upper bytes keep entry garbage on the
/// no-action path).
///
/// The loops are transcribed exactly (index bounds, copy order, restart),
/// not the intent: the scan reads one slot past the nominal end on entry.
///
/// Original: 0x00c22930 (cdecl, one stack word; `al` result).
lf_checker_rt::export!(cdecl, rw_00c22930(key: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x010482cc;
        const BASE: u32 = 0x016b9d84;
        const WRITE0: u32 = 0x016b9d88;
        const ROW: u32 = 0x10;
        let r = lf_checker_rt::relocated;
        let mut count = (r(COUNT) as *const i32).read_unaligned();
        loop {
            if count <= 1 {
                break;
            }
            let mut removed = false;
            let mut read = (count as u32).wrapping_mul(ROW).wrapping_add(r(BASE));
            let mut write = r(WRITE0);
            let mut cursor: i32 = 2;
            loop {
                if (read as *const u32).read_unaligned() == key {
                    if cursor < count {
                        let mut left = (count - cursor) as u32;
                        let mut from = write.wrapping_add(ROW);
                        loop {
                            let lo = (from as *const u64).read_unaligned();
                            (write as *mut u64).write_unaligned(lo);
                            let hi = (from.wrapping_add(8) as *const u64).read_unaligned();
                            (write.wrapping_add(8) as *mut u64).write_unaligned(hi);
                            from = from.wrapping_add(ROW);
                            left = left.wrapping_sub(1);
                            if left == 0 {
                                break;
                            }
                        }
                    }
                    count -= 1;
                    read = read.wrapping_sub(ROW);
                    removed = true;
                }
                cursor += 1;
                write = write.wrapping_add(ROW);
                if !(cursor.wrapping_sub(1) < count) {
                    break;
                }
            }
            (r(COUNT) as *mut u32).write_unaligned(count as u32);
            if !removed {
                break;
            }
        }
        1
    }
});
