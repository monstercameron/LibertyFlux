// original: 0x0086e9b0 WAITUNWARPED
/// Script native `WAITUNWARPED` (hash 0x771A298A).
///
/// Advances a time-warp wait state machine kept in engine memory: while the
/// mode word is 1 it accumulates a per-frame time step into a timer and
/// compares the truncated tick count against the script's limit argument,
/// clearing the mode once the limit is reached; any other mode resets the
/// timer, and mode 0 additionally arms the wait by setting mode 1. No
/// return slot is written; the exit register value is whatever the last
/// executed path left behind, reproduced exactly.
export!(cdecl, rw_0086e9b0(ctx: *const u8) -> u32 {
    unsafe {
        let state = *lf_k2_rt::global::<u32>(0x01bb54dc) as *mut u8;
        let mode = *(state.add(0x0c) as *const u32);
        if mode == 1 {
            let timer = *(state.add(0x28) as *const f32);
            if 0.0f32 > timer {
                *(state.add(0x0c) as *mut u32) = 0;
                1
            } else {
                let step = *lf_k2_rt::global::<f32>(0x00fe8b2c);
                let next = timer + step;
                *(state.add(0x28) as *mut f32) = next;
                let args = (*(ctx.add(8) as *const u32)) as *const u32;
                let limit = *args as i32;
                let ticks = cvttss2si(next);
                let below = u32::from(ticks < limit);
                // The original ends this path with `setl al`, so the exit
                // value is the tick count with its low byte replaced.
                let exit = (ticks as u32 & 0xFFFF_FF00) | below;
                if below == 0 {
                    *(state.add(0x0c) as *mut u32) = 0;
                }
                exit
            }
        } else {
            *(state.add(0x28) as *mut u32) = 0;
            if mode != 0 {
                mode
            } else {
                *(state.add(0x0c) as *mut u32) = 1;
                0
            }
        }
    }
});

/// Truncating float-to-int conversion with the x86 `cvttss2si` semantics:
/// round toward zero when the result fits in `i32`, otherwise `i32::MIN`
/// (this covers NaN and out-of-range magnitudes, where Rust's `as` cast
/// would saturate differently).
fn cvttss2si(x: f32) -> i32 {
    let t = x.trunc();
    if t >= -2147483648.0 && t < 2147483648.0 {
        t as i32
    } else {
        i32::MIN
    }
}
