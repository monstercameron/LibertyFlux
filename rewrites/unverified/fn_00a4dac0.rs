// original: 0x00A4DAC0 vehicle_flag_and_floats (proposed)

/// True when the enable flag is set and two floats are both positive.
///
/// Returns 0 unless bit 1 of the flag byte at `this + FLAG` (0x0F1D) is set.
/// Otherwise calls the measuring callee (`this` + `ARG`, 0x10D0, in `ecx`,
/// float answer on the x87 stack) and returns 1 only when that answer and
/// the float at `this + VAL` (0x10D8) are both strictly above zero. NaN and
/// signed zero fail the strict comparison on both sides, matching the
/// original's `comiss`/`jbe` shape.
///
/// Original: 0x00A4DAC0 (thiscall, no stack words), one float callee.
lf_checker_rt::export!(thiscall, rw_00A4DAC0(this: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x0F1D;
        const MASK: u8 = 0x02;
        const ARG: u32 = 0x10D0;
        const VAL: u32 = 0x10D8;
        const MEASURE_CALLEE: u32 = 1;
        if ((this + FLAG) as *const u8).read() & MASK == 0 {
            return 0;
        }
        let r: f32 =
            lf_checker_rt::callee_thiscall!(MEASURE_CALLEE, f32, this + ARG);
        if r > 0.0 {
            let v = f32::from_bits(
                ((this + VAL) as *const u32).read_unaligned(),
            );
            if v > 0.0 {
                return 1;
            }
        }
        0
    }
});
