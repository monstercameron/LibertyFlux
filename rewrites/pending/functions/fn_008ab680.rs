// original: 0x008AB680 audio_slew_follow
/// Slew-limited follower: silent while disabled, snap to the input on the
/// first tick after arming, then clamp the input within asymmetric rates
/// around the previous output, preferring the lower bound on small steps,
/// and optionally clamping to an output range. All comparisons keep the
/// original's NaN behaviour (unordered counts as not-greater).
export!(thiscall, rw_008AB680(obj: *mut u8, input: f32) -> f32 {
    unsafe {
        let flags = obj.add(0x18);
        let mode = *flags;
        if mode & 1 == 0 {
            return 0.0;
        }
        let cell = |o: usize| (obj.add(o) as *const f32);
        if mode & 2 != 0 {
            *flags = mode & 0xFD;
            *(obj.add(0x10) as *mut f32) = input;
            return input;
        }
        let prev = *cell(0x10);
        let upper = prev + *cell(0);
        let mut out = if upper > input { input } else { upper };
        let lower = prev - *cell(4);
        let floored = if input > lower { input } else { lower };
        let step = input - prev;
        let threshold = *(global::<f32>(0x00FE8628) as *const f32);
        if !(step >= threshold) {
            out = floored;
        }
        if mode & 4 != 0 {
            let hi = *cell(8);
            if !(hi > out) {
                out = hi;
            }
            let lo = *cell(0x0C);
            if !(out > lo) {
                out = lo;
            }
        }
        *(obj.add(0x10) as *mut f32) = out;
        out
    }
});
