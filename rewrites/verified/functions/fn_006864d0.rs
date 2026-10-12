// original: 0x006864D0 dof_array_checksum (proposed)

/// Compute a packed checksum over the pointer records in a DOF array. The
/// `thiscall` receiver has the item pointer at `+0x0c` and an unsigned
/// 16-bit item count at `+0x10`; each array element is a 32-bit pointer to a
/// record with bytes at `+4` and `+5` and a 16-bit word at `+6`. The checksum
/// starts both wrapping accumulators at `0xffff`. For each record, it joins
/// the high byte with the low nibble of byte `+4`, sets bit `0x80`, adds
/// that value and then the word at `+6` to the first accumulator, and adds
/// the first accumulator to the second after each addition. Every 180
/// records, both accumulators fold their high 16 bits into their low 16 bits.
/// The final result packs the folded second accumulator in the high word and
/// the folded first accumulator in the low bits. An empty array returns zero.
/// The original processes at most 180 items per chunk, including a partial
/// final chunk; all pointer fields and record offsets retain the source
/// program's 32-bit layout.
lf_checker_rt::export!(thiscall, rw_006864D0(array: u32) -> u32 {
    unsafe {
        const ITEM_POINTER: u32 = 0x0C;
        const ITEM_COUNT: u32 = 0x10;
        const ITEM_STRIDE: u32 = 4;
        const RECORD_FIELD_A: u32 = 0x04;
        const RECORD_FIELD_B: u32 = 0x05;
        const RECORD_FIELD_WORD: u32 = 0x06;
        const CHUNK_CAPACITY: u32 = 0xB4;
        const COMPONENT_BIT: u32 = 0x80;

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

        let item_count = unsafe { read_u16(array + ITEM_COUNT) } as u32;
        if item_count == 0 {
            return 0;
        }

        let items = unsafe { read_u32(array + ITEM_POINTER) };
        let mut first_checksum = 0xFFFFu32;
        let mut second_checksum = 0xFFFFu32;
        let mut first_unprocessed = 0u32;
        let mut remaining = item_count;

        while remaining != 0 {
            let chunk_size = remaining.min(CHUNK_CAPACITY);
            let chunk_end = first_unprocessed + chunk_size;
            while first_unprocessed < chunk_end {
                let pointer_slot = items
                    .wrapping_add(first_unprocessed.wrapping_mul(ITEM_STRIDE));
                let record = unsafe { read_u32(pointer_slot) };
                let high_byte = unsafe { read_u8(record + RECORD_FIELD_B) } as u16;
                let low_nibble = unsafe { read_u8(record + RECORD_FIELD_A) } & 0x0F;
                let packed_fields = (high_byte << 8) as u32
                    | low_nibble as u32
                    | COMPONENT_BIT;
                let trailing_word = unsafe { read_u16(record + RECORD_FIELD_WORD) } as u32;

                first_checksum = first_checksum.wrapping_add(packed_fields);
                second_checksum = second_checksum.wrapping_add(first_checksum);
                first_checksum = first_checksum.wrapping_add(trailing_word);
                second_checksum = second_checksum.wrapping_add(first_checksum);
                first_unprocessed += 1;
            }

            remaining -= chunk_size;
            first_checksum = (first_checksum & 0xFFFF).wrapping_add(first_checksum >> 16);
            second_checksum =
                (second_checksum & 0xFFFF).wrapping_add(second_checksum >> 16);
        }

        let low_fold = (first_checksum & 0xFFFF).wrapping_add(first_checksum >> 16);
        let packed_high = second_checksum
            .wrapping_shl(16)
            .wrapping_add(second_checksum & 0xFFFF_0000);
        low_fold | packed_high
    }
});
