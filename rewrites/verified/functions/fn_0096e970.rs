// original: 0x0096e970 audio_emitter_distance_update (proposed)

/// Update audio emitters by distance to the listener, notifying the audio
/// manager about each emitter loud enough to matter.
///
/// `this` points to the emitter set: a count at `+COUNT_OFF`, a table of
/// emitter pointers at `+EMITTER_TAB` and a parallel table of flag-object
/// pointers at `+FLAG_TAB` (one pair per index). `scale` is a gain applied
/// to every emitter, `listen_pos` points at three floats (the listener
/// position) and `max_dist` is the distance at which an emitter goes
/// silent.
///
/// For each index below the count (re-read every iteration), up to
/// `MAX_CALLS` notifications are sent: the loop stops once five have been
/// made. An index is skipped when either pointer is null or when the flag
/// object's byte at `+FLAG_BYTE` has none of the `FLAG_MASK` bits set. The
/// emitter position is three floats at `+POS_OFF`, or, when the dword at
/// `+ALT_PTR_OFF` is non-zero, at `+ALT_POS_OFF` past that pointer.
///
/// Loudness: `d` is the distance from the emitter to the listener,
/// `frac = min(d / max_dist, 1)` (a NaN quotient stays NaN and yields a NaN
/// weight) and `weight = (1 - frac) * scale`. Emitter is skipped unless
/// `weight >= MIN_WEIGHT`. Otherwise the manager is notified through the
/// fixed audio object with (flag object, weight bits, emitter, volume),
/// where `volume` is `(d * VOL_K1) * VOL_K2` truncated toward zero exactly
/// as `cvttss2si` does (out of range or NaN gives `0x80000000`).
///
/// Returns the number of notifications sent. Float operation order is the
/// original's (SSE scalar); comparisons reproduce `comiss`+`jbe`/`jb`
/// NaN behaviour (unordered takes the jump).
///
/// Original: 0x0096e970 (thiscall, three stack words, callee cleans 12).
lf_checker_rt::export!(thiscall, rw_0096e970(this: u32, scale: u32, listen_pos: u32, max_dist: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0x2d30;
        const EMITTER_TAB: u32 = 0x2a30;
        const FLAG_TAB: u32 = 0x2b30;
        const ALT_PTR_OFF: u32 = 0x20;
        const ALT_POS_OFF: u32 = 0x30;
        const POS_OFF: u32 = 0x10;
        const FLAG_BYTE: u32 = 5;
        const FLAG_MASK: u8 = 0x0c;
        const MAX_CALLS: u32 = 5;
        const AUDIO_OBJ: u32 = 0x012202e0;
        const K_ONE: u32 = 0x00fe88e8;
        const K_VOL1: u32 = 0x00e8b7f0;
        const K_MIN_W: u32 = 0x00fe87e4;
        const K_VOL2: u32 = 0x00fe8c58;
        const CALLEE_NOTIFY: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn kbits(va: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(va) as *const u32).read() }
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
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        /// `cvttss2si` semantics: truncate toward zero; NaN or out of range
        /// gives the integer-indefinite value (Rust `as` would saturate).
        #[inline(always)]
        fn cvtt(x: f32) -> u32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                0x80000000
            } else {
                (x as i32) as u32
            }
        }

        let one = f32::from_bits(kbits(K_ONE));
        let vol_k1 = f32::from_bits(kbits(K_VOL1));
        let min_w = f32::from_bits(kbits(K_MIN_W));
        let vol_k2 = f32::from_bits(kbits(K_VOL2));
        let gain = f32::from_bits(scale);
        let maxd = f32::from_bits(max_dist);
        let audio = lf_checker_rt::relocated(AUDIO_OBJ);

        let mut made: u32 = 0;
        let mut i: u32 = 0;
        while i < rd32(this.wrapping_add(COUNT_OFF)) {
            // `cmp made, 5; jae done`: at most five notifications.
            if !(made < MAX_CALLS) {
                break;
            }
            let emitter = rd32(this.wrapping_add(EMITTER_TAB).wrapping_add(i.wrapping_mul(4)));
            let flags = rd32(this.wrapping_add(FLAG_TAB).wrapping_add(i.wrapping_mul(4)));
            i = i.wrapping_add(1);
            if emitter == 0 || flags == 0 {
                continue;
            }
            if rd8(flags.wrapping_add(FLAG_BYTE)) & FLAG_MASK == 0 {
                continue;
            }
            let alt = rd32(emitter.wrapping_add(ALT_PTR_OFF));
            let pos = if alt != 0 {
                alt.wrapping_add(ALT_POS_OFF)
            } else {
                emitter.wrapping_add(POS_OFF)
            };
            let dx = sub(rdf(pos), rdf(listen_pos));
            let dy = sub(rdf(pos.wrapping_add(4)), rdf(listen_pos.wrapping_add(4)));
            let dz = sub(rdf(pos.wrapping_add(8)), rdf(listen_pos.wrapping_add(8)));
            let d2 = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
            let d = core::hint::black_box(d2).sqrt();
            let mut frac = div(d, maxd);
            // `comiss frac, 1; jbe keep`: unordered keeps frac, above clamps.
            if frac > one {
                frac = one;
            }
            let weight = mul(sub(one, frac), gain);
            // `comiss weight, min; jb skip`: unordered skips too.
            if !(weight >= min_w) {
                continue;
            }
            let vol = cvtt(mul(mul(d, vol_k1), vol_k2));
            made = made.wrapping_add(1);
            lf_checker_rt::callee_thiscall!(
                CALLEE_NOTIFY,
                u32,
                audio,
                flags,
                weight.to_bits(),
                emitter,
                vol
            );
        }
        made
    }
});
