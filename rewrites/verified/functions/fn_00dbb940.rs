// original: 0x00dbb940 ui_montage_time_text_update (proposed)

/// Split a running-time value in seconds into minutes, seconds and
/// hundredths, refresh three child text items and the panel's own text.
///
/// `this` points to the panel object. The child item pointers live at
/// `+0x1ec`, `+0x1f0` and `+0x1f4`, the panel's own text item at `+0x1e4`.
/// The float argument (stack word, seconds) is decomposed with the
/// original's exact operation order: minutes = trunc(seconds / 60),
/// seconds = trunc(seconds - float(minutes) * 60),
/// hundredths = trunc((seconds - float(minutes) * 60 - float(seconds)) * 100).
/// Each truncation is x87 `fistp` with the round-control bits forced to
/// truncate; NaN, infinities and out-of-range values yield low word 0 as
/// observed on the checker worker. The back-conversion treats the
/// truncated word as unsigned through a double. The minute count is then
/// compared (unsigned) against a global tick counter divided by 30: when it
/// is larger the three children are enabled with flag 1 and the highlight
/// colour is used, otherwise they get flag 0 and the dim colour. The colour
/// word is handed to the text item's slot `0x208`, the three parts are
/// formatted as two-digit fields into a stack buffer by the shared
/// format callee, and the buffer is handed to slot `0x1e0` with flag 0.
///
/// Original: 0x00dbb940 (thiscall, one stack word: the seconds as f32 bits).
lf_checker_rt::export!(thiscall, rw_00dbb940(this: u32, arg: u32) -> u32 {
    unsafe {
        const SECONDS_PER_MINUTE: f32 = 60.0;
        const INV_SECONDS_PER_MINUTE: f32 = f32::from_bits(0x3c888889); // 1/60
        const HUNDRED: f32 = 100.0;
        const TWO63: f32 = 9223372036854775808.0;
        const DIV30_MAGIC: i64 = 0x88888889u32 as i32 as i64;
        const TICKS_GLOBAL: u32 = 0x0106c284;
        const TIME_FORMAT: u32 = 0x00ef3d9c;
        const CHILD_MINUTES: u32 = 0x1ec;
        const CHILD_SECONDS: u32 = 0x1f0;
        const CHILD_HUNDREDTHS: u32 = 0x1f4;
        const OWN_TEXT: u32 = 0x1e4;
        const SLOT_ENABLE: u32 = 0x120;
        const SLOT_SET_COLOUR: u32 = 0x208;
        const SLOT_SET_TEXT: u32 = 0x1e0;
        const COLOUR_HIGHLIGHT: u32 = 0xffc73103;
        const COLOUR_DIM: u32 = 0xff6f6f6f;
        const CALLEE_FORMAT: u32 = 4;
        const CALLEE_COOKIE: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        /// x87 `fistp` to a 64-bit integer with truncation, low dword kept:
        /// NaN, infinities and magnitudes at or above 2^63 yield low word
        /// 0 as observed on the checker worker; anything else truncates
        /// toward zero.
        fn trunc_dw(x: f32) -> u32 {
            if !x.is_finite() || x < -TWO63 || x >= TWO63 {
                0
            } else {
                (x as i64) as u32
            }
        }

        /// The original's dword-to-float path: treated as unsigned through
        /// an exactly representable double, narrowed once to float.
        fn back_to_float(u: u32) -> f32 {
            (u as f64) as f32
        }

        unsafe fn vtable_slot(obj: u32, slot: u32) -> u32 {
            unsafe { rd32(rd32(obj) + slot) }
        }

        let seconds = f32::from_bits(arg);
        let minutes = trunc_dw(mul(seconds, INV_SECONDS_PER_MINUTE));
        let rest = sub(seconds, mul(back_to_float(minutes), SECONDS_PER_MINUTE));
        let secs = trunc_dw(rest);
        let frac = sub(rest, back_to_float(secs));
        let hundredths = trunc_dw(mul(frac, HUNDRED));

        // Signed divide of the global tick counter by 30 through the
        // original's multiply-and-shift sequence.
        let ticks = rd32(lf_checker_rt::relocated(TICKS_GLOBAL)) as i32;
        let prod = DIV30_MAGIC.wrapping_mul(ticks as i64);
        let mut q = (prod >> 32) as i32;
        q = q.wrapping_add(ticks);
        q >>= 5;
        q = q.wrapping_add((q as u32 >> 31) as i32);

        let highlight = minutes > q as u32;
        let flag: u32 = if highlight { 1 } else { 0 };
        let mut colour: u32 = if highlight {
            COLOUR_HIGHLIGHT
        } else {
            COLOUR_DIM
        };
        for off in [CHILD_MINUTES, CHILD_SECONDS, CHILD_HUNDREDTHS] {
            let child = rd32(this + off);
            let enable: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(vtable_slot(child, SLOT_ENABLE) as usize);
            enable(child, flag);
        }
        let text = rd32(this + OWN_TEXT);
        let set_colour: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(vtable_slot(text, SLOT_SET_COLOUR) as usize);
        set_colour(text, &mut colour as *mut u32 as u32);

        let mut buf = [0u32; 8];
        lf_checker_rt::callee_cdecl!(
            CALLEE_FORMAT,
            u32,
            buf.as_mut_ptr() as u32,
            lf_checker_rt::relocated(TIME_FORMAT),
            minutes,
            secs,
            hundredths
        );

        let set_text: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(vtable_slot(text, SLOT_SET_TEXT) as usize);
        set_text(text, buf.as_mut_ptr() as u32, 0);

        lf_checker_rt::callee_cdecl!(CALLEE_COOKIE, u32,);
        0
    }
});
