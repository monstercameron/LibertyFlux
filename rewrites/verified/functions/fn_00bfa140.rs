// original: 0x00bfa140 task_vec3_copy_to_14

/// Copy three dwords from `src` into the task object at `+DST`.
///
/// Original: 0x00bfa140 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00bfa140(this: u32, src: u32) -> u32 {
    unsafe {
        const DST: u32 = 0x14;
        let mut i = 0u32;
        while i < 3 {
            let v = (src.wrapping_add(i * 4) as *const u32).read_unaligned();
            ((this + DST).wrapping_add(i * 4) as *mut u32).write_unaligned(v);
            i += 1;
        }
        0
    }
});
