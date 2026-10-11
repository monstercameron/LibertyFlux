// original: 0x00C726B0 set_ped_task_flag

/// Set bit 25 of each non-null item referenced by a chain of task records to
/// the low bit of `enabled`. Each node stores a signed 16-bit item count at
/// `+0x20`, inline item pointers starting at `+0x24`, and the next node at
/// `+0x4c`.
/// Nonpositive counts visit no items; null item pointers are skipped.
/// The function has no semantic return value.
lf_checker_rt::export!(cdecl, rw_00c726b0(head: u32, enabled: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        const ITEM_COUNT: u32 = 0x20;
        const ITEM_ARRAY: u32 = 0x24;
        const NEXT_NODE: u32 = 0x4c;
        const FLAGS: u32 = 0x28;
        const FLAG_BIT: u32 = 0x0200_0000;
        let mut node = head;
        let requested_bit = ((enabled as u8 as u32) << 25) & FLAG_BIT;
        while node != 0 {
            let count = rd16(node + ITEM_COUNT) as i16;
            if count > 0 {
                for index in 0..count as u32 {
                    let item = rd32(node.wrapping_add(ITEM_ARRAY).wrapping_add(index.wrapping_mul(4)));
                    if item != 0 {
                        // The original reads the flag word even when the
                        // requested bit is zero; keep that fault behavior.
                        let old_flags = rd32(item + FLAGS);
                        let new_flags = (old_flags & !FLAG_BIT) | requested_bit;
                        if new_flags != old_flags {
                            wr32(item + FLAGS, new_flags);
                        }
                    }
                }
            }
            node = rd32(node + NEXT_NODE);
        }
    }
    0
});
