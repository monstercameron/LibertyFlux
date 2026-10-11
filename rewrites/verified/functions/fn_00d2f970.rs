// original: 0x00D2F970 path_choose_next_event (proposed)

/// Select a path-transition code from the current slot flags, the requested
/// node index, a mode byte and the count word pointed to by the fourth stack
/// argument. A current-slot mask returns 0x3bb immediately. In mode zero,
/// equal indices return 0x11a and otherwise a signed comparison of the next
/// index with the count selects 0x387 or 0x3af. In nonzero mode, equality
/// sets bit 0x10000 in the task flags and returns 0x516 or 0x3c1 according to
/// bit 0x40000000; adjacent indices return 0x384 or 0x387 according to mask
/// 0x280000, and other indices return 0x3af. The receiver is in ECX and four
/// 32-bit stack arguments are removed by the callee.
lf_checker_rt::export!(thiscall, rw_00D2F970(this: u32, _first: u32, current: u32, mode: u32, item: u32) -> u32 {
    const PATH_POINTER: u32 = 0x70;
    const CURRENT_NODE: u32 = 0x98;
    const SLOT_FLAGS: u32 = 0x78;
    const TASK_FLAGS: u32 = 0xd8;
    const SPECIAL_SLOT_MASK: u8 = 0x35;
    const TRANSITION_BLOCKED: u32 = 0x280000;
    const SPECIAL_RETURN_FLAG: u32 = 0x40000000;

    unsafe {
        let path = crate::read_u32(this.wrapping_add(PATH_POINTER));
        let mut slot_flags = 0u8;
        if path != 0 {
            let node = crate::read_u8(this.wrapping_add(CURRENT_NODE)) as i8 as i32;
            let path_count = crate::read_u32(path) as i32;
            if node >= 0 && node < path_count {
                let word = this
                    .wrapping_add(SLOT_FLAGS)
                    .wrapping_add(node.wrapping_mul(4) as u32);
                slot_flags = crate::read_u16(word) as u8;
            }
        }
        if slot_flags & SPECIAL_SLOT_MASK != 0 {
            return 0x3bb;
        }

        let item_count = crate::read_u32(item);
        let next = current.wrapping_add(1);
        if mode as u8 == 0 {
            if current == item_count {
                return 0x11a;
            }
            if (next as i32) >= (item_count as i32) {
                0x387
            } else {
                0x3af
            }
        } else if current == item_count {
            let flags_address = this.wrapping_add(TASK_FLAGS);
            let task_flags = crate::read_u32(flags_address) | 0x10000;
            crate::write_u32(flags_address, task_flags);
            if task_flags & SPECIAL_RETURN_FLAG != 0 {
                0x3c1
            } else {
                0x516
            }
        } else if next == item_count {
            if crate::read_u32(this.wrapping_add(TASK_FLAGS)) & TRANSITION_BLOCKED != 0 {
                0x384
            } else {
                0x387
            }
        } else {
            0x3af
        }
    }
});
