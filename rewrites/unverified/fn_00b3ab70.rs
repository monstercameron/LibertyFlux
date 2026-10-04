// original: 0x00b3ab70 task_bounds_store_secondary (proposed)

/// Store two four-float bound vectors into the secondary task-bounds
/// globals: the 16 bytes at `src_first` go to the first slots, the 16 bytes
/// at `src_second` to the second slots, then the secondary-ready flag byte is
/// set to 1. Words are moved raw (no arithmetic). Returns `src_second`,
/// matching the original's exit register. Original: 0x00b3ab70 (cdecl, two
/// stack words).
lf_checker_rt::export!(cdecl, rw_00b3ab70(src_first: u32, src_second: u32) -> u32 {
    unsafe {
        const FIRST_SLOTS: u32 = 0x016624e0;
        const SECOND_SLOTS: u32 = 0x016624d0;
        const READY_FLAG: u32 = 0x01662492;
        const VEC_WORDS: usize = 4;
        let dst_first = lf_checker_rt::global::<u32>(FIRST_SLOTS);
        let dst_second = lf_checker_rt::global::<u32>(SECOND_SLOTS);
        core::ptr::copy_nonoverlapping(
            src_first as *const u32,
            dst_first,
            VEC_WORDS,
        );
        core::ptr::copy_nonoverlapping(
            src_second as *const u32,
            dst_second,
            VEC_WORDS,
        );
        lf_checker_rt::global::<u8>(READY_FLAG).write(1);
        src_second
    }
});
