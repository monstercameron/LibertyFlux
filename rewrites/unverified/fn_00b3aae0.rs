// original: 0x00b3aae0 task_bounds_store_primary (proposed)

/// Store two four-float bound vectors into the primary task-bounds globals:
/// the 16 bytes at `src_min` go to the min-bound slots, the 16 bytes at
/// `src_max` to the max-bound slots, then the primary-ready flag byte is set
/// to 1. Words are moved raw (no arithmetic). Returns `src_max`, matching
/// the original's exit register. Original: 0x00b3aae0 (cdecl, two stack
/// words).
lf_checker_rt::export!(cdecl, rw_00b3aae0(src_min: u32, src_max: u32) -> u32 {
    unsafe {
        const MIN_SLOTS: u32 = 0x01662510;
        const MAX_SLOTS: u32 = 0x01662660;
        const READY_FLAG: u32 = 0x01662491;
        const VEC_WORDS: usize = 4;
        let dst_min = lf_checker_rt::global::<u32>(MIN_SLOTS);
        let dst_max = lf_checker_rt::global::<u32>(MAX_SLOTS);
        core::ptr::copy_nonoverlapping(
            src_min as *const u32,
            dst_min,
            VEC_WORDS,
        );
        core::ptr::copy_nonoverlapping(
            src_max as *const u32,
            dst_max,
            VEC_WORDS,
        );
        lf_checker_rt::global::<u8>(READY_FLAG).write(1);
        src_max
    }
});
