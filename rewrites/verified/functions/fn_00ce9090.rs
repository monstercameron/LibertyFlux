// original: 0x00ce9090 CTaskSimpleNMFlinch::vf26 (symbols)
/// NaturalMotion "flinch" task update: build and send up to three behaviour
/// messages, then run an optional follow-up call.
///
/// `this` is the task object: the style index at `+0x28` selects a 56-byte
/// entry in the parameter table, `+0x2C` gates the trailing virtual call,
/// and the computed timer lands at `+0x44`. `arg` is the owner object:
/// `+0x20` points at a position block, `+0x7B4` is the object handed to
/// every send call.
///
/// Behaviour in order. First message: log the style, copy three position
/// words through the position helper into a stack buffer, append them as a
/// vector parameter, then three table floats; draw a random number and
/// append its below-threshold bit plus the inverted bit; append four more
/// table floats and one table integer; send. Second message: a fresh buffer
/// with one integer parameter, plus, only for style zero, a randomised
/// integer blended from two table words; send and reset. Third part, only
/// when the table's flag byte for this style is set: read the position
/// helper again, form the squared distance of two coordinate differences,
/// and either take a square-root-scaled direction (square root from the
/// helper, three products) or copy a fallback direction, chosen by comparing
/// the squared distance against a threshold; send that as a second buffer
/// with two integers, a vector and a float; blend another randomised timer
/// from the table into `+0x44`; reset the buffer. Finally, when `+0x2C` is
/// set, call the virtual slot at `+0x6C` with the owner object, and reset
/// the first buffer.
///
/// Two details matter for bit-exactness. The distance compare is an
/// unordered-aware below-or-equal: NaN takes the fallback side. One stack
/// word the third part reads is never written at that address (it still
/// holds the last word the position helper wrote into the first part's
/// buffer), but the lane it lands in is never read afterwards, so the
/// rewrite omits it. The
/// float-to-int conversions truncate toward zero and yield 0x80000000 for
/// NaN, infinities and out-of-range values, matching the hardware
/// instruction (a plain cast saturates instead and would differ).
///
/// Original: 0x00CE9090 (thiscall, one stack word, callee pops 4).
/// Shared body; `mut_flip` inverts the distance branch (mutant control).
unsafe fn flinch_inner(this: u32, arg: u32, mut_flip: bool) -> u32 {
    unsafe {
        const STYLE_OFF: u32 = 0x28;
        const TIMER_OFF: u32 = 0x44;
        const VT_GATE_OFF: u32 = 0x2C;
        const ARG_POS: u32 = 0x20;
        const ARG_NM: u32 = 0x7B4;
        const VT_SLOT: u32 = 0x6C;
        const ENTRY_STRIDE: u32 = 56;
        const MSG_BUF_LEN: usize = 0xCC4;
        const POS_WORDS: usize = 4;
        const STYLE_TABLE: u32 = 0x0171CE00;
        const STYLE_NAMES: u32 = 0x010521A4;
        const LOG_TAG: u32 = 0x00EDCA44;
        const MSG_OPEN: u32 = 0x01051CC8;
        const RNG_THRESH: i32 = 0x3FFF;
        /// (table slot, name global) for the first message's three floats.
        const FIRST_FLOATS: [(u32, u32); 3] = [
            (0x18, 0x01051F24), (0x1C, 0x01051F28), (0x24, 0x01051F2C),
        ];
        const FIRST_VEC_NAME: u32 = 0x01051F0C;
        const FIRST_BIT_NAME: u32 = 0x01051F10;
        const FIRST_NBIT_NAME: u32 = 0x01051F14;
        /// (table slot, name global) for the first message's last floats.
        const LAST_FLOATS: [(u32, u32); 4] = [
            (0x28, 0x01051F18), (0x2C, 0x01051F1C),
            (0x30, 0x01051F20), (0x44, 0x01051F38),
        ];
        const LAST_INT_NAME: u32 = 0x01051F40;
        const LAST_INT_SLOT: u32 = 0x48;
        const FIRST_SEND_NAME: u32 = 0x01051F04;
        const SECOND_INT_NAME: u32 = 0x01051E14;
        const SECOND_SEND_NAME: u32 = 0x01051DFC;
        const FLAG_SLOT: u32 = 0x4C;
        const THIRD_VEC_NAME: u32 = 0x01051E90;
        const THIRD_INT_NAME: u32 = 0x01051E98;
        const THIRD_FLOAT_NAME: u32 = 0x01051E94;
        const THIRD_FLOAT_SLOT: u32 = 0x20;
        const THIRD_SEND_NAME: u32 = 0x01051E88;
        const LERP_SCALE: u32 = 0x00FE8680;
        const DIST_THRESH: u32 = 0x00FE870C;
        const TIMER_BASE: u32 = 0x011735B4;
        const CAL_INIT: u32 = 1;
        const CAL_ADD_INT: u32 = 2;
        const CAL_ADD_FLOAT: u32 = 3;
        const CAL_ADD_VEC: u32 = 4;
        const CAL_SEND: u32 = 5;
        const CAL_RESET: u32 = 6;
        const CAL_LOG: u32 = 7;
        const CAL_GETPOS: u32 = 8;
        const CAL_RNG: u32 = 9;
        const CAL_SQRT: u32 = 10;
        const CAL_ADD_INT2: u32 = 11;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn g32(file_va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(file_va).read() }
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        /// Truncating float-to-int conversion with x86 semantics.
        #[inline(always)]
        fn cvtt(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                0x80000000u32 as i32
            } else {
                x as i32
            }
        }

        let style = rd32(this + STYLE_OFF);
        let style_name = g32(STYLE_NAMES + style.wrapping_mul(4));
        lf_checker_rt::callee_cdecl!(CAL_LOG, u32, lf_checker_rt::relocated(LOG_TAG), style, style_name);

        let mut buf1 = [0u8; MSG_BUF_LEN];
        let bp1 = buf1.as_mut_ptr() as u32;
        lf_checker_rt::callee_thiscall!(CAL_INIT, u32, bp1);
        lf_checker_rt::callee_thiscall!(CAL_ADD_INT, u32, bp1, g32(MSG_OPEN), 1);

        let mut pos_a = [0u32; POS_WORDS];
        let pa = pos_a.as_mut_ptr() as u32;
        lf_checker_rt::callee_thiscall!(CAL_GETPOS, u32, this, pa, arg);
        lf_checker_rt::callee_thiscall!(
            CAL_ADD_VEC, u32, bp1, g32(FIRST_VEC_NAME),
            pos_a[0], pos_a[1], pos_a[2],
        );

        let entry = STYLE_TABLE + style.wrapping_mul(ENTRY_STRIDE);
        let mut i = 0;
        while i < 3 {
            let (slot, name) = FIRST_FLOATS[i];
            lf_checker_rt::callee_thiscall!(CAL_ADD_FLOAT, u32, bp1, g32(name), g32(entry + slot));
            i += 1;
        }
        let r1: u32 = lf_checker_rt::callee_cdecl!(CAL_RNG, u32,);
        let bit = ((r1 as i32) < RNG_THRESH) as u32;
        lf_checker_rt::callee_thiscall!(CAL_ADD_INT, u32, bp1, g32(FIRST_BIT_NAME), bit);
        lf_checker_rt::callee_thiscall!(CAL_ADD_INT, u32, bp1, g32(FIRST_NBIT_NAME), (bit == 0) as u32);
        let mut j = 0;
        while j < 4 {
            let (slot, name) = LAST_FLOATS[j];
            lf_checker_rt::callee_thiscall!(CAL_ADD_FLOAT, u32, bp1, g32(name), g32(entry + slot));
            j += 1;
        }
        lf_checker_rt::callee_thiscall!(
            CAL_ADD_INT2, u32, bp1, g32(LAST_INT_NAME),
            g32(entry + LAST_INT_SLOT),
        );
        let nm = rd32(arg + ARG_NM);
        lf_checker_rt::callee_thiscall!(CAL_SEND, u32, nm, g32(FIRST_SEND_NAME), bp1);

        let mut buf2 = [0u8; MSG_BUF_LEN];
        let bp2 = buf2.as_mut_ptr() as u32;
        lf_checker_rt::callee_thiscall!(CAL_INIT, u32, bp2);
        lf_checker_rt::callee_thiscall!(CAL_ADD_INT, u32, bp2, g32(MSG_OPEN), 1);
        if style == 0 {
            let hi = g32(STYLE_TABLE + 0x38);
            let lo = g32(STYLE_TABLE + 0x34);
            let r2: u32 = lf_checker_rt::callee_cdecl!(CAL_RNG, u32,);
            let scale = f32::from_bits(g32(LERP_SCALE));
            let t = fmul((r2 & 0xFFFF) as f32, scale);
            let span = fmul((hi.wrapping_sub(lo) as i32) as f32, t);
            let v = cvtt(span).wrapping_add(lo as i32) as u32;
            lf_checker_rt::callee_thiscall!(CAL_ADD_INT2, u32, bp2, g32(SECOND_INT_NAME), v);
        }
        lf_checker_rt::callee_thiscall!(CAL_SEND, u32, nm, g32(SECOND_SEND_NAME), bp2);
        lf_checker_rt::callee_thiscall!(CAL_RESET, u32, bp2);

        let flag = (lf_checker_rt::relocated(STYLE_TABLE) + style.wrapping_mul(ENTRY_STRIDE) + FLAG_SLOT)
            as *const u8;
        if flag.read() != 0 {
            let mut pos_c = [0u32; POS_WORDS];
            let pc = pos_c.as_mut_ptr() as u32;
            lf_checker_rt::callee_thiscall!(CAL_GETPOS, u32, this, pc, arg);
            let base = rd32(arg + ARG_POS);
            let d1 = fsub(
                f32::from_bits(rd32(base + 0x30)),
                f32::from_bits(pos_c[0]),
            );
            let d2 = fsub(
                f32::from_bits(rd32(base + 0x34)),
                f32::from_bits(pos_c[1]),
            );
            let q = fadd(fmul(d2, d2), fmul(d1, d1));
            let thresh = f32::from_bits(g32(DIST_THRESH));
            // Unordered-aware below-or-equal: NaN takes the fallback side.
            // (The fourth lane the original also stores is never read
            // afterwards, so only the three vector lanes are rebuilt.)
            let take_fallback = (!(q > thresh)) != mut_flip;
            let (v0, v1, v2) = if take_fallback {
                (
                    f32::from_bits(rd32(base + 0x10)),
                    f32::from_bits(rd32(base + 0x14)),
                    f32::from_bits(rd32(base + 0x18)),
                )
            } else {
                let s = f32::from_bits(lf_checker_rt::callee_cdecl!(CAL_SQRT, u32, q.to_bits()));
                (fmul(s, d1), fmul(s, d2), fmul(s, 0.0))
            };
            lf_checker_rt::callee_thiscall!(CAL_INIT, u32, bp2);
            lf_checker_rt::callee_thiscall!(CAL_ADD_INT, u32, bp2, g32(MSG_OPEN), 1);
            lf_checker_rt::callee_thiscall!(CAL_ADD_INT, u32, bp2, g32(THIRD_INT_NAME), 0);
            lf_checker_rt::callee_thiscall!(
                CAL_ADD_VEC, u32, bp2, g32(THIRD_VEC_NAME),
                v0.to_bits(), v1.to_bits(), v2.to_bits(),
            );
            lf_checker_rt::callee_thiscall!(
                CAL_ADD_FLOAT, u32, bp2, g32(THIRD_FLOAT_NAME),
                g32(entry + THIRD_FLOAT_SLOT),
            );
            lf_checker_rt::callee_thiscall!(CAL_SEND, u32, nm, g32(THIRD_SEND_NAME), bp2);
            let t0 = g32(TIMER_BASE);
            let hi = g32(entry + 0x40);
            let lo = g32(entry + 0x3C);
            let r3: u32 = lf_checker_rt::callee_cdecl!(CAL_RNG, u32,);
            let scale = f32::from_bits(g32(LERP_SCALE));
            let t = fmul((r3 & 0xFFFF) as f32, scale);
            let span = fmul(t, (hi.wrapping_sub(lo) as i32) as f32);
            let timer = cvtt(span)
                .wrapping_add(lo as i32)
                .wrapping_add(t0 as i32) as u32;
            wr32(this + TIMER_OFF, timer);
            lf_checker_rt::callee_thiscall!(CAL_RESET, u32, bp2);
        }

        if rd32(this + VT_GATE_OFF) != 0 {
            let hook: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(this) + VT_SLOT) as usize);
            hook(this, arg);
        }
        lf_checker_rt::callee_thiscall!(CAL_RESET, u32, bp1);
        0
    }
}

lf_checker_rt::export!(thiscall, rw_00ce9090(this: u32, arg: u32) -> u32 {
    unsafe { flinch_inner(this, arg, false) }
});
