// original: 0x00695700 id_table_search (proposed)

/// Search the sorted pointer array in an ID table for a packed 24-bit key.
/// The `thiscall` receiver points to a table whose pointer is at `+0` and
/// whose unsigned 16-bit item count is at `+4`. Each item pointer names a
/// record with its high key byte at `+0` and low 16 bits at `+2`. The first
/// two stack arguments contribute an unsigned byte and unsigned word to the
/// search key; the third points to a 32-bit output index. On an exact match,
/// store and return the matching record pointer. On a miss, store the lower
/// insertion position and return zero. An empty table stores index zero.
/// The original uses the third incoming argument slot as temporary storage
/// for the item-array pointer; the search below holds that intermediate in a
/// local instead. The 32-bit pointer array and object offsets are specific to
/// this original layout.
lf_checker_rt::export!(thiscall, rw_00695700(
    table: u32,
    key_high: u32,
    key_low: u32,
    output_index: u32
) -> u32 {
    unsafe {
        const ITEM_ARRAY_POINTER: u32 = 0x00;
        const ITEM_COUNT: u32 = 0x04;
        const RECORD_HIGH_BYTE: u32 = 0x00;
        const RECORD_LOW_WORD: u32 = 0x02;
        const POINTER_BYTES: u32 = 4;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let items = unsafe { read_u32(table + ITEM_ARRAY_POINTER) };
        let item_count = unsafe { read_u16(table + ITEM_COUNT) } as i32;
        let target_key = ((key_high as u8 as u32) << 16) | (key_low as u16 as u32);
        let mut lower = 0i32;
        let mut upper = item_count - 1;

        while lower <= upper {
            let middle = lower + ((upper - lower) / 2);
            unsafe { write_u32(output_index, middle as u32) };

            let pointer_slot = items.wrapping_add((middle as u32).wrapping_mul(POINTER_BYTES));
            let record = unsafe { read_u32(pointer_slot) };
            let record_key = ((unsafe { read_u8(record + RECORD_HIGH_BYTE) } as u32) << 16)
                | (unsafe { read_u16(record + RECORD_LOW_WORD) } as u32);

            if record_key == target_key {
                return record;
            }
            if record_key <= target_key {
                lower = middle + 1;
            } else {
                upper = middle - 1;
            }
        }

        unsafe { write_u32(output_index, lower as u32) };
        0
    }
});
