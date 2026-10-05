// original: 0x00cf54b0 ped_task_think (proposed)

/// Advance one step of a ped task's think state machine.
///
/// `this` is the task object, `arg0`/`arg1` are two related objects and
/// `arg2` is a float (bit pattern) carried through to several callees.
/// Original convention: thiscall with three stack words (the callee pops 0xc bytes).
///
/// The task holds a child pointer at `+0x10` (zero when there is no child),
/// a state word at `+0x14`, an opaque word at `+0x64`, two more words at
/// `+0x78`/`+0x7c` used when there is no child, and a flag byte at `+0x89`.
/// The child (when present) exposes two words at `+0x0c`/`+0x10` and a float
/// at `+0x4c`. `arg0` has a mask word at `+0x26c`; `arg1` a float at `+0x08`.
///
/// Behaviour. When a child is present, a predicate callee is asked about
/// `arg0`; if it agrees and the child's float is below 0.75, a worker callee
/// runs, flag bit 2 is set on the task, the low two bits of the mask word
/// are cleared, a follow-up callee runs and the state becomes 8. A gate
/// callee is then asked; a non-zero answer returns immediately. Otherwise a
/// range-test callee classifies the child's `+0x0c` word into 1 or 2, a
/// five-word callee runs with that class and the constant 1, and its answer
/// is returned (writing state 8 first when its low byte is zero).
///
/// When there is no child, the range test classifies `+0x7c` instead. If it
/// passes, a second classifier's low byte is kept as a flag and two
/// five-word probe callees run in sequence, the first filling one float
/// through an out-pointer; depending on their answers and on a third
/// predicate plus the sign of (out-float minus `arg1`'s float) against -0.5,
/// the task either records a code derived from the flag (0x75/0x76) with
/// state 7, or a neighbouring code (0x77/0x78) and continues to a final
/// four-word callee fed with `arg2` and two tuned globals. If the range test
/// fails, the same two probes run with a mode of 1 rather than 2: a failure
/// of either, or of the third predicate, or a difference at or above -0.5,
/// selects a slow path that runs one four-word callee with code 0x70 and the
/// final callee with `arg2` and two tuned globals (one pair when `+0x7c` is
/// 0x6d, another otherwise); a
/// difference below -0.5 selects a fast path whose code is 0x6e or 0x6f and
/// whose six-word callee also receives the task's `+0x78` word, ending in
/// state 7. Every path returns its last callee's full `eax`.
///
/// Edge cases: all float comparisons are ordered (`x < c` is false for NaN,
/// matching the original's conditional jumps after `comiss`); only the low
/// byte of each callee answer drives branches, while the full word is what a
/// returned answer compares; the slow paths reuse the incoming `arg0` and
/// `arg1` stack slots as temporaries for the two tuned globals, which is why
/// this function's proof runs with the stack comparison off. The callee
/// out-pointer aims at scratch below the stack pointer, so its address is
/// skipped in the call comparison.
lf_checker_rt::export!(thiscall, rw_00cf54b0(this: u32, arg0: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const C_PRED: u32 = 1;
        const C_WORKER: u32 = 2;
        const C_FOLLOW: u32 = 3;
        const C_GATE: u32 = 4;
        const C_RANGE: u32 = 5;
        const C_CLASSIFY: u32 = 6;
        const C_CLASS2: u32 = 7;
        const C_PROBE1: u32 = 8;
        const C_PROBE2: u32 = 9;
        const C_ACTION: u32 = 10;
        const C_FINISH: u32 = 11;
        const C_FINAL: u32 = 12;

        const T_CHILD: u32 = 0x10;
        const T_STATE: u32 = 0x14;
        const T_GATE_ARG: u32 = 0x64;
        const T_SLOW_A: u32 = 0x78;
        const T_SLOW_B: u32 = 0x7c;
        const T_FLAGS: u32 = 0x89;
        const A0_MASK: u32 = 0x26c;
        const A1_FLOAT: u32 = 8;
        const CH_C: u32 = 0x0c;
        const CH_D: u32 = 0x10;
        const CH_FLOAT: u32 = 0x4c;

        const HIGH: f32 = f32::from_bits(0x3f40_0000); // 0.75
        const LOW: f32 = f32::from_bits(0xbf00_0000); // -0.5
        const RATE: u32 = 0x447a_0000; // 1000.0
        const G50: u32 = 0x0105_3950;
        const G54: u32 = 0x0105_3954;
        const G58: u32 = 0x0105_3958;
        const G5C: u32 = 0x0105_395c;
        const G60: u32 = 0x0105_3960;
        const G64: u32 = 0x0105_3964;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn glob(va: u32) -> u32 {
            unsafe { (lf_checker_rt::relocated(va) as *const u32).read_unaligned() }
        }
        /// The shared tail: state 5 would become 8 (no path reaches it as 5).
        #[inline(always)]
        unsafe fn tail(esi: u32) {
            unsafe {
                if rd32(esi + T_STATE) == 5 {
                    wr32(esi + T_STATE, 8);
                }
            }
        }

        let esi = this;
        let edi = arg0;
        let ebx = arg1;
        let child = rd32(esi + T_CHILD);
        if child != 0 {
            let ok: u32 = lf_checker_rt::callee_stdcall!(C_PRED, u32, edi, 0);
            if (ok as u8) != 0 {
                let x = rdf(child + CH_FLOAT);
                if x < HIGH {
                    let f = rdf(child + CH_FLOAT);
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        C_WORKER, u32, esi, ebx, edi, rd32(child + CH_D),
                        rd32(child + CH_C), f.to_bits());
                    ((esi + T_FLAGS) as *mut u8).write(rd8(esi + T_FLAGS) | 4);
                    wr32(edi + A0_MASK, rd32(edi + A0_MASK) & 0xffff_fffc);
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        C_FOLLOW, u32, esi, edi);
                    wr32(esi + T_STATE, 8);
                }
            }
            let g: u32 = lf_checker_rt::callee_cdecl!(
                C_GATE, u32, edi, rd32(esi + T_GATE_ARG));
            if (g as u8) != 0 {
                return g;
            }
            let t: u32 = lf_checker_rt::callee_cdecl!(
                C_RANGE, u32, rd32(child + CH_C));
            let class = if (t as u8) != 0 { 2u32 } else { 1u32 };
            let r: u32 = lf_checker_rt::callee_cdecl!(
                C_CLASSIFY, u32, edi, ebx, arg2, class, 1);
            if (r as u8) == 0 {
                wr32(esi + T_STATE, 8);
            }
            return r;
        }

        let ebp = rd32(esi + T_SLOW_B);
        let t: u32 = lf_checker_rt::callee_cdecl!(C_RANGE, u32, ebp);
        if (t as u8) != 0 {
            let b: u32 = lf_checker_rt::callee_cdecl!(C_CLASS2, u32, ebp);
            let flag = b as u8;
            let mut out = 0u32;
            let p1: u32 = lf_checker_rt::callee_cdecl!(
                C_PROBE1, u32, edi, ebx, arg2, 2,
                (&mut out as *mut u32) as u32);
            if (p1 as u8) == 0 {
                let code = if flag == 0 { 0x78u32 } else { 0x77u32 };
                let r: u32 = lf_checker_rt::callee_thiscall!(
                    C_ACTION, u32, esi, edi, 9, code, RATE);
                let r2: u32 = lf_checker_rt::callee_thiscall!(
                    C_FINAL, u32, esi, ebx, arg2, glob(G50), glob(G54));
                wr32(esi + T_STATE, 6);
                tail(esi);
                return r2;
            }
            let p2: u32 = lf_checker_rt::callee_cdecl!(
                C_PROBE2, u32, edi, ebx, arg2, 2, 0);
            if (p2 as u8) != 0 {
                let e: u32 = lf_checker_rt::callee_stdcall!(
                    C_PRED, u32, edi, 4);
                if (e as u8) != 0 {
                    let diff = sub(f32::from_bits(out), rdf(ebx + A1_FLOAT));
                    if diff < LOW {
                        let code = if flag == 0 { 0x76u32 } else { 0x75u32 };
                        let r: u32 = lf_checker_rt::callee_thiscall!(
                            C_ACTION, u32, esi, edi, 9, code, RATE);
                        wr32(esi + T_STATE, 7);
                        tail(esi);
                        return r;
                    }
                }
                let code = if flag == 0 { 0x78u32 } else { 0x77u32 };
                let r: u32 = lf_checker_rt::callee_thiscall!(
                    C_ACTION, u32, esi, edi, 9, code, RATE);
                let r2: u32 = lf_checker_rt::callee_thiscall!(
                    C_FINAL, u32, esi, ebx, arg2, glob(G50), glob(G54));
                wr32(esi + T_STATE, 6);
                tail(esi);
                return r2;
            }
            let code = if flag == 0 { 0x76u32 } else { 0x75u32 };
            let r: u32 = lf_checker_rt::callee_thiscall!(
                C_ACTION, u32, esi, edi, 9, code, RATE);
            wr32(esi + T_STATE, 7);
            tail(esi);
            return r;
        }

        let mut out = 0u32;
        let p1: u32 = lf_checker_rt::callee_cdecl!(
            C_PROBE1, u32, edi, ebx, arg2, 1,
            (&mut out as *mut u32) as u32);
        if (p1 as u8) != 0 {
            let p2: u32 = lf_checker_rt::callee_cdecl!(
                C_PROBE2, u32, edi, ebx, arg2, 1, 0);
            if (p2 as u8) != 0 {
                let e: u32 = lf_checker_rt::callee_stdcall!(
                    C_PRED, u32, edi, 4);
                if (e as u8) != 0 {
                    let diff = sub(f32::from_bits(out), rdf(ebx + A1_FLOAT));
                    if diff < LOW {
                        let _: u32 = lf_checker_rt::callee_thiscall!(
                            C_ACTION, u32, esi, edi, 9, 0x6e, RATE);
                        let r: u32 = lf_checker_rt::callee_thiscall!(
                            C_FINISH, u32, esi, ebx, edi, rd32(esi + T_SLOW_A),
                            ebp, 9, 0x6e);
                        wr32(esi + T_STATE, 7);
                        tail(esi);
                        return r;
                    }
                }
            } else {
                let diff = sub(f32::from_bits(out), rdf(ebx + A1_FLOAT));
                let code_fast = if diff < LOW { 0x6e } else { 0x6f };
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    C_ACTION, u32, esi, edi, 9, code_fast, RATE);
                let r: u32 = lf_checker_rt::callee_thiscall!(
                    C_FINISH, u32, esi, ebx, edi, rd32(esi + T_SLOW_A),
                    ebp, 9, 0x6e);
                wr32(esi + T_STATE, 7);
                tail(esi);
                return r;
            }
        }
        let (g3, g4) = if rd32(esi + T_SLOW_B) == 0x6d {
            (glob(G58), glob(G5C))
        } else {
            (glob(G60), glob(G64))
        };
        let _: u32 = lf_checker_rt::callee_thiscall!(
            C_ACTION, u32, esi, edi, 9, 0x70, RATE);
        let r: u32 = lf_checker_rt::callee_thiscall!(
            C_FINAL, u32, esi, ebx, arg2, g3, g4);
        wr32(esi + T_STATE, 6);
        tail(esi);
        r
    }
});
