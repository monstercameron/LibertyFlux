// original: 0x00b32d10 subtask_kind_class (proposed)

/// Classify a record by the kind word it points to.
///
/// `obj` points to a record; `v` is its first word. When `v - 12` is above
/// 17 the result keeps the high bits of `v - 12` with the low byte set to 1;
/// otherwise the result is 0 or 1 from a fixed table (1 for `v` in 13, 14,
/// 15, 17, 18, 19, 20, 21, 24, 25, 26).
///
/// Original: 0x00b32d10 (cdecl, one stack word; a range-checked jump table
/// whose out-of-range arm falls into a set-low-byte exit).
lf_checker_rt::export!(cdecl, rw_00b32d10(obj: u32) -> u32 {
    unsafe {
        const KIND_HIT: [u32; 11] = [13, 14, 15, 17, 18, 19, 20, 21, 24, 25, 26];
        let v = (obj as *const u32).read_unaligned();
        let d = v.wrapping_sub(12);
        if d > 0x11 {
            d & !0xFF | 1
        } else {
            u32::from(KIND_HIT.contains(&v))
        }
    }
});
