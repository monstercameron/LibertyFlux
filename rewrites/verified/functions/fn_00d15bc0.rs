// original: 0x00d15bc0 combat_task_select_response (proposed)
//
// Picks this task's response to the current combat situation. `this` is the
// task, `a1` the actor, `a2` the situation code; the result is a response
// code (0x13f, 0x76c-0x777, 0xc8), the incoming `a2`, or a 0x76f default.
//
// First the task hook runs (callee 1), then a probe chain over the record
// at `a1+0x2b0`: a getter (callee 2, six calls whose answers share one
// scripted object) plus a table lookup (callee 3) set two flags. When the
// actor's status word bit 2 is clear and the record word is zero with the
// first flag set, a second getter answer equal to 0x2e runs a setup call
// (callee 6), any other value ends the task with 0x13f; with a nonzero
// record word and both flags set the task also ends with 0x13f after two
// calls (callees 4, 5). Otherwise a selector (callee 7) answers: null
// returns `a2`.
//
// Next, unless the actor's state nibble is past 1, a motion flag is clear,
// or the code is 0x771 (or 0x76d with a live target), the actor's position
// is staged into three scratch words and offered to two motion calls
// (callees 8, 9, whose frame-pointer argument is snapshotted, not
// compared); a refused second call with a live target runs a notifier
// (callee 10). Either way the default answer becomes 0x76f from here on.
//
// A finish check (callee 11) failing ends with 0x772. With status bit 3 set
// and code 0x76d, an approve virtual call on the +8 object must answer
// 0x770 with matching generation fields, else 0x76d or 0x770 is returned.
// A set marker (+0x211 on the +0x3c object) may then end with 0x777 through
// a readiness call (callee 22), a mask test and a gate call (callee 21).
//
// Otherwise the code dispatches: 0x76d runs a positioning attempt (a probe
// call, callee 12, lowers the 1.0 threshold to 0.5; the approve call must stay
// silent; a gate, callee 13, and a scan, callee 14, must pass with no live
// target; a scaled tick, callee 15, must stay under the threshold) and then
// the gather call of 0x00d15950 (callee 16, stubbed: actor, target, position,
// 3.0 from writable data, scratch words, 0, 0), returning 0x76f on success.
// 0x76f returns 0xc8 unless the scan passes with a wrong mode mask. 0x774
// compares the actor-target distance against 20.0 or 30.0 (gate callee 20
// picks) past two lookup calls (callees 18, 19) and status tests, returning
// 0x76e. 0x773 and 0x776 thread two encrypted-region helpers (callees 24,
// 25, intercepted like the rest) with mode and flag tests, returning 0x776
// or 0x773. Every other code returns the default answer. All returns are
// clean immediates or reloads, no leftovers.
// Original convention: thiscall (this in ECX, two stack words).
lf_checker_rt::export!(thiscall, rw_00d15bc0(this: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const REC: u32 = 0x2b0;
        const ONE: f32 = f32::from_bits(0x3f800000);
        const HALF: f32 = f32::from_bits(0x3f000000);
        const STEP: f32 = f32::from_bits(0x38000100);
        const NEAR: f32 = f32::from_bits(0x41a00000); // 20.0
        const FAR: f32 = f32::from_bits(0x41f00000); // 30.0
        const RADIUS3_FILE_VA: u32 = 0x1053cc0;

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
        #[inline(always)]
        fn below_eq(a: f32, b: f32) -> bool {
            !(core::hint::black_box(a) > core::hint::black_box(b))
        }
        #[inline(always)]
        unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
            unsafe {
                let vt = rd32(obj);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(vt + slot) as usize);
                f(obj)
            }
        }

        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, a1);
        let esi = a1.wrapping_add(REC);
        let mut dflt = a2;
        // Probe chain.
        let g1: u32 = lf_checker_rt::callee_thiscall!(2, u32, esi);
        let mut f12: u8 = 0;
        let mut f13: u8 = 0;
        let mut al: u8 = 0;
        if g1 != 0 {
            let g2: u32 = lf_checker_rt::callee_thiscall!(2, u32, esi);
            if rd32(g2 + 0x18) != 0 {
                f12 = 1;
                let g3: u32 = lf_checker_rt::callee_thiscall!(2, u32, esi);
                let t: u32 = lf_checker_rt::callee_cdecl!(3, u32, rd32(g3 + 0x18));
                if (rd32(t + 0x20) >> 5) & 1 == 0 {
                    f13 = 1;
                } else {
                    let g4: u32 = lf_checker_rt::callee_thiscall!(2, u32, esi);
                    if rd32(g4 + 0x60) as i32 <= 0 {
                        al = f12;
                        f13 = 0;
                    } else {
                        f13 = 1;
                    }
                }
                if f12 != 0 {
                    let g5: u32 = lf_checker_rt::callee_thiscall!(2, u32, esi);
                    let t2: u32 = lf_checker_rt::callee_cdecl!(3, u32, rd32(g5 + 0x18));
                    al = if rd32(t2 + 4) != rd32(esi) { 1 } else { 0 };
                }
            }
        }
        // NOTE: the original's f12==0 skip jumps over the L4B block, leaving
        // al == 0; the `if f12 != 0` above reproduces that (f12 is only 1
        // when the chain reached the table lookup).
        if rd8(a1 + 0x26c) & 4 == 0 {
            let recw = rd32(esi);
            if recw == 0 {
                if f12 != 0 {
                    let g6: u32 = lf_checker_rt::callee_thiscall!(2, u32, esi);
                    if rd32(g6 + 0x18) != 0x2e {
                        let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, esi);
                        return 0x13f;
                    }
                    let _: u32 = lf_checker_rt::callee_thiscall!(6, u32, esi, a1, 1);
                }
            } else if f13 == 0 || al != 0 {
                // f13==0 goes straight; f13!=0 needs al!=0 (else D15CA7
                // rejoins D15CC9 since recw != 0).
                let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, esi);
                let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, esi, 9, 0xb, 0, 0);
                return 0x13f;
            }
        }
        let sel: u32 = lf_checker_rt::callee_thiscall!(7, u32, rd32(a1 + 0x224), 1);
        let mut w1c = sel;
        if sel == 0 {
            return a2;
        }
        // Motion staging.
        let mut f20 = [0u32; 3];
        if rd8(a1 + 0x1e2) & 0xf < 2
            && rd8(rd32(a1 + 0x224) + 0x38) & 1 != 0
            && (a2 == 0x76d || a2 != 0x771)
            && (a2 != 0x76d || rd32(a1 + 0xd68) == 0)
        {
            let ap = rd32(a1 + 0x20);
            f20 = [rd32(ap + 0x30), rd32(ap + 0x34), rd32(ap + 0x38)];
            let p20 = (&mut f20 as *mut u32) as u32;
            let live = rd32(a1 + 0xd68);
            if live != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(8, u32, live, p20, 0);
            }
            let m6990: u32 = lf_checker_rt::callee_thiscall!(
                9,
                u32,
                rd32(a1 + 0x224).wrapping_add(0x10),
                p20,
                0
            );
            if (m6990 as u8) == 0 {
                if rd32(a1 + 0xd68) != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(10, u32, a1);
                }
                dflt = 0x76f;
            }
        }
        // Finish check.
        if rd32(this + 8) != 0 {
            let fin: u32 = lf_checker_rt::callee_thiscall!(11, u32, w1c, 0, 0);
            if (fin as u8) == 0 {
                return 0x772;
            }
        }
        if rd32(this + 0x60) & 8 != 0 {
            if a2 != 0x76d {
                return 0x770;
            }
            let c = rd32(this + 8);
            if c == 0 {
                return 0x770;
            }
            if vcall0(c, 0x0c) != 0x770 {
                return 0x770;
            }
            let y = rd32(this + 8);
            if rd32(y + 0x14) != 4 {
                return 0x770;
            }
            if rd32(y + 0x20) != 0x76d {
                return 0x770;
            }
            return 0x76d;
        }
        // Marker gate.
        if rd8(rd32(this + 0x3c) + 0x211) != 0 {
            let rdy: u32 = lf_checker_rt::callee_thiscall!(22, u32, a1);
            if (rdy as u8) == 0 {
                let m: u32 = lf_checker_rt::callee_thiscall!(17, u32, rd32(a1 + 0x224));
                if rd32(m + 0x8f4) & 0x60000 != 0x40000 {
                    // fall to switch
                } else {
                    let g: u32 = lf_checker_rt::callee_thiscall!(21, u32, esi, 1);
                    if (g as u8) != 0 {
                        return 0x777;
                    }
                    return switch(this, a1, a2, dflt, w1c, &mut f20);
                }
                return switch(this, a1, a2, dflt, w1c, &mut f20);
            } else if rd8(a1 + 0xa60) == 1 {
                let g: u32 = lf_checker_rt::callee_thiscall!(21, u32, esi, 1);
                if (g as u8) != 0 {
                    return 0x777;
                }
                return switch(this, a1, a2, dflt, w1c, &mut f20);
            } else {
                let m: u32 = lf_checker_rt::callee_thiscall!(17, u32, rd32(a1 + 0x224));
                if rd32(m + 0x8f4) & 0x60000 == 0x40000 {
                    let g: u32 = lf_checker_rt::callee_thiscall!(21, u32, esi, 1);
                    if (g as u8) != 0 {
                        return 0x777;
                    }
                }
                return switch(this, a1, a2, dflt, w1c, &mut f20);
            }
        }
        return switch(this, a1, a2, dflt, w1c, &mut f20);

        /// Situation-code dispatch.
        unsafe fn switch(
            this: u32,
            a1: u32,
            a2: u32,
            dflt: u32,
            _w1c: u32,
            f20: &mut [u32; 3],
        ) -> u32 {
            unsafe {
                const ONE: f32 = f32::from_bits(0x3f800000);
                const HALF: f32 = f32::from_bits(0x3f000000);
                const STEP: f32 = f32::from_bits(0x38000100);
                const NEAR: f32 = f32::from_bits(0x41a00000);
                const FAR: f32 = f32::from_bits(0x41f00000);
                const RADIUS3_FILE_VA: u32 = 0x1053cc0;
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
                #[inline(always)]
                fn below_eq(a: f32, b: f32) -> bool {
                    !(core::hint::black_box(a) > core::hint::black_box(b))
                }
                #[inline(always)]
                fn above(a: f32, b: f32) -> bool {
                    core::hint::black_box(a) > core::hint::black_box(b)
                }
                #[inline(always)]
                unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
                    unsafe {
                        let vt = rd32(obj);
                        let f: extern "thiscall" fn(u32) -> u32 =
                            core::mem::transmute(rd32(vt + slot) as usize);
                        f(obj)
                    }
                }
                // Table decode: ten entries for 0x76d..0x776, the rest default.
                if a2.wrapping_sub(0x76d) > 9 {
                    return dflt;
                }
                match a2 {
                    0x76d => {
                        let pr: u32 = lf_checker_rt::callee_cdecl!(12, u32, a1);
                        // The 1.0 store lands on the w1c slot: a push sits
                        // between the lea-math and the store.
                        let mut w = ONE.to_bits();
                        if (pr as u8) != 0 {
                            w = HALF.to_bits();
                        }
                        let c = rd32(this + 8);
                        let mut f13b: u8 = 0;
                        if c != 0 && vcall0(c, 0x0c) == 0x76f {
                            f13b = 1;
                        }
                        if (rd32(this + 0x60) >> 0x0c) & 1 != 0 {
                            return dflt;
                        }
                        let gt: u32 = lf_checker_rt::callee_thiscall!(
                            13,
                            u32,
                            a1.wrapping_add(0x2b0),
                            1
                        );
                        if (gt as u8) == 0 {
                            return dflt;
                        }
                        if f13b != 0 {
                            return dflt;
                        }
                        let sc: u32 = lf_checker_rt::callee_cdecl!(14, u32, a1);
                        if (sc as u8) == 0 {
                            return dflt;
                        }
                        if rd32(a1 + 0xd68) != 0 {
                            return dflt;
                        }
                        let tick: u32 = lf_checker_rt::callee_cdecl!(15, u32,);
                        let x1 = f32::from_bits(w);
                        let x0 = mul((tick as i32) as f32, STEP);
                        if below_eq(x1, x0) {
                            return dflt;
                        }
                        let r3: f32 = unsafe { *lf_checker_rt::global::<f32>(RADIUS3_FILE_VA) };
                        let p20 = (f20 as *mut u32) as u32;
                        let ok: u32 = lf_checker_rt::callee_cdecl!(
                            16,
                            u32,
                            a1,
                            rd32(this + 0x3c),
                            rd32(a1 + 0x20).wrapping_add(0x30),
                            r3.to_bits(),
                            p20,
                            0,
                            0
                        );
                        if (ok as u8) == 0 {
                            return dflt;
                        }
                        0x76f
                    }
                    0x76f => {
                        let sc: u32 = lf_checker_rt::callee_cdecl!(14, u32, a1);
                        if (sc as u8) == 0 {
                            return 0xc8;
                        }
                        let m: u32 = lf_checker_rt::callee_thiscall!(17, u32, rd32(a1 + 0x224));
                        if rd32(m + 0x8f4) & 0x180 != 0x80 {
                            return dflt;
                        }
                        0xc8
                    }
                    0x774 => {
                        let p: u32 = lf_checker_rt::callee_thiscall!(18, u32, a1);
                        if p != 0 {
                            let q: u32 = lf_checker_rt::callee_thiscall!(18, u32, a1);
                            let r: u32 =
                                lf_checker_rt::callee_thiscall!(19, u32, q.wrapping_add(8));
                            if r != a1 {
                                return dflt;
                            }
                        }
                        let tgt = rd32(this + 0x3c);
                        let g: u32 = lf_checker_rt::callee_thiscall!(
                            20,
                            u32,
                            tgt.wrapping_add(0x2b0),
                            1
                        );
                        let x2 = if (g as u8) != 0 { NEAR } else { FAR };
                        let ap = rd32(a1 + 0x20);
                        let bp = rd32(tgt + 0x20);
                        let dy = sub(rdf(ap + 0x34), rdf(bp + 0x34));
                        let dx = sub(rdf(ap + 0x30), rdf(bp + 0x30));
                        let dz = sub(rdf(ap + 0x38), rdf(bp + 0x38));
                        let dist2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                        let st = rd32(a1 + 0x26c);
                        if st & 0x2000000 == 0 {
                            return 0x76e;
                        }
                        if rd32(a1 + 0xb30) != 0 && (st as u8) & 4 != 0 {
                            return dflt;
                        }
                        if below_eq(mul(x2, x2), dist2) {
                            return dflt;
                        }
                        0x76e
                    }
                    0x773 => {
                        let r: u32 = lf_checker_rt::callee_cdecl!(23, u32,);
                        if (r as u8) == 0 {
                            return case773_tail(a1, dflt);
                        }
                        let t = rd32(this + 0x3c);
                        let p: u32 = lf_checker_rt::callee_thiscall!(24, u32, t);
                        if (p as u8) == 0 {
                            return case773_tail(a1, dflt);
                        }
                        if rd32(rd32(a1 + 0x21c) + 0x12c) != 2
                            && rd8(a1 + 0xa60) == 1
                        {
                            return 0x770;
                        }
                        case773_tail(a1, dflt)
                    }
                    0x776 => {
                        let f12b = rd32(rd32(a1 + 0x21c) + 0x12c) == 2;
                        let q: u32 = lf_checker_rt::callee_cdecl!(25, u32,);
                        let f13b = (q as u8) != 0;
                        let tgt = rd32(this + 0x3c);
                        let p: u32 = lf_checker_rt::callee_thiscall!(24, u32, tgt);
                        if (p as u8) == 0 && rd8(tgt + 0x26c) & 4 != 0 {
                            return 0x773;
                        }
                        if f13b {
                            return 0x773;
                        }
                        if !f12b {
                            return 0x773;
                        }
                        dflt
                    }
                    _ => dflt,
                }
            }
        }

        /// Shared tail of the 0x773 case.
        unsafe fn case773_tail(a1: u32, dflt: u32) -> u32 {
            unsafe {
                #[inline(always)]
                unsafe fn rd32(a: u32) -> u32 {
                    unsafe { (a as *const u32).read_unaligned() }
                }
                #[inline(always)]
                unsafe fn rd8(a: u32) -> u8 {
                    unsafe { (a as *const u8).read() }
                }
                let v = rd32(rd32(a1 + 0x21c) + 0x12c);
                let q: u32 = lf_checker_rt::callee_cdecl!(25, u32,);
                if v != 2 {
                    // NOTE: the original reads the callee answer first; the
                    // order is (call, compare saved value, test answer).
                    let _ = q;
                    return dflt;
                }
                if (q as u8) != 0 {
                    return dflt;
                }
                if rd8(a1 + 0xa60) == 1 {
                    return 0x776;
                }
                if rd32(a1 + 0x264) & 0x400000 == 0 {
                    return dflt;
                }
                0x776
            }
        }

    }
});
