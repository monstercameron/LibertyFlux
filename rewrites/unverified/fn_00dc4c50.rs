// original: 0x00DC4C50 task_state_flag_lookup

/// Read a packed task state bit through the task metadata table.
///
/// The receiver points to a packed key: the low 16 bits select a global
/// metadata pointer and the high 16 bits select a 32-byte record. A null
/// metadata pointer returns false. Otherwise bit 6 of the byte at record
/// offset 0x1e is returned as the low-byte boolean.
lf_checker_rt::export!(thiscall, rw_00DC4C50(key_address: u32) -> u8 {
    unsafe {
        const KEY_VALUE: u32 = 0x00;
        const METADATA_POINTERS: u32 = 0x0117_8284;
        const METADATA_RECORD_BYTES: u32 = 0x20;
        const STATE_FLAGS: u32 = 0x1e;
        const TARGET_BIT: u8 = 0x40;

        let packed_key = (key_address as *const u32).read_unaligned();
        let metadata_index = packed_key & 0xffff;
        let record_index = packed_key >> 16;
        let pointer_slot = METADATA_POINTERS
            .wrapping_add(metadata_index.wrapping_mul(4));
        let metadata = (lf_checker_rt::relocated(pointer_slot) as *const u32)
            .read_unaligned();
        if metadata == 0 {
            return 0;
        }

        let flag_address = metadata
            .wrapping_add(record_index.wrapping_mul(METADATA_RECORD_BYTES))
            .wrapping_add(STATE_FLAGS);
        let flags = (flag_address as *const u8).read();
        u8::from(flags & TARGET_BIT != 0)
    }
});
