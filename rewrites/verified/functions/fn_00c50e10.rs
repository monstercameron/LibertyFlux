// original: 0x00C50E10 task_bank_flag_bit_14

/// Read one bit from the flags word associated with a signed 16-bit task id.
///
/// The id is stored at byte offset 0x2e in the input record. It indexes a
/// pointer table at the relocated image address `0x1295CD8`; the selected
/// object supplies its flags word at byte offset 0x94. Returns bit 14 as
/// zero or one. Table indexing follows the original's signed index and
/// 32-bit address arithmetic.
lf_checker_rt::export!(stdcall, rw_00c50e10(record: u32) -> u32 {
    unsafe {
        const TABLE_FILE_VA: u32 = 0x1295_cd8;
        const FLAGS_OFFSET: u32 = 0x94;
        const FLAG_BIT: u32 = 14;
        let task_id = (record.wrapping_add(0x2e) as *const i16).read_unaligned() as i32;
        let table_slot = TABLE_FILE_VA.wrapping_add((task_id as u32).wrapping_mul(4));
        let flag_object = (lf_checker_rt::relocated(table_slot) as *const u32).read_unaligned();
        let flags = (flag_object.wrapping_add(FLAGS_OFFSET) as *const u32).read_unaligned();
        (flags >> FLAG_BIT) & 1
    }
});
