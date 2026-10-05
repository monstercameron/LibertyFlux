// original: 0x00bf9d70 task_vec3_copy_from_14

/// Copy three dwords from the task object at `+SRC` into `out`.
///
/// Original: 0x00bf9d70 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00bf9d70(this: u32, out: u32) -> u32 {
    unsafe {
        const SRC: u32 = 0x14;
        let mut i = 0u32;
        while i < 3 {
            let v = ((this + SRC).wrapping_add(i * 4) as *const u32).read_unaligned();
            (out.wrapping_add(i * 4) as *mut u32).write_unaligned(v);
            i += 1;
        }
        0
    }
});
