// original: 0x00969DD0 copy_timing_pair_to_outputs

/// Copies two 32-bit values from object offsets `0x134c` and `0x1350` to
/// optional output pointers. Neither value is written unless both pointers are
/// non-null; the return value is the second source value on that path, or the
/// incoming EAX when either pointer is null.
lf_checker_rt::export!(thiscall, rw_00969dd0(this: u32, first_out: u32, second_out: u32) -> u32 {
    if first_out == 0 || second_out == 0 {
        return 0;
    }
    unsafe {
        let first_value = (this.wrapping_add(0x134c) as *const u32).read_unaligned();
        (first_out as *mut u32).write_unaligned(first_value);
        let second_value = (this.wrapping_add(0x1350) as *const u32).read_unaligned();
        (second_out as *mut u32).write_unaligned(second_value);
        second_value
    }
});
