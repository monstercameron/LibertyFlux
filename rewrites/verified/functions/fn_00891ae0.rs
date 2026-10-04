// original: 0x00891ae0 audSound_timing_update
/// Recompute this sound's playback window from its rate and position inputs.
///
/// Scales two optional float rate inputs by 1000 and truncates them toward
/// zero (SSE truncate for the first, x87 truncate-to-64-bit for the second,
/// keeping the low word) to advance two base counters. When the mode word
/// selects external timing, it asks the registered timing hook for a scale
/// factor; a -1 answer leaves the window at its default span. An optional
/// second helper refines the end counter. It then clamps the window
/// (start/end), applies the force-default and force-rescale option flags,
/// re-polls the timing hook when rescaling, stores the window, and returns
/// the status word the original leaves in EAX.
export!(thiscall, rw_00891ae0(sound: u32) -> u32 {
    unsafe {
        const RATE_SCALE: f32 = 1000.0;
        const INV_RATE: f32 = f32::from_bits(0x3c23d70a);
        // cvttss2si: truncate toward zero; NaN or out-of-range gives 0x80000000.
        fn cvt_trunc(x: f32) -> u32 {
            if x.is_nan() {
                return 0x80000000;
            }
            let t = x.trunc();
            if t < -2147483648.0 || t >= 2147483648.0 {
                0x80000000
            } else {
                (t as i32) as u32
            }
        }
        // fistp qword with truncation, low 32 bits kept. An invalid
        // conversion stores the qword indefinite 0x8000000000000000, whose
        // low dword (the only part the original reads) is zero.
        fn fistp_low(x: f32) -> u32 {
            if x.is_nan() {
                return 0;
            }
            let t = (x.trunc()) as f64;
            if t < -9223372036854775808.0 || t >= 9223372036854775808.0 {
                0
            } else {
                (t as i64) as u32
            }
        }
        let hook_target = |obj: u32| -> u32 {
            let idx = *((obj + 0x3b) as *const u8) as u32;
            *global::<u32>(0x115d714 + idx * 4)
        };
        let hook_call = |tgt: u32, obj: u32| -> u32 {
            let f: extern "cdecl" fn(u32, u32) -> u32 =
                core::mem::transmute(tgt as usize);
            f(obj, 0)
        };
        let rate_ptr = *((sound + 0x5c) as *const u32);
        let mut base = *((sound + 0x58) as *const u32);
        if rate_ptr != 0 {
            let f = (rate_ptr as *const f32).read_unaligned();
            base = base.wrapping_add(cvt_trunc(f * RATE_SCALE));
        }
        let pos_ptr = *((sound + 0x64) as *const u32);
        let mut span = *((sound + 0x68) as *const u32);
        if pos_ptr != 0 {
            let f = (pos_ptr as *const f32).read_unaligned();
            span = span.wrapping_add(fistp_low(f * RATE_SCALE));
        }
        let saved = *((sound + 0x60) as *const u32);
        let mut edge = *((sound + 0x6c) as *const u32);
        let mode = *((sound + 0x70) as *const u32) & 3;
        if mode == 1 {
            let answer = hook_call(hook_target(sound), sound);
            let scaled = ((span as f64) as f32) * INV_RATE * ((answer as i32) as f32);
            if answer == 0xffffffff {
                let mut out = saved.wrapping_add(base);
                *((sound + 0x54) as *mut u32) = 0;
                if *((sound + 8) as *const u8) & 8 != 0 {
                    out = base;
                }
                *((sound + 0x50) as *mut u32) = out;
                return out;
            }
            span = fistp_low(scaled);
        }
        if *((sound + 0x39) as *const u8) & 0x10 != 0 {
            let tune: u32 = callee_thiscall!(2, u32, sound, 0);
            let scaled = ((edge as f64) as f32) * INV_RATE * ((tune as i32) as f32);
            edge = fistp_low(scaled);
        }
        let mut start = saved;
        if edge >= base {
            edge = edge.wrapping_sub(base).wrapping_add(span);
            *((sound + 0x54) as *mut u32) = edge;
        } else {
            start = start.wrapping_sub(edge).wrapping_add(base);
            *((sound + 0x54) as *mut u32) = span;
        }
        *((sound + 0x50) as *mut u32) = start;
        let flags = *((sound + 8) as *const u8);
        if flags & 8 != 0 {
            *((sound + 0x50) as *mut u32) = base;
        }
        if flags & 0x10 == 0 {
            return (start & 0xffffff00) | (flags as u32);
        }
        let mode2 = *((sound + 0x70) as *const u32) & 3;
        if mode2 != 1 {
            *((sound + 0x54) as *mut u32) = span;
            return mode2;
        }
        let answer = hook_call(hook_target(sound), sound);
        let scaled = ((span as f64) as f32) * INV_RATE * ((answer as i32) as f32);
        if answer == 0xffffffff {
            let mut out = saved.wrapping_add(base);
            *((sound + 0x54) as *mut u32) = 0;
            if flags & 8 != 0 {
                out = base;
            }
            *((sound + 0x50) as *mut u32) = out;
            return out;
        }
        span = fistp_low(scaled);
        *((sound + 0x54) as *mut u32) = span;
        // Leftover: the original's EAX here is the x87 control word with the
        // truncate bits set (fninit default 0x37f | 0xc00), always 0xf7f.
        0xf7f
    }
});
