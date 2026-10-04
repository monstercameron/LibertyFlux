// original: 0x00D82CD0 ui_state_dispatch_update (proposed)

/// Per-frame input/UI state update: dispatch on a mode byte, fill four
/// output words, and store them into the state object.
///
/// `obj` is the state object (vtable at `+0x00` with the probe hook in slot
/// `+0xec`, base block at `+0x20`, gate structs at `+0xf50`/`+0xf0c`/`+0xf80`,
/// table indices at `+0xde0`/`+0xde4`, flags across `+0xe00..+0x1304`). The
/// entry path clears a flag, publishes the object to the current-state
/// global, probes a 2D magnitude (accumulating a truncated millisecond
/// counter at `+0xee4` while it stays below 2.25), then either returns a
/// helper's answer early or fetches the mode record. A null record, a clear
/// enable bit, or a disabled mode index returns through the zeroing path
/// (outputs zeroed, timer recorded). Otherwise an optional notifier runs, a
/// wide-box check either finishes through the common tail or falls into the
/// mode switch: byte `+0x2a` of the mode record selects one of twenty bodies
/// (plus a default), each a few compares, float ops and helper calls that
/// set the byte `ob` and the words `oc`, `o10`, `o14` (plus scratch `o18`,
/// `o20`, `o30`). The tail applies a timer-gated correction, calls the
/// six-argument adjustment helper on the four outputs, and stores them at
/// `+0x1088`/`+0xf14` bit 7/`+0x1078`/`+0x107c`, then a final probe dot
/// product at `+0xf00`. Returns `obj`, except the early paths return the
/// helper answer, the sign-extended mode index, or the timer.
///
/// Original: 0x00D82CD0 (cdecl, one stack word). Direct helpers are
/// intercepted per site; the probe hook is thiscall through the planted
/// vtable slot with a scratch buffer. The millisecond counter uses x87
/// truncation of quadword range, emulated exactly (NaN, infinities and
/// out-of-range values yield 0x80000000). Float operation order is the
/// original's throughout, including reversed addends and negations placed
/// after their producing operation.
lf_checker_rt::export!(cdecl, rw_00d82cd0(obj: u32) -> u32 {
    unsafe {
        const OBJ_BASE: u32 = 0x20;
        const OBJ_GATE: u32 = 0xf50;
        const OBJ_NOTE: u32 = 0xf0c;
        const OBJ_AUX: u32 = 0xf80;
        const OBJ_AUXN: u32 = 0xf84;
        const OBJ_TBL0: u32 = 0xde0;
        const OBJ_TBL1: u32 = 0xde4;
        const OBJ_MODE: u32 = 0xe6e;
        const OBJ_FLAG73: u32 = 0xe73;
        const OBJ_E12: u32 = 0xe12;
        const OBJ_E38: u32 = 0xe38;
        const OBJ_E3C: u32 = 0xe3c;
        const OBJ_EE0: u32 = 0xee0;
        const OBJ_EE4: u32 = 0xee4;
        const OBJ_DD4: u32 = 0xdd4;
        const OBJ_E48: u32 = 0xe48;
        const OBJ_EF8: u32 = 0xef8;
        const OBJ_EF9: u32 = 0xef9;
        const OBJ_F00: u32 = 0xf00;
        const OBJ_F10: u32 = 0xf10;
        const OBJ_F14: u32 = 0xf14;
        const OBJ_F1C: u32 = 0xf1c;
        const OBJ_F1F: u32 = 0xf1f;
        const OBJ_O88: u32 = 0x1088;
        const OBJ_O78: u32 = 0x1078;
        const OBJ_O7C: u32 = 0x107c;
        const OBJ_BIG: u32 = 0x1304;
        const VT_PROBE: u32 = 0xec;
        const GATE_KIND: u32 = 0x210;
        const GATE_SUB: u32 = 0xa74;
        const GATE_FLAG: u32 = 0x224;
        const AUX_BIT: u32 = 0x164;
        const BASE_DX: u32 = 0x10;
        const BASE_PX: u32 = 0x30;
        const REC_T: u32 = 0x10;
        const REC_MODE: u32 = 0x2a;
        const REC_EN: u32 = 0x2b;
        const G_CUR: u32 = 0x1797758;
        const G_TIMER: u32 = 0x11735b4;
        const G_FSRC: u32 = 0x11735bc;
        const G_ARR: u32 = 0x16578d8;
        const G_TAB: u32 = 0x1178284;
        const C_MODES: u32 = 0x1177a80;
        const MAG_LIM: f32 = 2.25;
        const MS_SCALE: f32 = 1000.0;
        const BOX_HALF: f32 = 270.0;
        const HALF: f32 = 0.5;
        const NHALF: f32 = -0.5;
        const TENTH: f32 = f32::from_bits(0x3DCC_CCCD);
        const LEN15: f32 = 15.0;
        const TWO: f32 = 2.0;
        const TWO_HALF: f32 = 2.5;
        const QNEG: f32 = -0.25;
        const QPOS: f32 = 0.25;
        const FIFTY: f32 = 50.0;
        const K0012: f32 = f32::from_bits(0x3C44_9BA6);
        const C005: f32 = f32::from_bits(0x3D4C_CCCD);
        const C25: f32 = 25.0;
        const O2: f32 = f32::from_bits(0x3E4C_CCCD);
        const P75: f32 = f32::from_bits(0x3F40_0000);
        const N75: f32 = f32::from_bits(0xBF40_0000);
        const C001: f32 = f32::from_bits(0x3A83_126F);
        const C03: f32 = f32::from_bits(0x3E99_999A);
        const C095: f32 = f32::from_bits(0x3DC2_8F5C);
        const ONE: f32 = 1.0;
        const SIGN: u32 = 0x8000_0000;
        const I64LIM: f32 = 9223372036854775808.0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(va) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn g8(va: u32) -> u8 {
            unsafe { (lf_checker_rt::global::<u8>(va) as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wg32(va: u32, v: u32) {
            unsafe { (lf_checker_rt::global::<u32>(va) as *mut u32).write_unaligned(v) }
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
        fn neg(a: f32) -> f32 {
            f32::from_bits(core::hint::black_box(a.to_bits()) ^ SIGN)
        }
        /// Low word of x87 quadword truncation as the worker observes it:
        /// in-range magnitudes truncate toward zero; NaN, infinities and
        /// out-of-range magnitudes yield 0 (measured on NaN, both infinities
        /// and both out-of-range signs, not the indefinite value real x87
        /// produces). The -2^63 edge truncates to -2^63.
        #[inline(always)]
        fn trunc_qword_low(v: f32) -> u32 {
            if v.is_nan() || v >= I64LIM || v < -I64LIM {
                0
            } else {
                (v as i64) as u32
            }
        }
        /// Probe hook through the object's vtable slot with a scratch buffer.
        #[inline(always)]
        unsafe fn hook(obj: u32, scratch: *mut u32) -> u32 {
            unsafe {
                let slot = rd32(rd32(obj).wrapping_add(VT_PROBE));
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(obj, scratch as u32)
            }
        }
        /// Six-argument output-fill helper shared by case 3 and the default
        /// path: takes the four live outputs plus the object and the mode
        /// record, fills them through the stub. The byte-word local replicates
        /// the frame overlap (the byte sits atop the word's low bytes) so the
        /// stub's pre-write snapshot matches; returns the filled byte.
        #[inline(always)]
        unsafe fn fill6(obj: u32, edi: u32, ob: u8, oc: &mut f32, o10: &mut f32, o14: &mut f32) -> u8 {
            unsafe {
                let mut obw: u32 = (ob as u32) | oc.to_bits().wrapping_shl(8);
                lf_checker_rt::callee_cdecl!(
                    10, u32, obj, oc as *mut f32 as u32, o14 as *mut f32 as u32,
                    o10 as *mut f32 as u32, &mut obw as *mut u32 as u32, edi
                );
                obw as u8
            }
        }

        // ---- entry ----
        wr8(obj + OBJ_F1F, rd8(obj + OBJ_F1F) & 0xef);
        let mut ob: u8 = 0;
        let mut oc: f32 = 0.0;
        let mut o10: f32 = 0.0;
        let mut o14: f32 = 0.0;
        let mut o18: f32 = 0.0;
        let mut o20: f32 = 0.0;
        let mut o30: f32 = 0.0;
        wg32(G_CUR, obj);
        lf_checker_rt::callee_thiscall!(1, u32, obj);
        let mut hs = [0u32; 8];
        let rp = hook(obj, hs.as_mut_ptr());
        let h0 = rdf(rp);
        let h1 = rdf(rp + 4);
        let m = add(mul(h0, h0), mul(h1, h1));
        if !(m >= MAG_LIM) {
            let fsrc = f32::from_bits(g32(G_FSRC));
            let t = trunc_qword_low(mul(fsrc, MS_SCALE));
            // The conversion result is also left in the o18 slot.
            o18 = f32::from_bits(t);
            wr32(obj + OBJ_EE4, rd32(obj + OBJ_EE4).wrapping_add(t));
        } else {
            wr32(obj + OBJ_EE4, 0);
        }
        let gate = rd32(obj + OBJ_GATE);
        if gate != 0 {
            if rd8(gate + GATE_KIND) != 0 {
                let s = rd32(gate + GATE_SUB);
                if s == 1 || s == 2 {
                    return lf_checker_rt::callee_cdecl!(2, u32, obj);
                }
            }
            if rd32(gate + GATE_FLAG) == 0 {
                return lf_checker_rt::callee_cdecl!(2, u32, obj);
            }
        }
        let edi = lf_checker_rt::callee_cdecl!(3, u32, obj);
        // Zeroing path, shared by the null record and the clear enable bit.
        let zeroing = |obj: u32| -> u32 {
            unsafe {
                wr8(obj + OBJ_F14, rd8(obj + OBJ_F14) & 0x7f);
                wr32(obj + OBJ_O88, 0);
                wr32(obj + OBJ_O78, 0);
                wr32(obj + OBJ_O7C, C095.to_bits());
                wr32(obj + OBJ_E3C, 0);
                let t = g32(G_TIMER);
                wr32(obj + OBJ_E38, t);
                wr32(obj + OBJ_EE0, 0);
                t
            }
        };
        if edi == 0 {
            return zeroing(obj);
        }
        if rd8(edi + REC_EN) & 0x80 != 0 {
            wr8(obj + OBJ_F1C, rd8(obj + OBJ_F1C) | 8);
        }
        if rd8(obj + OBJ_F14) & 8 == 0 {
            return zeroing(obj);
        }
        let f10 = rd8(obj + OBJ_F10);
        if (f10 as i8) >= 0 {
            if g8(G_ARR.wrapping_add(f10 as u32)) != 0 {
                // fall through to the notifier
            } else {
                wr32(obj + OBJ_E3C, 0);
                return (f10 as i8) as i32 as u32;
            }
        }
        let note = rd32(obj + OBJ_NOTE);
        if note != 0 {
            lf_checker_rt::callee_thiscall!(4, u32, note, obj.wrapping_add(OBJ_NOTE));
        }
        wr32(obj + OBJ_NOTE, 0);
        // Common tail: timer-gated correction, six-argument adjustment of
        // the four outputs, object stores, final probe dot product.
        let tail = |obj: u32, ob: u8, oc: f32, o10: f32, o14: f32, o18: f32| -> u32 {
            unsafe {
            let mut o10l = o10;
            let mut o14l = o14;
            if rd32(obj + OBJ_BIG) == 1 {
                let n = rd32(obj + OBJ_AUXN) as i32;
                let e = if n > 0 { rd32(obj + OBJ_AUX) } else { 0 };
                if rd8(e.wrapping_add(AUX_BIT)) & 1 != 0 {
                    let r = lf_checker_rt::callee_thiscall!(14, u32, obj, 1);
                    if rd8(r.wrapping_add(AUX_BIT)) & 1 == 0 {
                        o14l = 0.0;
                        if C005 > o10l {
                            o10l = C005;
                        }
                    }
                }
            }
            let e6e = rd8(obj + OBJ_MODE);
            let big = rd32(obj + OBJ_BIG) == 1;
            let flag: u32 = if e6e == 1 || (big && (e6e == 0x0a || e6e == 0x0b || e6e == 0x0c)) {
                1
            } else {
                0
            };
            let o18b = (o18.to_bits() & 0xffff_ff00) | flag;
            let mut hs = [0u32; 8];
            let rp = hook(obj, hs.as_mut_ptr());
            let t0 = rdf(rp);
            let t4 = rdf(rp + 4);
            let t8 = rdf(rp + 8);
            let s = add(add(mul(t0, t0), mul(t4, t4)), mul(t8, t8));
            let above: u32 = if C25 > s { 1 } else { 0 };
            let mut wb: u32 = (ob as u32) | oc.to_bits().wrapping_shl(8);
            let mut wc = oc;
            let mut w10 = o10l;
            let mut w14 = o14l;
            lf_checker_rt::callee_stdcall!(
                15, u32, &mut wc as *mut f32 as u32, &mut wb as *mut u32 as u32,
                &mut w14 as *mut f32 as u32, &mut w10 as *mut f32 as u32, o18b, above
            );
            let ob2 = wb as u8;
            wr32(obj + OBJ_O88, wc.to_bits());
            wr8(obj + OBJ_F14, rd8(obj + OBJ_F14) & 0x7f | (ob2 & 1) << 7);
            let base = rd32(obj + OBJ_BASE);
            wr32(obj + OBJ_O78, w14.to_bits());
            wr32(obj + OBJ_O7C, w10.to_bits());
            let mut hs2 = [0u32; 8];
            let rp2 = hook(obj, hs2.as_mut_ptr());
            let u = add(
                mul(rdf(base + BASE_DX + 4), rdf(rp2 + 4)),
                mul(rdf(rp2), rdf(base + BASE_DX)),
            );
            let t = add(u, mul(rdf(base + BASE_DX + 8), rdf(rp2 + 8)));
            wr32(obj + OBJ_F00, t.to_bits());
            let _ = ob;
            obj
            }
        };
        // ---- wide-box check: finishes through the tail either way ----
        if rd8(obj + OBJ_EF8) & 0x80 != 0 {
            ob = 0;
            o14 = 0.0;
            oc = 0.0;
            o10 = O2;
            let base = rd32(obj + OBJ_BASE);
            let qx = rdf(base + BASE_PX);
            let qy = rdf(base + BASE_PX + 4);
            let r: u32 = lf_checker_rt::callee_thiscall!(
                5, u32, lf_checker_rt::relocated(C_MODES),
                sub(qx, BOX_HALF).to_bits(), add(qx, BOX_HALF).to_bits(),
                sub(qy, BOX_HALF).to_bits(), add(qy, BOX_HALF).to_bits()
            );
            if r & 0xff != 0 {
                wr8(obj + OBJ_EF8, rd8(obj + OBJ_EF8) & 0x7f);
                lf_checker_rt::callee_cdecl!(6, u32, obj, obj.wrapping_add(OBJ_E48));
            }
            return tail(obj, ob, oc, o10, o14, o18);
        }
        // ---- table-index refresh branch ----
        if rd8(obj + OBJ_EF9) & 1 != 0 {
            let e6e = rd8(obj + OBJ_MODE);
            if e6e != 5 && e6e != 0 {
                wr8(edi + REC_MODE, 0x16);
                wr32(edi + REC_T, g32(G_TIMER).wrapping_add(0x7d0));
                wr8(obj + OBJ_EF9, rd8(obj + OBJ_EF9) & 0xfe);
                let v1 = rd32(obj + OBJ_TBL1);
                let v0 = rd32(obj + OBJ_TBL0);
                lf_checker_rt::callee_thiscall!(7, u32, obj.wrapping_add(OBJ_DD4));
                lf_checker_rt::callee_thiscall!(8, u32, obj.wrapping_add(OBJ_DD4));
                wr32(obj + OBJ_TBL1, v0);
                wr32(obj + OBJ_TBL0, v1);
                let i1 = v1 & 0xffff;
                let i0 = v0 & 0xffff;
                let t1 = g32(G_TAB.wrapping_add(i1.wrapping_mul(4)));
                let t0 = g32(G_TAB.wrapping_add(i0.wrapping_mul(4)));
                if i1 != 0xffff && i0 != 0xffff && t1 != 0 && t0 != 0 {
                    let ax: u32 = lf_checker_rt::callee_thiscall!(
                        9, u32, lf_checker_rt::relocated(C_MODES), v1, v0
                    );
                    wr16(obj + OBJ_E12, (ax & 0xffff) as u16);
                } else {
                    lf_checker_rt::callee_thiscall!(7, u32, obj.wrapping_add(OBJ_DD4));
                }
            }
        }
        // ---- mode switch ----
        let sw = rd8(edi + REC_MODE);
        let timer_le_rec = || unsafe { g32(G_TIMER) <= rd32(edi + REC_T) };
        match sw {
            // Byte 1: fixed small outputs, record timer on expiry.
            1 => {
                ob = 0;
                o14 = 0.0;
                oc = 0.0;
                o10 = O2;
                if !timer_le_rec() {
                    wr8(edi + REC_MODE, 0);
                    wr32(obj + OBJ_E38, g32(G_TIMER));
                }
                tail(obj, ob, oc, o10, o14, o18)
            }
            // Byte 24: like byte 1 with a unit output.
            24 => {
                ob = 0;
                o14 = 0.0;
                oc = 0.0;
                o10 = ONE;
                if !timer_le_rec() {
                    wr8(edi + REC_MODE, 0);
                    wr32(obj + OBJ_E38, g32(G_TIMER));
                }
                tail(obj, ob, oc, o10, o14, o18)
            }
            // Byte 3: fill helper, direction probe, gated outputs.
            3 => {
                let _ = fill6(obj, edi, ob, &mut oc, &mut o10, &mut o14);
                ob = 0;
                // The original reads its own saved edi slot here; incoming
                // edi is pinned to 0 by the contract, so this is -0.0.
                oc = f32::from_bits(0x8000_0000);
                let flip = rd8(obj + OBJ_FLAG73) & 1 != 0;
                let mut s = [0u32; 3];
                let ra = lf_checker_rt::callee_thiscall!(11, u32, obj, s.as_mut_ptr() as u32);
                if flip {
                    o30 = neg(rdf(ra));
                    o20 = neg(rdf(ra + 4));
                    o18 = neg(rdf(ra + 8));
                } else {
                    o30 = rdf(ra);
                    o20 = rdf(ra + 4);
                    o18 = rdf(ra + 8);
                }
                let mut hs = [0u32; 8];
                let rp = hook(obj, hs.as_mut_ptr());
                let d = add(
                    add(mul(rdf(rp + 4), o20), mul(rdf(rp), o30)),
                    mul(rdf(rp + 8), o18),
                );
                if d > TWO {
                    o14 = 0.0;
                    o10 = P75;
                } else {
                    o14 = N75;
                    o10 = 0.0;
                    if flip {
                        o14 = P75;
                    }
                }
                if !timer_le_rec() {
                    wr8(edi + REC_MODE, 0);
                    wr32(obj + OBJ_E38, g32(G_TIMER));
                }
                tail(obj, ob, oc, o10, o14, o18)
            }
            // Bytes 13/14/4/5/7/8: single-helper outputs, some negated.
            13 => {
                ob = 0;
                o14 = N75;
                o10 = 0.0;
                oc = lf_checker_rt::callee_cdecl!(12, f32, obj);
                if !timer_le_rec() {
                    wr8(edi + REC_MODE, 0);
                    wr32(obj + OBJ_E38, g32(G_TIMER));
                }
                tail(obj, ob, oc, o10, o14, o18)
            }
            14 => {
                ob = 0;
                o14 = N75;
                o10 = 0.0;
                o18 = lf_checker_rt::callee_cdecl!(12, f32, obj);
                oc = neg(o18);
                if !timer_le_rec() {
                    wr8(edi + REC_MODE, 0);
                    wr32(obj + OBJ_E38, g32(G_TIMER));
                }
                tail(obj, ob, oc, o10, o14, o18)
            }
            4 => {
                ob = 1;
                oc = lf_checker_rt::callee_cdecl!(12, f32, obj);
                o14 = 0.0;
                o10 = 0.0;
                if !timer_le_rec() {
                    wr8(edi + REC_MODE, 0);
                }
                tail(obj, ob, oc, o10, o14, o18)
            }
            5 => {
                ob = 1;
                o18 = lf_checker_rt::callee_cdecl!(12, f32, obj);
                oc = neg(o18);
                o14 = 0.0;
                o10 = 0.0;
                if !timer_le_rec() {
                    wr8(edi + REC_MODE, 0);
                }
                tail(obj, ob, oc, o10, o14, o18)
            }
            7 => {
                ob = 0;
                oc = lf_checker_rt::callee_cdecl!(12, f32, obj);
                o14 = ONE;
                o10 = 0.0;
                if !timer_le_rec() {
                    wr8(edi + REC_MODE, 0);
                }
                tail(obj, ob, oc, o10, o14, o18)
            }
            8 => {
                ob = 0;
                o18 = lf_checker_rt::callee_cdecl!(12, f32, obj);
                oc = neg(o18);
                o14 = ONE;
                o10 = 0.0;
                if !timer_le_rec() {
                    wr8(edi + REC_MODE, 0);
                }
                tail(obj, ob, oc, o10, o14, o18)
            }
            // Byte 6: constant outputs.
            6 => {
                ob = 1;
                o14 = 0.0;
                oc = 0.0;
                o10 = 0.0;
                if !timer_le_rec() {
                    wr8(edi + REC_MODE, 0);
                }
                tail(obj, ob, oc, o10, o14, o18)
            }
            // Byte 9: half output.
            9 => {
                ob = 0;
                o14 = HALF;
                oc = 0.0;
                o10 = 0.0;
                if !timer_le_rec() {
                    wr8(edi + REC_MODE, 0);
                }
                tail(obj, ob, oc, o10, o14, o18)
            }
            // Byte 25: probe length against 15, helper ratio below.
            25 => {
                let mut hs = [0u32; 8];
                let rp = hook(obj, hs.as_mut_ptr());
                let len = add(mul(rdf(rp), rdf(rp)), mul(rdf(rp + 4), rdf(rp + 4))).sqrt();
                o18 = len;
                if len > LEN15 {
                    oc = 0.0;
                    o14 = 0.0;
                    ob = 0;
                    o10 = ONE;
                } else {
                    ob = 1;
                    oc = lf_checker_rt::callee_cdecl!(12, f32, obj);
                    o14 = 0.0;
                    o10 = 0.0;
                    if HALF > len {
                        wr8(edi + REC_MODE, 0);
                    }
                }
                if !timer_le_rec() {
                    wr8(edi + REC_MODE, 0);
                }
                tail(obj, ob, oc, o10, o14, o18)
            }
            // Byte 26: like 25 with a negated helper answer.
            26 => {
                let mut hs = [0u32; 8];
                let rp = hook(obj, hs.as_mut_ptr());
                let len = add(mul(rdf(rp), rdf(rp)), mul(rdf(rp + 4), rdf(rp + 4))).sqrt();
                o30 = len;
                if len > LEN15 {
                    oc = 0.0;
                    o14 = 0.0;
                    ob = 0;
                    o10 = ONE;
                } else {
                    ob = 1;
                    o18 = lf_checker_rt::callee_cdecl!(12, f32, obj);
                    oc = neg(o18);
                    o14 = 0.0;
                    o10 = 0.0;
                    if HALF > len {
                        wr8(edi + REC_MODE, 0);
                    }
                }
                if !timer_le_rec() {
                    wr8(edi + REC_MODE, 0);
                }
                tail(obj, ob, oc, o10, o14, o18)
            }
            // Byte 27: probe length with a flag output.
            27 => {
                oc = 0.0;
                o14 = 0.0;
                let mut hs = [0u32; 8];
                let rp = hook(obj, hs.as_mut_ptr());
                let len = add(mul(rdf(rp), rdf(rp)), mul(rdf(rp + 4), rdf(rp + 4))).sqrt();
                if len > LEN15 {
                    o10 = ONE;
                } else {
                    ob = 1;
                    o10 = 0.0;
                    if HALF > len {
                        wr8(edi + REC_MODE, 0);
                    }
                }
                if !timer_le_rec() {
                    wr8(edi + REC_MODE, 0);
                }
                tail(obj, ob, oc, o10, o14, o18)
            }
            // Bytes 10/11/20/21: staged outputs with byte-selected tweaks.
            10 | 11 | 20 | 21 => {
                oc = QNEG;
                ob = 0;
                if sw == 0x0b || sw == 0x15 {
                    oc = QPOS;
                }
                o14 = 0.0;
                o10 = C001;
                if g32(G_TIMER) > rd32(edi + REC_T).wrapping_sub(0x4e2) {
                    oc = neg(oc);
                }
                if sw == 0x15 || sw == 0x14 {
                    o10 = C03;
                }
                if !timer_le_rec() {
                    if sw == 0x14 || sw == 0x15 {
                        wr8(edi + REC_MODE, 1);
                        wr32(edi + REC_T, g32(G_TIMER).wrapping_add(0xfa0));
                    } else {
                        wr8(edi + REC_MODE, 0);
                    }
                }
                tail(obj, ob, oc, o10, o14, o18)
            }
            // Byte 19: input float through a gain, clamped both sides.
            19 => {
                ob = 0;
                o14 = 0.0;
                o10 = if !timer_le_rec() { ONE } else { 0.0 };
                let x1 = rdf(obj + OBJ_O88);
                let x0 = mul(f32::from_bits(g32(G_FSRC)), TWO_HALF);
                if x1 > 0.0 {
                    let s = add(x0, x1);
                    oc = if s > HALF { HALF } else { s };
                } else {
                    let d = sub(x1, x0);
                    oc = if !(NHALF > d) { d } else { NHALF };
                }
                if !timer_le_rec() {
                    wr8(edi + REC_MODE, 0);
                }
                tail(obj, ob, oc, o10, o14, o18)
            }
            // Bytes 22/28: probe direction dot, thresholded.
            22 | 28 => {
                ob = 0;
                oc = 0.0;
                let base = rd32(obj + OBJ_BASE);
                let mut hs = [0u32; 8];
                let rp = hook(obj, hs.as_mut_ptr());
                let d = add(
                    add(mul(rdf(rp + 4), rdf(base + BASE_DX + 4)), mul(rdf(rp), rdf(base + BASE_DX))),
                    mul(rdf(rp + 8), rdf(base + BASE_DX + 8)),
                );
                if d > TENTH {
                    o14 = 0.0;
                    o10 = HALF;
                } else {
                    o14 = NHALF;
                    if rd8(edi + REC_MODE) == 0x1c {
                        o14 = f32::from_bits(0xBF80_0000);
                    }
                    o10 = 0.0;
                }
                if !timer_le_rec() {
                    wr8(edi + REC_MODE, 0);
                }
                tail(obj, ob, oc, o10, o14, o18)
            }
            // Byte 23: scaled input, direction probe, offset helper.
            23 => {
                ob = 0;
                oc = rdf(obj + OBJ_O88);
                let x0 = mul(f32::from_bits(g32(G_FSRC)), FIFTY);
                let mut s = [0u32; 3];
                let ra = lf_checker_rt::callee_thiscall!(11, u32, obj, s.as_mut_ptr() as u32);
                o14 = ONE;
                o10 = 0.0;
                o18 = mul(x0, K0012);
                let x1 = o18;
                o20 = mul(x1, rdf(ra));
                o18 = mul(rdf(ra + 4), x1);
                o30 = mul(rdf(ra + 8), x1);
                let mut hs = [0u32; 8];
                let rp = hook(obj, hs.as_mut_ptr());
                let mut bs = [0u32; 3];
                bs[0] = add(rdf(rp), o20).to_bits();
                bs[1] = add(rdf(rp + 4), o18).to_bits();
                bs[2] = add(rdf(rp + 8), o30).to_bits();
                lf_checker_rt::callee_thiscall!(13, u32, obj, bs.as_mut_ptr() as u32);
                if !timer_le_rec() {
                    wr8(edi + REC_MODE, 0);
                }
                tail(obj, ob, oc, o10, o14, o18)
            }
            // Bytes 15-18: clear on expiry, then the default fill.
            15 | 16 | 17 | 18 => {
                if !timer_le_rec() {
                    wr8(edi + REC_MODE, 0);
                }
                ob = fill6(obj, edi, ob, &mut oc, &mut o10, &mut o14);
                tail(obj, ob, oc, o10, o14, o18)
            }
            // Default (also bytes 2 and 12): fill helper, straight to tail.
            _ => {
                ob = fill6(obj, edi, ob, &mut oc, &mut o10, &mut o14);
                tail(obj, ob, oc, o10, o14, o18)
            }
        }
    }
});
