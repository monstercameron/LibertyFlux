// original: 0x0086e940 WAIT
/// Native handler `WAIT`.
///
/// Advance the script sleep timer: while the timer state word is 1, add one
/// scaled frame step to the accumulated time and keep waiting until the
/// truncated total reaches the script's frame count argument. Any other state
/// resets the accumulator; state 0 additionally arms the timer.
///
/// The state word lives at offset 0xC and the accumulated time (a float) at
/// offset 0x28 of the script state block pointed to by a global. The frame
/// step is the product of two float globals. The truncation matches
/// CVTTSS2SI exactly (NaN or out-of-range yields 0x80000000); the limit
/// comparison is signed.
///
/// `ctx` is the native call context: word 2 points at the script argument
/// array, whose first word is the frame count to wait.
lf_rn14_rt::export!(cdecl, rw_0086e940(ctx: u32) -> u32 {
    unsafe {
        let state = *lf_rn14_rt::global::<u32>(0x1BB54DC) as *mut u32;
        let mode = *state.add(3);
        if mode != 1 {
            *(state as *mut f32).add(10) = 0.0;
            if mode != 0 {
                return mode;
            }
            *state.add(3) = 1;
            return 0;
        }
        let acc = *(state as *const f32).add(10);
        if 0.0f32 > acc {
            *state.add(3) = 0;
            return 1;
        }
        let step = *lf_rn14_rt::global::<f32>(0x110B730)
            * *lf_rn14_rt::global::<f32>(0xFE8C58);
        let total = step + acc;
        *(state as *mut f32).add(10) = total;
        let args = *((ctx + 8) as *const u32) as *const u32;
        // CVTTSS2SI semantics: truncate toward zero; NaN or a value outside
        // the i32 range yields 0x80000000. (Rust's `as` saturates instead,
        // so the out-of-range cases are handled explicitly.)
        let ticks = if total.is_nan() || total >= 2_147_483_648.0 || total < -2_147_483_648.0 {
            0x8000_0000u32 as i32
        } else {
            total as i32
        };
        let flag = u32::from(ticks < *args as i32);
        // The original merges the flag into the low byte of EAX (setl al).
        let merged = (ticks as u32 & 0xFFFF_FF00) | flag;
        if flag != 0 {
            return merged;
        }
        *state.add(3) = 0;
        merged
    }
});
