// original: 0x00685CC0 mark_record_dirty_if_key_present

/// Ask the frame-record lookup helper for the record identified by two key
/// words. If it returns a pointer, set that record's dirty bit at byte 4 and
/// return one; otherwise return zero. The helper is intercepted by the
/// contract. This thiscall method pops both key words.
lf_checker_rt::export!(thiscall, rw_00685cc0(this: u32, key_high: u32, key_low: u32) -> u32 {
    unsafe {
        const DIRTY_FLAG_OFFSET: u32 = 4;
        const DIRTY_FLAG: u8 = 0x10;

        let record = lf_checker_rt::callee_thiscall!(1, u32, this, key_high, key_low);
        if record == 0 {
            0
        } else {
            let flags = crate::read_byte(record.wrapping_add(DIRTY_FLAG_OFFSET));
            crate::write_byte(record.wrapping_add(DIRTY_FLAG_OFFSET), flags | DIRTY_FLAG);
            1
        }
    }
});
