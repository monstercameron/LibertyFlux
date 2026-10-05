// original: 0x00AF7880 veh_state_zero_from (proposed)

/// Clear the state tail starting at a caller-chosen offset.
///
/// Zeroes the `0x0E - start` bytes at `this + 0x54 + start`. A `start` of
/// `0x0E` or more clears nothing (the count is unsigned, so large values
/// also clear nothing). The original clears whole dwords then leftover
/// bytes; the cleared range is identical.
///
/// Original: 0x00AF7880 (thiscall, one stack argument, callee cleans it).
lf_checker_rt::export!(thiscall, rw_00AF7880(this: u32, start: u32) -> u32 {
    unsafe {
        const TAIL: u32 = 0x54;
        const TAIL_LEN: u32 = 0x0E;
        if start < TAIL_LEN {
            let n = TAIL_LEN - start;
            core::ptr::write_bytes((this + TAIL + start) as *mut u8, 0, n as usize);
        }
        0
    }
});
