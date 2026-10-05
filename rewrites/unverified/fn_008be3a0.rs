// original: 0x008BE3A0 input_float_and_state_poll (proposed)

/// Poll input floats through scripted readers, then run the state dispatch.
///
/// With two arguments (mode byte, selector): when the mode byte is non-zero,
/// five float-reader callees are polled (each returns a pointer to one or
/// two floats), the floats are routed through two multi-argument calls, a
/// frame-slot out-call produces a further float, and one float is converted
/// to int with truncation semantics exactly matching `cvttss2si` (including
/// the indefinite `0x80000000` for NaN, infinities and out-of-range inputs)
/// before three final calls. The sum of two reader floats is computed and
/// discarded (its store targets a slot that is never read as a value), so
/// the rewrite omits it. Every reader's slot-pointer argument points into
/// the caller's own frame and is skipped in the proof; only the frame-slot
/// out-call's word is observed downstream.
///
/// Control then (or immediately, when the mode byte is zero) reads a
/// progress counter: at or above 15 it returns 0. Otherwise a three-argument
/// poll decides: a negative answer or one above 2 returns 0, an answer of 2
/// returns at once, and 0 or 1 runs a bounded loop (at most 15 single-argument
/// calls, advancing the counter, bailing out to return the poll answer once
/// the counter reaches a scripted bound) followed by a mode-flag branch. The
/// flag-set path makes three calls and returns the poll answer; the clear
/// path branches on a selector global (absent: refresh plus two calls and a
/// timed latch; present: a test call splitting into a two-call update or an
/// object call gated on one object byte, always ending in a final call that
/// clears the latch) and returns the poll answer. The counter re-check inside
/// the loop head is dead: the entry check already filtered, and no stubbed
/// callee writes globals between them.
///
/// Original: 0x008BE3A0 (cdecl, two stack arguments; thirty-one direct callees:
/// float readers returning pointers, one frame-slot out-call, several
/// object calls on constant objects, one object call on a planted heap
/// object read through a global).
lf_checker_rt::export!(cdecl, rw_008BE3A0(arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const WORD_A: u32 = 0x01160C10;
        const MODE: u32 = 0x01160C32;
        const KIND: u32 = 0x0116C250;
        const SUB: u32 = 0x0116C253;
        const PARAM: u32 = 0x01161868;
        const FCONST: u32 = 0x0116187C;
        const COUNT: u32 = 0x011618E0;
        const LATCH: u32 = 0x011609E0;
        const SEL: u32 = 0x01031BB8;
        const BOUND: u32 = 0x01172D8C;
        const OBJPTR: u32 = 0x01BB5624;
        const OBJ_THIS: u32 = 0x01176888;
        const SINK_THIS: u32 = 0x0118D7F0;
        const CVT_THIS: u32 = 0x01161578;
        const C_R2: u32 = 0;
        const C_R1A: u32 = 1;
        const C_R1B: u32 = 2;
        const C_MIX: u32 = 3;
        const C_USE: u32 = 4;
        const C_R1C: u32 = 5;
        const C_OK: u32 = 6;
        const C_EMIT: u32 = 7;
        const C_SEAL: u32 = 8;
        const C_I2: u32 = 9;
        const C_APPLY: u32 = 10;
        const C_FIN1: u32 = 11;
        const C_TRIG: u32 = 12;
        const C_FIN3: u32 = 13;
        const C_R1D: u32 = 14;
        const C_SLOT: u32 = 15;
        const C_FIN2F: u32 = 16;
        const C_TOUCH: u32 = 17;
        const C_CVTU: u32 = 18;
        const C_FLUSH: u32 = 19;
        const C_UPD2: u32 = 20;
        const C_POLL: u32 = 21;
        const C_STEP: u32 = 22;
        const C_PAIR: u32 = 23;
        const C_TRIO: u32 = 24;
        const C_ONE: u32 = 25;
        const C_REFRESH: u32 = 26;
        const C_CLEAR: u32 = 27;
        const C_TEST: u32 = 28;
        const C_OBJ: u32 = 29;
        const C_SYNC: u32 = 30;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(a).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { lf_checker_rt::global::<u8>(a).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { lf_checker_rt::global::<u32>(a).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn m32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        /// Exact `cvttss2si`: truncation toward zero, indefinite 0x80000000
        /// for NaN, infinities and out-of-range magnitudes. (Rust's `as`
        /// saturates those cases instead, so they are excluded first.)
        #[inline(always)]
        fn cvt(f: f32) -> i32 {
            if f.is_nan() || f >= 2147483648.0f32 || f < -2147483648.0f32 {
                0x80000000u32 as i32
            } else {
                f as i32
            }
        }

        if (arg1 as u8) != 0 {
            let mut slot: u32 = 0;
            let slotp: u32 = &mut slot as *mut u32 as u32;
            let f67: f32 =
                f32::from_bits(m32(lf_checker_rt::callee_cdecl!(C_R2, u32, 0x67u32, slotp)));
            let f69: f32 =
                f32::from_bits(m32(lf_checker_rt::callee_cdecl!(C_R1A, u32, slotp)));
            if rd8(MODE) == 0 {
                let fb: f32 =
                    f32::from_bits(m32(lf_checker_rt::callee_cdecl!(C_R1B, u32, slotp)));
                // Dead sum: the original adds fb to a scratch slot and stores
                // the result where no value-read ever follows. Omitted.
                let _ = (f69, fb);
            }
            lf_checker_rt::callee_cdecl!(C_R2, u32, 0x66u32, slotp);
            let sel: u32 = if rd8(KIND) == 0x6A || rd8(SUB) != 0 { 2 } else { 7 };
            let mut p1: u32 = 0;
            let mut p2: u32 = 0;
            lf_checker_rt::callee_cdecl!(
                C_MIX, u32, 1u32, sel, &mut p1 as *mut u32 as u32,
                &mut p2 as *mut u32 as u32, f67.to_bits()
            );
            lf_checker_rt::callee_cdecl!(C_USE, u32, rd32(WORD_A));
            let mut cur: f32;
            if rd8(MODE) != 0 {
                cur = f32::from_bits(m32(lf_checker_rt::callee_cdecl!(C_R1C, u32, slotp)));
                if (lf_checker_rt::callee_thiscall!(C_OK, u32, SINK_THIS) as u8) == 0 {
                    let q: u32 = lf_checker_rt::callee_cdecl!(C_R1C, u32, slotp);
                    cur = f32::from_bits(m32(q.wrapping_add(4)));
                }
            } else {
                cur = f32::from_bits(m32(lf_checker_rt::callee_cdecl!(C_R2, u32, 0x68u32, slotp)));
                if (lf_checker_rt::callee_thiscall!(C_OK, u32, SINK_THIS) as u8) == 0 {
                    let q: u32 = lf_checker_rt::callee_cdecl!(C_R2, u32, 0x68u32, slotp);
                    cur = f32::from_bits(m32(q.wrapping_add(4)));
                }
            }
            lf_checker_rt::callee_thiscall!(C_EMIT, u32, OBJ_THIS, 0x00E7D2CCu32);
            let sealed: u32 = lf_checker_rt::callee_thiscall!(
                C_SEAL, u32, OBJ_THIS, 0u32, 0u32, 0x0116186Cu32, cur.to_bits(), 2u32,
                1u32, 0u32, 1u32
            );
            wr32(WORD_A, sealed);
            let vi: u32 = m32(lf_checker_rt::callee_stdcall!(C_I2, u32, 0x42u32, slotp));
            let vj: u32 = m32(lf_checker_rt::callee_stdcall!(C_I2, u32, 0x3Eu32, slotp));
            let vk: u32 = m32(lf_checker_rt::callee_stdcall!(C_I2, u32, 0x3Bu32, slotp));
            let fc: f32 = f32::from_bits(rd32(FCONST));
            lf_checker_rt::callee_cdecl!(
                C_APPLY, u32, rd32(WORD_A), rd32(PARAM), 0x01161874u32, fc.to_bits(),
                vk, vj, vi
            );
            lf_checker_rt::callee_cdecl!(C_FIN1, u32, rd32(WORD_A));
            lf_checker_rt::callee_cdecl!(C_TRIG, u32, 1u32);
            lf_checker_rt::callee_cdecl!(C_FIN3, u32, rd32(WORD_A), 1u32, 2u32);
            let mut fd: f32 =
                f32::from_bits(m32(lf_checker_rt::callee_cdecl!(C_R1D, u32, slotp)));
            if (lf_checker_rt::callee_thiscall!(C_OK, u32, SINK_THIS) as u8) == 0 {
                let q: u32 = lf_checker_rt::callee_cdecl!(C_R1D, u32, slotp);
                fd = f32::from_bits(m32(q.wrapping_add(4)));
            }
            let mut fout: u32 = 0;
            lf_checker_rt::callee_cdecl!(
                C_SLOT, u32, 3u32, 0u32, &mut fout as *mut u32 as u32, 0u32
            );
            let fs: f32 = f32::from_bits(fout);
            lf_checker_rt::callee_cdecl!(C_FIN2F, u32, rd32(WORD_A), 1u32, fs.to_bits());
            lf_checker_rt::callee_cdecl!(C_TOUCH, u32, rd32(WORD_A));
            wr32(COUNT, 0);
            let n: i32 = cvt(f32::from_bits(m32(lf_checker_rt::callee_cdecl!(
                C_R2, u32, 0x1Du32, slotp
            ))));
            lf_checker_rt::callee_thiscall!(C_CVTU, u32, CVT_THIS, n as u32, 0u32, 0xFFu32);
            lf_checker_rt::callee_cdecl!(C_FLUSH, u32,);
            lf_checker_rt::callee_cdecl!(C_SYNC, u32,);
            lf_checker_rt::callee_cdecl!(C_UPD2, u32, 0xFFFF_FFFFu32, 0u32);
            let _ = fd;
        }
        if (rd32(COUNT) as i32) >= 15 {
            return 0;
        }
        let edi: u32 = lf_checker_rt::callee_cdecl!(C_POLL, u32, arg1, rd8(MODE) as u32, arg2);
        let e: i32 = edi as i32;
        if e < 0 {
            return 0;
        }
        if e > 2 {
            return 0;
        }
        if e == 2 {
            return edi;
        }
        let bound: i32 = rd32(BOUND) as i32;
        loop {
            let y: i32 = rd32(COUNT) as i32;
            if y >= 15 {
                break;
            }
            if y >= bound {
                return edi;
            }
            lf_checker_rt::callee_cdecl!(C_STEP, u32, y as u32);
            wr32(COUNT, (y as u32).wrapping_add(1));
        }
        if rd8(MODE) != 0 {
            lf_checker_rt::callee_cdecl!(C_PAIR, u32, rd32(WORD_A), 0u32);
            lf_checker_rt::callee_cdecl!(C_TRIO, u32, rd32(WORD_A), 0u32, 1u32);
            lf_checker_rt::callee_cdecl!(C_ONE, u32, rd32(WORD_A));
            return edi;
        }
        let s: u32 = rd32(SEL);
        if s == 0xFFFF_FFFF {
            lf_checker_rt::callee_cdecl!(C_REFRESH, u32,);
            lf_checker_rt::callee_thiscall!(C_EMIT, u32, OBJ_THIS, 0x00E7D30Cu32);
            wr32(LATCH, 0x1388);
            lf_checker_rt::global::<u8>(0x01172CD3).write(0);
            lf_checker_rt::callee_cdecl!(C_CLEAR, u32, 0u32);
            return edi;
        }
        if (lf_checker_rt::callee_cdecl!(C_TEST, u32, rd32(WORD_A), s) as u8) != 0 {
            lf_checker_rt::callee_cdecl!(C_PAIR, u32, rd32(WORD_A), s);
            lf_checker_rt::callee_cdecl!(C_TRIO, u32, rd32(WORD_A), s, 1u32);
        } else {
            let obj: u32 = rd32(OBJPTR);
            lf_checker_rt::callee_thiscall!(C_OBJ, u32, obj);
            if m32(obj.wrapping_add(0x169)) as u8 == 0 {
                lf_checker_rt::callee_cdecl!(C_PAIR, u32, rd32(WORD_A), 0u32);
                lf_checker_rt::callee_cdecl!(C_TRIO, u32, rd32(WORD_A), 0u32, 1u32);
            }
        }
        lf_checker_rt::callee_cdecl!(C_ONE, u32, rd32(WORD_A));
        wr32(LATCH, 0);
        edi
    }
});
