// original: 0x00A3CE30 vehicle_initialize_attachment_record

/// Initialize a compact attachment record. The receiver points to a record
/// whose words at offsets `0x00`, `0x04`, and `0x08` are cleared, whose
/// 16-bit field at `0x12` and byte at `0x10` are cleared, and whose word at
/// `0x0c` becomes one. Return the receiver unchanged.
///
/// The original is a thiscall method with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00a3ce30(this: u32) -> u32 {
    const RECORD_WORD_0: u32 = 0x00;
    const RECORD_WORD_1: u32 = 0x04;
    const RECORD_WORD_2: u32 = 0x08;
    const RECORD_STATE: u32 = 0x0c;
    const RECORD_FLAG: u32 = 0x10;
    const RECORD_KIND: u32 = 0x12;

    unsafe {
        (this.wrapping_add(RECORD_WORD_0) as *mut u32).write_unaligned(0);
        (this.wrapping_add(RECORD_WORD_1) as *mut u32).write_unaligned(0);
        (this.wrapping_add(RECORD_WORD_2) as *mut u32).write_unaligned(0);
        (this.wrapping_add(RECORD_STATE) as *mut u32).write_unaligned(1);
        (this.wrapping_add(RECORD_FLAG) as *mut u8).write(0);
        (this.wrapping_add(RECORD_KIND) as *mut u16).write_unaligned(0);
    }
    this
});
