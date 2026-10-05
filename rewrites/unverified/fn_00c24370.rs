// original: 0x00c24370 cam_sample_append (proposed)
/// Append `sample` to the global sample table (base 0x016c8658) if the count
/// at 0x016c86a8 is below 20, compared SIGNED (a negative count also
/// stores, at a slot below the table base), and the sample lies within the
/// inclusive float bounds [lower, upper] held in read-only data. Otherwise
/// stores nothing. Returns the count afterwards (the incremented count on
/// the storing path, the unchanged count otherwise).
///
/// Each `comiss` + `jb` pair jumps when the comparison is unordered too, so
/// the conditions are the negated ordered relations: store only if
/// `upper >= sample` and `sample >= lower` both hold ordered-or-equal.
///
/// Original: 0x00c24370 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00c24370(sample: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x016c86a8;
        const TABLE: u32 = 0x016c8658;
        const UPPER: u32 = 0x00fe88e8;
        const LOWER: u32 = 0x00fe8628;
        const CAP: u32 = 0x14;
        let count = (lf_checker_rt::relocated(COUNT) as *const u32).read_unaligned();
        if count as i32 >= CAP as i32 {
            return count;
        }
        let v = f32::from_bits(sample);
        let upper = ((lf_checker_rt::relocated(UPPER)) as *const f32).read_unaligned();
        if !(upper >= v) {
            return count;
        }
        let lower = ((lf_checker_rt::relocated(LOWER)) as *const f32).read_unaligned();
        if !(v >= lower) {
            return count;
        }
        let slot = lf_checker_rt::relocated(TABLE).wrapping_add(count.wrapping_mul(4));
        (slot as *mut u32).write_unaligned(sample);
        let next = count.wrapping_add(1);
        (lf_checker_rt::relocated(COUNT) as *mut u32).write_unaligned(next);
        next
    }
});
