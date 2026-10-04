// original: 0x00a96340 fade_channel_state
/// Classifies the fade channel into one of four states (0-3).
///
/// A non-zero state word forces 0. Otherwise a readiness gate selects
/// between the float-threshold path (comparing the measured level against
/// 1.0 and 0.0) and a sub-state dispatch that combines the stored sub-state
/// with a second readiness probe. NaN levels fall through to the dispatch.
export!(thiscall, rw_00a96340(this: u32) -> u32 {
    unsafe {
        if *(this as *const u32) != 0 {
            return 0;
        }
        let gated = callee_thiscall!(1, u32, this) & 0xFF != 0;
        let mut middle = !gated;
        if gated {
            let r = *((this + 0x14) as *const f32);
            if r >= 1.0 {
                middle = true;
            }
        }
        if middle {
            let f: f32 = callee_thiscall!(2, f32, this);
            let ok = callee_thiscall!(3, u32, this) & 0xFF != 0;
            if ok {
                if f >= 1.0 {
                    return 2;
                }
                if f <= 0.0 {
                    return 0;
                }
            } else {
                if f >= 1.0 {
                    return 0;
                }
                if 0.0 >= f {
                    return 2;
                }
            }
        }
        let s = callee_thiscall!(4, u32, this);
        if s == 3 {
            let a = callee_thiscall!(3, u32, this);
            return if a & 0xFF != 0 { 3 } else { 1 };
        } else if s == 1 {
            let a = callee_thiscall!(3, u32, this);
            return if a & 0xFF == 0 { 3 } else { 1 };
        } else {
            let a = callee_thiscall!(3, u32, this);
            return if a & 0xFF != 0 { 2 } else { 0 };
        }
    }
});
