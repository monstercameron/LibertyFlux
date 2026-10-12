// original: 0x00647010 rage::ptxEmitRuleStd::vf23

/// Sample the four-lane keyframe curve at time `t`: scan the key rows
/// (0x30 bytes each) at `this + ROWS` for the first key at or past `t`
/// (ordered float compare, so a not-a-number key never matches) and
/// interpolate within the preceding segment; when no key reaches `t` the
/// last row is used, and with one key or none the first row is. Each lane
/// is `d * slope + base` with `d = t - key` in the original's operand
/// order (lane 0 multiplies `d` first, lanes 1-3 multiply the slope
/// first), stored to `out`. When `cb` is non-null the sample is reported
/// through the sampler callee with lane tag 7. Thiscall, four stack
/// arguments; returns the sampler's answer, or `out` when no callback runs.
lf_checker_rt::export!(thiscall, rw_00647010(this: u32, out: u32, t_bits: u32, a3: u32, cb: u32) -> u32 {
    unsafe {
        const ROWS: u32 = 0x17c;
        const COUNT: u32 = 0x180;
        const ROW_STRIDE: u32 = 0x30;
        const LANE_TAG: u32 = 7;
        const SAMPLE_CALLEE: u32 = 1;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let t = f32::from_bits(t_bits);
        let count =
            ((this.wrapping_add(COUNT)) as *const u16).read_unaligned() as u32;
        let rows = read_word(this.wrapping_add(ROWS));
        let mut index = 1u32;
        if count > 1 {
            let mut probe = rows.wrapping_add(ROW_STRIDE);
            while index < count {
                let key = f32::from_bits(read_word(probe));
                if key >= t {
                    break;
                }
                index = index.wrapping_add(1);
                probe = probe.wrapping_add(ROW_STRIDE);
            }
        }
        let entry = rows
            .wrapping_add(index.wrapping_mul(ROW_STRIDE))
            .wrapping_sub(ROW_STRIDE);
        let delta = sub(t, f32::from_bits(read_word(entry)));
        let lane0 = add(
            mul(delta, f32::from_bits(read_word(entry.wrapping_add(0x20)))),
            f32::from_bits(read_word(entry.wrapping_add(0x10))),
        );
        let lane1 = add(
            mul(f32::from_bits(read_word(entry.wrapping_add(0x24))), delta),
            f32::from_bits(read_word(entry.wrapping_add(0x14))),
        );
        let lane2 = add(
            mul(f32::from_bits(read_word(entry.wrapping_add(0x28))), delta),
            f32::from_bits(read_word(entry.wrapping_add(0x18))),
        );
        let lane3 = add(
            mul(f32::from_bits(read_word(entry.wrapping_add(0x2c))), delta),
            f32::from_bits(read_word(entry.wrapping_add(0x1c))),
        );
        write_word(out, lane0.to_bits());
        write_word(out.wrapping_add(4), lane1.to_bits());
        write_word(out.wrapping_add(8), lane2.to_bits());
        write_word(out.wrapping_add(12), lane3.to_bits());
        if cb != 0 {
            // Five stack words: the pushed callback slot is overwritten by
            // the time bits before the call, so `cb` travels in ecx only.
            lf_checker_rt::callee_thiscall!(
                SAMPLE_CALLEE, u32, cb, 0, out, LANE_TAG, t_bits, a3
            )
        } else {
            out
        }
    }
});
