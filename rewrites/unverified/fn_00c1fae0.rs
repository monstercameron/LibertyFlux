// original: 0x00C1FAE0 streaming_initialize_sixteen_slots

/// Initialize sixteen consecutive five-word slot records. Each record holds
/// the word pattern `[u32::MAX, 0, 0, u32::MAX, 0]` at a 20-byte stride. The
/// receiver points to the first record in ECX; the method returns no defined
/// value. All writes are confined to the 320-byte record array.
lf_checker_rt::export!(thiscall, rw_00c1fae0(this: u32) -> () {
    unsafe {
        const SLOT_COUNT: usize = 16;
        const SLOT_STRIDE: u32 = 20;
        const FIRST_FIELD: u32 = 0xFFFF_FFFF;
        const EMPTY_FIELD: u32 = 0;
        const WORD_OFFSETS: [u32; 5] = [0, 4, 8, 12, 16];
        const FIELD_VALUES: [u32; 5] = [FIRST_FIELD, EMPTY_FIELD, EMPTY_FIELD, FIRST_FIELD, EMPTY_FIELD];

        for slot_index in 0..SLOT_COUNT {
            let slot_base = this.wrapping_add((slot_index as u32).wrapping_mul(SLOT_STRIDE));
            for (field_offset, field_value) in WORD_OFFSETS.into_iter().zip(FIELD_VALUES) {
                (slot_base.wrapping_add(field_offset) as *mut u32).write_unaligned(field_value);
            }
        }
    }
});
