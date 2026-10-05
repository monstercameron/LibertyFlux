// original: 0x00ABF270 stream_zero_flag_stride_table (proposed)

/// Zero one flag byte in each of sixteen table entries.
///
/// The original writes 0 to `FIRST + k * STRIDE` for k in 0..16 (cdecl, no
/// arguments), stepping a pointer until it reaches END. No value is returned.
lf_checker_rt::export!(cdecl, rw_00ABF270() -> u32 {
    unsafe {
        const FIRST: u32 = 0x0154CCEC;
        const END: u32 = 0x0154CFEC;
        const STRIDE: u32 = 0x30;
        let end = lf_checker_rt::relocated(END);
        let mut slot = lf_checker_rt::relocated(FIRST);
        loop {
            (slot as *mut u8).write(0);
            slot = slot.wrapping_add(STRIDE);
            if slot >= end {
                break;
            }
        }
        0
    }
});
