// original: 0x008e6aa0 partition_entries_by_key
/// Bidirectional partition of `[begin, end)` around the pivot key: scan up
/// past entries strictly greater than the pivot, scan down past entries
/// strictly below it, swap while the cursors have not crossed. Returns the
/// split pointer. The third word is never read; the pivot is the fourth.
export!(cdecl, rw_008e6aa0(
    begin: *mut u8,
    end: *mut u8,
    _unused: u32,
    pivot: u32,
) -> u32 {
    unsafe {
        let pivot = f32::from_bits(pivot);
        let mut lo = begin;
        let mut hi = end;
        loop {
            while f32::from_bits(read_unaligned(lo.add(4) as *const u32))
                > pivot
            {
                lo = lo.add(8);
            }
            loop {
                hi = hi.sub(8);
                if !(pivot
                    > f32::from_bits(read_unaligned(hi.add(4) as *const u32)))
                {
                    break;
                }
            }
            if (lo as usize) >= (hi as usize) {
                break;
            }
            let a = entry_at(lo, 0);
            let b = entry_at(hi, 0);
            set_entry(lo, 0, b);
            set_entry(hi, 0, a);
            lo = lo.add(8);
        }
        lo as u32
    }
});

