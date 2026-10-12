// original: 0x00876C60 rebase_frame_sample_bounds

/// Rebase all optional pointer fields in a frame-sample bounds record.
///
/// Each non-null pointer at offsets `+0x08`, `+0x00`, `+0x04`, `+0x14`,
/// `+0x0c`, and `+0x10` is passed, in that order, to the relocation helper
/// with the caller's adjustment as its this pointer. The helper's returned
/// delta is wrapping-added to the field. The three maximum bounds and three
/// minimum bounds are then reset to their finite sentinel bit patterns.
///
/// Original calling convention: thiscall with one stack adjustment argument.
lf_checker_rt::export!(thiscall, rw_00876c60(record: u32, adjustment: u32) -> u32 {
    const POINTER_FIELDS: [u32; 6] = [0x08, 0x00, 0x04, 0x14, 0x0c, 0x10];
    const UPPER_BOUND_OFFSETS: [u32; 3] = [0x28, 0x24, 0x20];
    const LOWER_BOUND_OFFSETS: [u32; 3] = [0x38, 0x34, 0x30];
    const MAX_BOUND: u32 = 0x7f7f_ffff;
    const MIN_BOUND: u32 = 0x0080_0000;
    unsafe {
        for field_offset in POINTER_FIELDS {
            let old_pointer = (record.wrapping_add(field_offset) as *const u32).read_unaligned();
            if old_pointer != 0 {
                let delta = lf_checker_rt::callee_thiscall!(1, u32, adjustment, old_pointer);
                (record.wrapping_add(field_offset) as *mut u32)
                    .write_unaligned(old_pointer.wrapping_add(delta));
            }
        }
        for offset in UPPER_BOUND_OFFSETS {
            (record.wrapping_add(offset) as *mut u32).write_unaligned(MAX_BOUND);
        }
        for offset in LOWER_BOUND_OFFSETS {
            (record.wrapping_add(offset) as *mut u32).write_unaligned(MIN_BOUND);
        }
    }
    record
});
