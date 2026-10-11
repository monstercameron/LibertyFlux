// original: 0x00B80DD0 ped_task_manager_contains_pointer

/// Compare the supplied pointer value against six entries beginning at
/// `this + 0x14`. Return one in AL on the first exact match and zero if all
/// six entries differ; the array is read-only.
lf_checker_rt::export!(thiscall, rw_b80dd0(this: u32, needle: u32) -> u32 {
    unsafe {
        const TASK_ARRAY: u32 = 0x14;
        const TASK_COUNT: u32 = 6;
        for index in 0..TASK_COUNT {
            let task = ((this + TASK_ARRAY + index * 4) as *const u32).read_unaligned();
            if task == needle {
                return 1;
            }
        }
        0
    }
});
