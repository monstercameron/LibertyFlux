// original: 0x005DB000 is_class_record_active_by_index

/// Return one when the unsigned class index in ECX selects a nonnull record
/// whose byte at `+0x08` is nonzero. A null table entry returns zero. This
/// fastcall entry consumes its index from ECX and returns the boolean in AL.
lf_checker_rt::export!(fastcall, rw_005db000(class_index: u32) -> u8 {
    const CLASS_TABLE: u32 = 0x0118_f6f8;
    const ACTIVE_FLAG_OFFSET: usize = 0x08;
    unsafe {
        let table = lf_checker_rt::global::<*mut u8>(CLASS_TABLE);
        let record = table.add(class_index as usize).read_unaligned();
        if record.is_null() {
            0
        } else {
            u8::from(record.add(ACTIVE_FLAG_OFFSET).read() != 0)
        }
    }
});
