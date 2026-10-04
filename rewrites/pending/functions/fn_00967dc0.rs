// original: 0x00967DC0 sample_derived_value
/// Sample one derived value into the table at `this`.
///
/// The sample count lives at `this + 0x2334`; a count at or above 0x40
/// first runs the reset helper (callee 1). Then the derive helper
/// (callee 2) runs on `query` and its `ST0` float result is truncated to
/// an integer exactly like the original's x87 `fistp` in chop mode (see
/// `fistp_qword_lo`) and stored at `this + count * 4 + 0x2540` (the
/// count is re-read after the call, as in the original). Finally the
/// count is incremented. No return value.
///
/// Original: 0x00967DC0 (thiscall, one stack word).

export!(thiscall, rw_00967DC0(this: u32, query: u32) -> u32 {
    unsafe {
        /// Low dword of `fistp qword` in chop mode: truncation toward zero;
        /// out-of-range, NaN or infinite input yields the invalid sentinel
        /// `0x8000000000000000` (low dword 0).
        fn fistp_qword_lo(x: f32) -> u32 {
            const TWO63: f32 = 9.223372e18; // 2^63, exactly representable
            if !x.is_finite() {
                return 0;
            }
            let t = x.trunc();
            if t >= TWO63 || t < -TWO63 {
                return 0;
            }
            (t as i64) as u32
        }
        const COUNT: u32 = 0x2334;
        const TABLE: u32 = 0x2540;
        const CAPACITY: u32 = 0x40;
        if (this.wrapping_add(COUNT) as *const u32).read_unaligned() >= CAPACITY {
            callee_thiscall!(1, u32, this);
        }
        let derived: f32 = callee_thiscall!(2, f32, this, query);
        let n = (this.wrapping_add(COUNT) as *const u32).read_unaligned();
        (this.wrapping_add(n.wrapping_mul(4)).wrapping_add(TABLE) as *mut u32)
            .write_unaligned(fistp_qword_lo(derived));
        (this.wrapping_add(COUNT) as *mut u32).write_unaligned(n.wrapping_add(1));
        0
    }
});
