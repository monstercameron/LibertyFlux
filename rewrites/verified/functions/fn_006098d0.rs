// original: 0x006098D0 physics_copy_record

/// Copy the selected record's defined words from `source` into the record
/// array referenced by `this + 0x88`. The record stride is 0x40 bytes; the
/// gaps at 0x0c, 0x1c and 0x2c are left untouched. The machine return is the
/// final copied word. This is a 32-bit thiscall with an index and source
/// pointer on the stack.
lf_checker_rt::export!(thiscall, rw_006098D0(this: u32, record_index: u32, source: u32) -> u32 {
    unsafe {
        const RECORDS: u32 = 0x88;
        const STRIDE: u32 = 0x40;
        const WORD_OFFSETS: [u32; 12] = [0, 4, 8, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28, 0x30, 0x34, 0x38];
        let records = ((this + RECORDS) as *const u32).read_unaligned();
        let destination = records.wrapping_add(record_index.wrapping_mul(STRIDE));
        let mut last = 0;
        for offset in WORD_OFFSETS {
            last = ((source.wrapping_add(offset)) as *const u32).read_unaligned();
            ((destination.wrapping_add(offset)) as *mut u32).write_unaligned(last);
        }
        last
    }
});
