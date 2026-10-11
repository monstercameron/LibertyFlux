// original: 0x00D5B230 loader_remove_ring_entry (proposed)

/// Remove the ring entry at a signed logical index from an eight-slot queue.
/// Entries after it are copied one slot toward the head, using wrapping
/// indices and the queue's `+0x24` head field. The signed length is at
/// `+0x28`, and the active slot is at `+0x20`; removing index zero wraps the
/// active slot to seven. The function returns the decremented active value
/// (zero when the stored slot wrapped to seven).
lf_checker_rt::export!(thiscall, rw_00D5B230(queue: u32, logical_index: u32) -> u32 {
    unsafe {
        const ENTRY_WORDS: u32 = 8;
        const ENTRY_MASK: u32 = ENTRY_WORDS - 1;
        const ACTIVE_SLOT: u32 = 0x20;
        const HEAD_SLOT: u32 = 0x24;
        const QUEUE_LENGTH: u32 = 0x28;
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let length = read_u32(queue.wrapping_add(QUEUE_LENGTH));
        let mut cursor = logical_index.wrapping_add(1);
        if (cursor as i32) < (length as i32) {
            let head = read_u32(queue.wrapping_add(HEAD_SLOT));
            loop {
                let destination = head.wrapping_add(cursor) & ENTRY_MASK;
                let source = destination.wrapping_add(1) & ENTRY_MASK;
                let source_value = read_u32(queue.wrapping_add(source.wrapping_mul(4)));
                write_u32(queue.wrapping_add(destination.wrapping_mul(4)), source_value);
                cursor = cursor.wrapping_add(1);
                if !((cursor as i32) < (length as i32)) {
                    break;
                }
            }
        }

        let old_active = read_u32(queue.wrapping_add(ACTIVE_SLOT));
        write_u32(queue.wrapping_add(QUEUE_LENGTH), length.wrapping_sub(1));
        if old_active == 0 {
            write_u32(queue.wrapping_add(ACTIVE_SLOT), ENTRY_MASK);
            0
        } else {
            let new_active = old_active.wrapping_sub(1);
            write_u32(queue.wrapping_add(ACTIVE_SLOT), new_active);
            new_active
        }
    }
});

