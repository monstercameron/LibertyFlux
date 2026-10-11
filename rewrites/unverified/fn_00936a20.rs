// original: 0x00936A20 compact_network_state_records

/// Remove the prefix before `start_index` from the receiver's sixteen-byte
/// record array and parallel byte array. The signed difference between the
/// record count and `start_index` controls whether records are copied; when it
/// is positive, that many records move toward the front in ascending order.
/// The three receiver counters at `+0xd60`, `+0xd88`, and `+0xd98` are then
/// reduced with wrapping 32-bit subtraction. Record words are copied as raw
/// data, including fields that happen to contain floating-point bits. This is
/// a thiscall routine with one stack argument and no defined return value.
lf_checker_rt::export!(thiscall, rw_00936a20(this: u32, start_index: u32) -> () {
    unsafe {
        const RECORDS: u32 = 8;
        const RECORD_BYTES: u32 = 16;
        const PARALLEL_BYTES: u32 = 0xc80;
        const RECORD_COUNT: u32 = 0xd60;
        const SECOND_COUNTER: u32 = 0xd88;
        const THIRD_COUNTER: u32 = 0xd98;

        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let count_address = this.wrapping_add(RECORD_COUNT);
        let remaining = read_u32(count_address).wrapping_sub(start_index) as i32;
        if remaining > 0 {
            for destination_index in 0..remaining as u32 {
                let source_index = start_index.wrapping_add(destination_index);
                let source = this
                    .wrapping_add(RECORDS)
                    .wrapping_add(source_index.wrapping_mul(RECORD_BYTES));
                let destination = this
                    .wrapping_add(RECORDS)
                    .wrapping_add(destination_index.wrapping_mul(RECORD_BYTES));
                for word_index in 0..4u32 {
                    let value = read_u32(source.wrapping_add(word_index * 4));
                    write_u32(destination.wrapping_add(word_index * 4), value);
                }

                let byte_value = unsafe {
                    (this.wrapping_add(PARALLEL_BYTES).wrapping_add(source_index) as *const u8).read()
                };
                unsafe {
                    (this.wrapping_add(PARALLEL_BYTES).wrapping_add(destination_index) as *mut u8)
                        .write(byte_value)
                };
            }
        }

        write_u32(count_address, read_u32(count_address).wrapping_sub(start_index));
        let second_address = this.wrapping_add(SECOND_COUNTER);
        write_u32(second_address, read_u32(second_address).wrapping_sub(start_index));
        let third_address = this.wrapping_add(THIRD_COUNTER);
        write_u32(third_address, read_u32(third_address).wrapping_sub(start_index));
    }
});
