// original: 0x005DAF80 set_active_record_field_5c

/// Store `value` at field `+0x5c` of the indexed record only when its active
/// byte at `+0x08` is nonzero. For the original's observable EAX value, return
/// `value` on the store path and `class_index` on the inactive path. This is a
/// cdecl function with two caller-popped stack arguments and no bounds check.
lf_checker_rt::export!(cdecl, rw_005daf80(class_index: u32, value: u32) -> u32 {
    const CLASS_TABLE: u32 = 0x0118_f6f8;
    const ACTIVE_FLAG_OFFSET: usize = 0x08;
    const VALUE_OFFSET: usize = 0x5c;
    unsafe {
        let table = lf_checker_rt::global::<*mut u8>(CLASS_TABLE);
        let record = table.add(class_index as usize).read_unaligned();
        if record.add(ACTIVE_FLAG_OFFSET).read() != 0 {
            record.add(VALUE_OFFSET).cast::<u32>().write_unaligned(value);
            value
        } else {
            class_index
        }
    }
});
