// original: 0x005DAF60 read_active_field_24_or_minus_one

/// Return field `+0x24` from the requested class record when its active byte
/// at `+0x08` is nonzero; otherwise return `u32::MAX`. The unsigned table index
/// is a cdecl stack argument, and the table has no bounds check here.
lf_checker_rt::export!(cdecl, rw_005daf60(class_index: u32) -> u32 {
    const CLASS_TABLE: u32 = 0x0118_f6f8;
    const ACTIVE_FLAG_OFFSET: usize = 0x08;
    const VALUE_OFFSET: usize = 0x24;
    unsafe {
        let table = lf_checker_rt::global::<*mut u8>(CLASS_TABLE);
        let record = table.add(class_index as usize).read_unaligned();
        if record.add(ACTIVE_FLAG_OFFSET).read() != 0 {
            record.add(VALUE_OFFSET).cast::<u32>().read_unaligned()
        } else {
            u32::MAX
        }
    }
});
