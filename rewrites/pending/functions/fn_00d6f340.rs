// original: 0x00D6F340 replay_time_split
/// Split a stored replay timestamp into minutes, seconds and centiseconds.
///
/// The object holds four second-counts; two flag bytes select which one is
/// split: both set reads the first, only the first flag reads the third,
/// only the second flag reads the second, neither reads the fourth. The
/// chosen value is divided by 60 for whole minutes, the remainder (computed
/// against minutes times 60 so it stays exact for large counts) is
/// truncated for whole seconds, and the leftover fraction times 100 is
/// truncated for centiseconds. Each result is stored through its out
/// pointer. All truncation rounds toward zero, matching the original's
/// float-to-integer conversion exactly, including out-of-range inputs.
export!(thiscall, rw_00d6f340(this: u32, out_min: u32, out_sec: u32, out_centi: u32, flag_a: u32, flag_b: u32) -> u32 {
    unsafe {
        /// 1/60: whole minutes in a second-count.
        const INV_SECONDS_PER_MINUTE: u32 = 0x00FE8724;
        /// 60.0: seconds per minute.
        const SECONDS_PER_MINUTE: u32 = 0x00FE8B80;
        /// 100.0: centiseconds per second.
        const CENTIS_PER_SECOND: u32 = 0x00FE8BB0;

        /// Truncate toward zero to 32 bits, matching the original's
        /// convert-with-chop then load-low-dword sequence: NaN and
        /// magnitudes at or above 2^63 produce the indefinite value
        /// whose low dword is zero.
        fn trunc_low32(x: f32) -> u32 {
            const TWO63: f32 = 9223372036854775808.0;
            if x.is_nan() || x >= TWO63 || x <= -TWO63 {
                0
            } else {
                (x as i64) as u32
            }
        }
        /// The original's integer-to-float path converts the stored word
        /// as an unsigned 32-bit value through a wider float and back.
        fn word_to_f32(w: u32) -> f32 {
            (w as f64) as f32
        }
        /// Scalar multiply with forced operand order (see the progress-bar
        /// rewrite: plain `*` lets LLVM commute operands, which changes
        /// NaN payloads on some hosts; the call boundary keeps the order).
        #[inline(never)]
        fn fmul(a: f32, b: f32) -> f32 {
            a * b
        }

        let first = (flag_a as u8) != 0;
        let second = (flag_b as u8) != 0;
        let sel = if first {
            if second {
                *((this.wrapping_add(8)) as *const f32)
            } else {
                *((this.wrapping_add(0x10)) as *const f32)
            }
        } else if second {
            *((this.wrapping_add(0x0C)) as *const f32)
        } else {
            *((this.wrapping_add(0x14)) as *const f32)
        };
        let per_min = *(relocated(INV_SECONDS_PER_MINUTE) as *const f32);
        let sec_per_min = *(relocated(SECONDS_PER_MINUTE) as *const f32);
        let centi_per_sec = *(relocated(CENTIS_PER_SECOND) as *const f32);
        let minutes = trunc_low32(fmul(sel, per_min));
        *(out_min as *mut u32) = minutes;
        let rest = sel - fmul(word_to_f32(minutes), sec_per_min);
        let seconds = trunc_low32(rest);
        *(out_sec as *mut u32) = seconds;
        let centis = trunc_low32(fmul(rest - word_to_f32(seconds), centi_per_sec));
        *(out_centi as *mut u32) = centis;
        out_centi
    }
});
