// original: 0x00dcf700 ui_string_list_remove (proposed)

/// Remove every list entry equal to `text`, freeing each removed string.
/// A null or empty search string, or an empty list, does nothing.
///
/// `this` points to the list: entry-pointer array at `+0x228`, 16-bit count
/// at `+0x22c`. Entries are compared with an inline strcmp that steps two
/// bytes at a time and only distinguishes equal from not equal. On a match
/// the entry goes to the deallocator callee (cdecl, one word), the tail
/// shifts one slot left and the count shrinks by one; the scan index still
/// advances, so an entry shifted into the removed slot is not re-checked.
/// The count is re-read after every entry. No result.
///
/// Original: 0x00DCF700 (thiscall, one stack word, no result).
lf_checker_rt::export!(thiscall, rw_00dcf700(this: u32, text: u32) -> u32 {
    unsafe {
        /// String deallocator callee id.
        const FREE: u32 = 1;
        const ITEMS: u32 = 0x228;
        const COUNT: u32 = 0x22c;
        if text == 0 || (text as *const u8).read() == 0 {
            return 0;
        }
        let total = ((this + COUNT) as *const u16).read_unaligned() as u32;
        if 0 >= total {
            return 0;
        }
        let mut i: u32 = 0;
        loop {
            let base = ((this + ITEMS) as *const u32).read_unaligned();
            let entry = ((base.wrapping_add(i.wrapping_mul(4))) as *const u32).read_unaligned();
            // Inline strcmp, two bytes per step, as in the original.
            let mut a = entry;
            let mut b = text;
            let ord = loop {
                let ca = (a as *const u8).read();
                let cb = (b as *const u8).read();
                if ca != cb {
                    break 1u32;
                }
                if ca == 0 {
                    break 0u32;
                }
                let ca1 = ((a.wrapping_add(1)) as *const u8).read();
                let cb1 = ((b.wrapping_add(1)) as *const u8).read();
                if ca1 != cb1 {
                    break 1u32;
                }
                a = a.wrapping_add(2);
                b = b.wrapping_add(2);
                if ca1 == 0 {
                    break 0u32;
                }
            };
            if ord == 0 {
                lf_checker_rt::callee_cdecl!(FREE, u32, entry);
                // Shift the tail left and shrink the count (u16).
                let n = ((this + COUNT) as *const u16).read_unaligned() as u32;
                let base = ((this + ITEMS) as *const u32).read_unaligned();
                let mut j = i;
                while j.wrapping_add(1) < n {
                    let v = ((base.wrapping_add(j.wrapping_add(1).wrapping_mul(4))) as *const u32)
                        .read_unaligned();
                    ((base.wrapping_add(j.wrapping_mul(4))) as *mut u32).write_unaligned(v);
                    j = j.wrapping_add(1);
                }
                ((this + COUNT) as *mut u16)
                    .write_unaligned(n.wrapping_sub(1) as u16);
            }
            i = i.wrapping_add(1);
            let n = ((this + COUNT) as *const u16).read_unaligned() as u32;
            if i >= n {
                break;
            }
        }
        0
    }
});
