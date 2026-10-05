// original: 0x00CD6C40 CTaskSimpleFall::vf17
/// Fall-task state machine: advance the task by its current state.
///
/// `this` is the task object (state word at `+0x14`, timer at `+0x28`,
/// parameters at `+0x18`/`+0x1c`, helper object at `+0x20`), `ped` the ped
/// it runs on. Thiscall with one stack word; returns 1 in `al` only from
/// state 4, otherwise 0. Every path first flags the ped (`+0x29c` bit 0)
/// and stores 0x32 at `+0x298`, then dispatches once on the state word
/// (anything above 4 returns at once).
///
/// State 0 blends a new timer: the helper word at `+0x28` is widened to
/// float (unsigned) and mixed with the timeout callee's answer (signed) as
/// `s1 + f32(t) * k * (f*1.1 - s1)` with `s1 = f*0.9`, in that operand
/// order, then truncated toward zero through the x87 with the control word
/// saved and restored (out-of-range or NaN yields the indefinite integer,
/// whose low word is kept). The result is clamped to the limit global when
/// the ped check (callee 1) agrees and the ped's words allow it. The step
/// callee (callee 2) is then asked with (`ped`, params, 8.0); a zero answer
/// parks the task in state 4 with a zero timer. Otherwise a flag word
/// reached through the ped at `+0x1f8` may set 2.0 into the helper at
/// `+0x54`, and the state becomes 1. A ped nibble of 2 or more parks the
/// task without doing any of this.
/// State 1 raises ped bits, then compares the helper float at `+0x4c`
/// against 1.0 and 0.8 with above-or-equal tests that also take the set
/// branch for NaN (Rust `!(a > b)`); failures park in state 3 or return.
/// The probe callee (callee 3) maps 0, 1 or 3 to 0x81 and anything else to
/// 0x80, the step callee runs with (`ped`, 9, mapped, 2.0), and the state
/// becomes 2. A null helper parks in state 3.
/// State 2 needs ped bit 0, clears bit 15, maps the probe answer to
/// 0x8e/0x8f the same way, runs the step callee with (`ped`, 10, mapped,
/// 24.0) and parks in state 3.
/// State 3 raises ped bits and, unless the ped opts out, compares the timer
/// against the timeout callee's answer: below-or-equal zeroes the timer and
/// parks in state 4, otherwise the same flag word as state 0 either
/// subtracts the timeout from the timer or, when its bit is set, runs five
/// helper callees (4-9) over scratch and returns. The scratch the original
/// passes is uninitialized stack, pinned to zero by the contract's stack
/// fill, so the rewrite passes its own zeroed words; the word the original
/// pushes from post-call `ecx` is the stub's zero sequence step on both
/// sides, passed as an explicit 0.
/// State 4 runs the finish callee (callee 10) with -8.0 and returns 1.
lf_checker_rt::export!(thiscall, rw_00CD6C40(this: u32, ped: u32) -> u32 {
    unsafe {
        const STATE_OFF: u32 = 0x14;
        const PARAM0_OFF: u32 = 0x18;
        const PARAM1_OFF: u32 = 0x1c;
        const HELPER_OFF: u32 = 0x20;
        const TIMER_OFF: u32 = 0x28;
        const PED_NIBBLE_OFF: u32 = 0x1e2;
        const PED_FLOAT_OFF: u32 = 0x4c;
        const PED_SET_OFF: u32 = 0x54;
        const PED_FLAGW_OFF: u32 = 0x26c;
        const PED_MARK_OFF: u32 = 0x298;
        const PED_OPT_OFF: u32 = 0x29c;
        const PED_AUX_OFF: u32 = 0x1f8;
        const PED_INNER_OFF: u32 = 0x228;
        const PED_GATE_OFF: u32 = 0xa70;
        const PED_BIT_OFF: u32 = 0x219;
        const PED_RAISE_OFF: u32 = 0xbe0;
        const INNER_FLAG_OFF: u32 = 0x4c8;
        const AUX_FLAG_OFF: u32 = 0x28;
        const MARK_VALUE: u8 = 0x32;
        const LIMIT: u32 = 0x01051A44;
        const CLOCK: u32 = 0x011735B4;
        const RATE: u32 = 0x011735BC;
        const C09: u32 = 0x00FE88BC;
        const C11: u32 = 0x00FE8914;
        const CK: u32 = 0x00FE8684;
        const C10: u32 = 0x00FE88E8;
        const C08: u32 = 0x00FE8898;
        const C100: u32 = 0x00FE8B08;
        const STEP_IDLE: u32 = 0x41000000; // 8.0f
        const STEP_LOW: u32 = 0x40000000; // 2.0f
        const STEP_HIGH: u32 = 0x41800000; // 24.0f
        const FINISH_TIMER: u32 = 0xC1000000; // -8.0f

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        fn cfloat(va: u32) -> f32 {
            f32::from_bits(unsafe { (lf_checker_rt::global::<u32>(va) as *const u32).read_unaligned() })
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
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        /// What `fistp qword` under a truncate control word keeps in the low
        /// word: truncation for finite in-range values, otherwise the
        /// indefinite integer's low word (zero).
        #[inline(always)]
        fn fistp_low(f: f32) -> u32 {
            if !f.is_finite() || f >= 9223372036854775808.0 || f <= -9223372036854775808.0 {
                0
            } else {
                (f as i64) as u32
            }
        }
        /// The probe-answer map shared by states 1 and 2.
        #[inline(always)]
        fn probe_map(t: u32, yes: u32, no: u32) -> u32 {
            if t == 0 || t == 1 || t == 3 { yes } else { no }
        }

        wr32(ped + PED_OPT_OFF, rd32(ped + PED_OPT_OFF) | 1);
        ((ped + PED_MARK_OFF) as *mut u8).write(MARK_VALUE);
        let state = rd32(this + STATE_OFF);
        if state > 4 {
            return 0;
        }
        // Scratch for the state-3 helper calls; the original passes
        // uninitialized (contract-zeroed) stack words.
        let mut frame = [0u32; 12];
        let fbase = frame.as_mut_ptr() as u32;
        match state {
            0 => {
                if rd8(ped + PED_NIBBLE_OFF) & 0xF >= 2 {
                    wr32(this + TIMER_OFF, 0);
                    wr32(this + STATE_OFF, 4);
                    return 0;
                }
                let f = rd32(this + TIMER_OFF) as f32;
                let s1 = mul(f, cfloat(C09));
                let t: u32 = lf_checker_rt::callee_thiscall!(0, u32, this);
                let blended = add(mul(mul((t as i32) as f32, cfloat(CK)), sub(mul(f, cfloat(C11)), s1)), s1);
                let ticks = fistp_low(blended);
                let limit = rd32(lf_checker_rt::relocated(LIMIT));
                wr32(this + TIMER_OFF, ticks);
                if ticks > limit {
                    let ok: u32 = lf_checker_rt::callee_thiscall!(1, u32, ped);
                    if ok & 0xFF != 0
                        && rd32(ped + PED_GATE_OFF) != 1
                        && rd32(rd32(ped + PED_INNER_OFF) + INNER_FLAG_OFF) == 0
                    {
                        wr32(this + TIMER_OFF, limit);
                    }
                }
                let go: u32 = lf_checker_rt::callee_thiscall!(
                    2, u32, this, ped, rd32(this + PARAM0_OFF), rd32(this + PARAM1_OFF), STEP_IDLE
                );
                if go & 0xFF == 0 {
                    wr32(this + TIMER_OFF, 0);
                    wr32(this + STATE_OFF, 4);
                    return 0;
                }
                let aux = rd32(ped + PED_AUX_OFF);
                if aux != 0 && rd32(aux + AUX_FLAG_OFF) & 0x3c0 == 0x80 {
                    wr32(rd32(this + HELPER_OFF) + PED_SET_OFF, STEP_LOW);
                }
                wr32(this + STATE_OFF, 1);
                0
            }
            1 => {
                wr32(ped + PED_RAISE_OFF, rd32(ped + PED_RAISE_OFF) | 6);
                let helper = rd32(this + HELPER_OFF);
                if helper == 0 {
                    wr32(this + STATE_OFF, 3);
                    return 0;
                }
                let x = f32::from_bits(rd32(helper + PED_FLOAT_OFF));
                if !(cfloat(C10) > x) {
                    wr32(this + STATE_OFF, 3);
                    return 0;
                }
                let flags = rd32(ped + PED_FLAGW_OFF);
                if flags & 0x8000 == 0 || flags & 1 != 0 {
                    return 0;
                }
                if !(x > cfloat(C08)) {
                    return 0;
                }
                let t: u32 = lf_checker_rt::callee_thiscall!(3, u32, ped);
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    2, u32, this, ped, 9, probe_map(t, 0x81, 0x80), STEP_LOW
                );
                wr32(this + STATE_OFF, 2);
                0
            }
            2 => {
                if rd32(ped + PED_FLAGW_OFF) & 1 == 0 {
                    return 0;
                }
                wr32(ped + PED_FLAGW_OFF, rd32(ped + PED_FLAGW_OFF) & 0xffff7fff);
                let t: u32 = lf_checker_rt::callee_thiscall!(3, u32, ped);
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    2, u32, this, ped, 0x0a, probe_map(t, 0x8e, 0x8f), STEP_HIGH
                );
                wr32(this + STATE_OFF, 3);
                0
            }
            3 => {
                wr32(ped + PED_RAISE_OFF, rd32(ped + PED_RAISE_OFF) | 6);
                if rd8(ped + PED_FLAGW_OFF) & 1 == 0 && rd8(ped + PED_OPT_OFF) & 4 == 0 {
                    return 0;
                }
                let t: u32 = lf_checker_rt::callee_cdecl!(4, u32,);
                let timer = rd32(this + TIMER_OFF);
                if timer <= t {
                    wr32(this + TIMER_OFF, 0);
                    wr32(this + STATE_OFF, 4);
                    return 0;
                }
                let aux = rd32(ped + PED_AUX_OFF);
                if aux == 0 || rd32(aux + AUX_FLAG_OFF) & 0x3c0 != 0x80 {
                    wr32(this + TIMER_OFF, timer.wrapping_sub(t));
                    return 0;
                }
                if rd8(ped + PED_BIT_OFF) == 0 {
                    return 0;
                }
                let clock = rd32(lf_checker_rt::relocated(CLOCK));
                let _: u32 =
                    lf_checker_rt::callee_thiscall!(5, u32, fbase, 0, clock, 0x32);
                let scaled = mul(
                    f32::from_bits(rd32(lf_checker_rt::relocated(RATE))),
                    cfloat(C100),
                );
                let p = rd32(ped + PED_AUX_OFF);
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    6, u32, fbase + 16, p, scaled.to_bits(), 0x32, 0, 0
                );
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    7, u32, fbase + 32, ped, fbase + 40
                );
                let _: u32 = lf_checker_rt::callee_thiscall!(8, u32, fbase + 32);
                let _: u32 = lf_checker_rt::callee_thiscall!(9, u32, fbase);
                0
            }
            _ => {
                lf_checker_rt::callee_thiscall!(10, u32, this, FINISH_TIMER);
                1
            }
        }
    }
});
