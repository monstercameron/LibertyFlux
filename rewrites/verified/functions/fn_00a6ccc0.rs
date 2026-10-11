// original: 0x00A6CCC0 zero_fixed_stride_fields

/// Clears the first word of each of fifty records beginning at `this + 4`.
///
/// Records are separated by 36 bytes. The routine returns `this`, uses the
/// thiscall convention, and does not inspect the prior record contents.
lf_checker_rt::export!(thiscall, rw_00a6ccc0(this: u32) -> u32 {
    unsafe {
        const FIRST_FIELD: u32 = 4;
        const RECORD_COUNT: u32 = 50;
        const RECORD_STRIDE: u32 = 0x24;

        for record_index in 0..RECORD_COUNT {
            let field = this.wrapping_add(FIRST_FIELD + record_index * RECORD_STRIDE);
            (field as *mut u32).write_unaligned(0);
        }
        this
    }
});
