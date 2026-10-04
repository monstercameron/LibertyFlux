// original: 0x008e6640 insertion_sort_entries
/// Insertion sort over `[begin, end)` of 8-byte entries by float key: an
/// element greater than the first is shifted into place at the front,
/// anything else goes through the single-element insert helper (cdecl/3,
/// stubbed; its dead fourth word is not modelled). Returns the last key or
/// helper answer seen, matching the original's exit register.
export!(cdecl, rw_008e6640(begin: *mut u8, end: *const u8) -> u32 {
    unsafe {
        let mut last = 0u32;
        if begin as *const u8 != end {
            let mut cur = begin.add(8);
            if cur as *const u8 != end {
                let mut span = 8usize;
                loop {
                    let saved = entry_at(cur, 0);
                    let base_key = f32::from_bits(read_unaligned(
                        begin.add(4) as *const u32,
                    ));
                    if f32::from_bits(saved.1) > base_key {
                        memmove(begin, begin.add(8), span);
                        set_entry(begin, 0, saved);
                        last = saved.1;
                    } else {
                        last = callee_cdecl!(
                            1, u32, cur as u32, saved.0, saved.1
                        );
                    }
                    cur = cur.add(8);
                    span += 8;
                    if cur as *const u8 == end {
                        break;
                    }
                }
            }
        }
        last
    }
});
