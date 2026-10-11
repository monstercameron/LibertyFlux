// original: 0x00a2d6c0 save_record_append_primary

/// Append a four-word source record to the next 16-byte slot, copy its
/// scalar bits to the parallel array, write the tag (and the secondary
/// routine's zero word), increment the shared count, and return the tag.
/// The input is cdecl: record pointer, single-precision scalar, and tag.
lf_checker_rt::export!(cdecl, rw_00a2d6c0(record: u32, scalar: f32, tag: u32) -> u32 {
    const COUNTER_VA: u32 = 0x012dd690;
    const TABLE_VA: u32 = 0x012ddc80;
    const SCALAR_VA: u32 = 0x012dd63c;
    const TAG_VA: u32 = 0x012dd664;
    unsafe {
        let count = lf_checker_rt::global::<u32>(COUNTER_VA).read_unaligned();
        let new_count = count.wrapping_add(1);
        let record_words = lf_checker_rt::global::<u32>(TABLE_VA).add(count.wrapping_mul(4) as usize);
        let word_0 = (record as *const u32).add(0).read_unaligned();
        record_words.add(0).write_unaligned(word_0);
        let word_1 = (record as *const u32).add(1).read_unaligned();
        record_words.add(1).write_unaligned(word_1);
        let word_2 = (record as *const u32).add(2).read_unaligned();
        record_words.add(2).write_unaligned(word_2);
        let word_3 = (record as *const u32).add(3).read_unaligned();
        record_words.add(3).write_unaligned(word_3);
        lf_checker_rt::global::<f32>(SCALAR_VA).add(new_count as usize).write_unaligned(scalar);
        lf_checker_rt::global::<u32>(TAG_VA).add(new_count as usize).write_unaligned(tag);
        lf_checker_rt::global::<u32>(COUNTER_VA).write_unaligned(new_count);
        tag
    }
});
