// original: 0x00aba040 match_names_fill_slots

/// Match eleven global name strings against an item list, fill hit slots.
///
/// Each of the eleven global string pointers is compared against the names
/// of the items (0xe0-byte records whose first word points at the name)
/// until one matches; the matching item index is stored into the next output
/// slot. Slots with no match, and every slot when the item count is zero,
/// are left untouched. The string comparison checks two bytes per step and
/// only the equal/not-equal outcome is used.
export!(thiscall, rs64_aba040(this: *mut u32, arg0: *const u8) -> u32 {
    fn streq(a: u32, b: u32) -> bool {
        let mut p = a;
        let mut q = b;
        loop {
            let x0 = unsafe { *(p as *const u8) };
            let y0 = unsafe { *(q as *const u8) };
            if x0 != y0 {
                return false;
            }
            if x0 == 0 {
                return true;
            }
            let x1 = unsafe { *((p.wrapping_add(1)) as *const u8) };
            let y1 = unsafe { *((q.wrapping_add(1)) as *const u8) };
            if x1 != y1 {
                return false;
            }
            if x1 == 0 {
                return true;
            }
            p = p.wrapping_add(2);
            q = q.wrapping_add(2);
        }
    }
    unsafe {
        const NAMES: u32 = 0x0103_EE60;
        const SLOTS: u32 = 11;
        const ITEM_STRIDE: u32 = 0xe0;
        let count = *(((arg0 as u32).wrapping_add(0x14)) as *const u16) as u32;
        let items = *(arg0 as *const u32);
        let mut slot = this as u32;
        let mut k = 0u32;
        while k < SLOTS {
            if count != 0 {
                let want = *global::<u32>(NAMES).add(k as usize);
                let mut idx = 0u32;
                let mut item = items;
                while idx < count {
                    let name = *(item as *const u32);
                    if streq(name, want) {
                        *(slot as *mut u32) = idx;
                        break;
                    }
                    idx += 1;
                    item = item.wrapping_add(ITEM_STRIDE);
                }
            }
            slot = slot.wrapping_add(4);
            k += 1;
        }
        (relocated(NAMES).wrapping_sub(this as u32) & 0xFFFF_FF00) | 1
    }
});
