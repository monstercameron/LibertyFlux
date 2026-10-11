// original: 0x00969DD0 copy_timing_pair_to_outputs

/// Copies two 32-bit values from object offsets `0x134c` and `0x1350` to
/// optional output pointers. Each output is independent; the return value is
/// the last source value read, or the incoming EAX when both outputs are null.
lf_checker_rt::export!(thiscall, rw_00969dd0(this: u32, first_out: u32, second_out: u32) -> u32 {
    let mut last_value = 0u32;
    unsafe {
        if first_out != 0 {
            last_value = (this.wrapping_add(0x134c) as *const u32).read_unaligned();
            (first_out as *mut u32).write_unaligned(last_value);
        }
        if second_out != 0 {
            last_value = (this.wrapping_add(0x1350) as *const u32).read_unaligned();
            (second_out as *mut u32).write_unaligned(last_value);
        }
    }
    last_value
});
