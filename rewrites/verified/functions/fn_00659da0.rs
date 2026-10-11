// original: 0x00659DA0 stream_build_child_byte_bitset

/// Clear five dwords at `this + 0x10`, then visit each entry pointer from the
/// array at `this + 0`. Each entry contains an unsigned 16-bit row count at
/// `+8` and a row-pointer array at `+4`. For each row, examine its first 40
/// signed bytes. A non-negative byte sets the bit at `row_index * 40 + byte`
/// in the five-dword bitset. The bitset is accumulated across all entries.
/// Return the unsigned 16-bit entry count at `this + 4`.
lf_checker_rt::export!(thiscall, rw_00659DA0(this: u32) -> u32 {
    unsafe {
        const ENTRY_ARRAY: u32 = 0x00;
        const ENTRY_COUNT: u32 = 0x04;
        const BITSET: u32 = 0x10;
        const ROW_ARRAY: u32 = 0x04;
        const ROW_COUNT: u32 = 0x08;
        const ROW_BYTES: u32 = 40;
        const BITSET_WORDS: u32 = 5;

        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u32 {
            unsafe { core::ptr::read_unaligned(address as *const u16) as u32 }
        }

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { core::ptr::read_unaligned(address as *const u32) }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { core::ptr::write_unaligned(address as *mut u32, value) }
        }

        #[inline(always)]
        unsafe fn read_i8(address: u32) -> i8 {
            unsafe { core::ptr::read(address as *const i8) }
        }

        let entry_count = read_u16(this.wrapping_add(ENTRY_COUNT));
        let entry_array = read_u32(this.wrapping_add(ENTRY_ARRAY));
        let mut word_index = 0u32;
        while word_index < BITSET_WORDS {
            write_u32(
                this.wrapping_add(BITSET).wrapping_add(word_index * 4),
                0,
            );
            word_index += 1;
        }

        let mut entry_index = 0u32;
        while entry_index < entry_count {
            let entry = read_u32(entry_array.wrapping_add(entry_index * 4));
            let row_array = read_u32(entry.wrapping_add(ROW_ARRAY));
            let row_count = read_u16(entry.wrapping_add(ROW_COUNT));
            let mut row_index = 0u32;
            while row_index < row_count {
                let row = read_u32(row_array.wrapping_add(row_index * 4));
                let row_bit_base = row_index * ROW_BYTES;
                let mut byte_index = 0u32;
                while byte_index < ROW_BYTES {
                    if read_i8(row.wrapping_add(byte_index)) >= 0 {
                        let bit_index = row_bit_base + byte_index;
                        let word_address = this
                            .wrapping_add(BITSET)
                            .wrapping_add((bit_index / 32) * 4);
                        let bit = 1u32 << (bit_index & 31);
                        let prior = read_u32(word_address);
                        write_u32(word_address, prior | bit);
                    }
                    byte_index += 1;
                }
                row_index += 1;
            }
            entry_index += 1;
        }

        entry_count
    }
});
