// original: 0x0086ea10 WAITUNPAUSED
/// Wait like `WAIT`, but do not accumulate time while paused.
///
/// State machine on the current script thread (`+0xC`: 1 means waiting,
/// `+0x28`: accumulated milliseconds as a float):
///
/// * If the thread is not waiting, the accumulator is cleared; a thread that
///   was idle (state 0) enters the waiting state, any other state returns.
/// * A negative accumulator ends the wait immediately.
/// * While the global pause flag is set, nothing accumulates and the wait
///   continues.
/// * Otherwise one frame's scaled time is added; the wait ends once the
///   truncated accumulator reaches the requested milliseconds (argument 0).
///
/// The portable parts are plain Rust; two x86 artifacts of the original are
/// reproduced for bitwise equality: the `cvttss2si` conversion
/// (out-of-range and NaN yield `0x80000000`, unlike Rust's saturating `as`
/// cast) and the merge of the still-waiting flag into the low byte of the
/// truncated time left in `eax`.
export!(cdecl, rw_0086ea10(ctx: u32) -> u32 {
    unsafe {
        let thread = *global::<u32>(0x01BB54DC) as *mut u8;
        let state = *(thread.add(0xC) as *mut u32);
        if state != 1 {
            *(thread.add(0x28) as *mut u32) = 0;
            if state != 0 {
                return state;
            }
            *(thread.add(0xC) as *mut u32) = 1;
            return 0;
        }
        let acc = *(thread.add(0x28) as *const f32);
        if 0.0f32 > acc {
            *(thread.add(0xC) as *mut u32) = 0;
            return 1;
        }
        if *global::<u8>(0x018B9B40) != 0 {
            return 1;
        }
        let frame = *global::<f32>(0x0110B730);
        let scale = *global::<f32>(0x00FE8C58);
        let new_acc = frame * scale + acc;
        *(thread.add(0x28) as *mut f32) = new_acc;
        let args = *(ctx as *const u32).add(2) as *const u32;
        let wait_ms = *args as i32;
        // Exact `cvttss2si`: truncate toward zero; anything that does not fit
        // (including NaN and infinities) yields 0x80000000.
        let wide = new_acc as f64;
        let elapsed: i32 = if new_acc.is_nan() || wide >= 2147483648.0 || wide <= -2147483649.0 {
            0x8000_0000u32 as i32
        } else {
            new_acc as i32
        };
        let still_waiting = u32::from(elapsed < wait_ms);
        // The original merges the flag into `al`, keeping the truncated
        // time's high bytes in `eax`.
        let merged = (elapsed as u32 & 0xFFFF_FF00) | still_waiting;
        if still_waiting != 0 {
            return merged;
        }
        *(thread.add(0xC) as *mut u32) = 0;
        merged
    }
});
