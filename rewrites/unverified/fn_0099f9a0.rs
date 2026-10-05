// original: 0x0099F9A0 PAIN_POSITIONED (symbols)

/// Start a positioned pain vocalisation for a creature, or reject the request.
///
/// `this` is the audio controller; `kind` (stack arg 0) selects the pain
/// flavour (2, 4 and 5 map to slots 1, 2 and 3, anything else to slot 0).
/// Returns 1 when the vocalisation started and 0 when it was rejected.
///
/// The request passes a chain of gates before anything audible happens: a
/// global enable byte, a state word, two slot pointers that must already be
/// empty (live ones are released through the release callee first), a
/// distance check (a float answer from the range callee must not exceed 40),
/// and an availability probe on the voice pool. Rejection at any gate
/// returns 0.
///
/// Past the gates the controller reserves two voice slots through the voice
/// pool (its answers select ids and volumes), builds a parameter block on its
/// frame (volumes, pitch, position), and submits it twice through the submit
/// callee (once per slot). Each submission writes the new voice handle back
/// into its slot; a null second handle faults reading through the kind value
/// as a pointer (both sides fault identically), a null first handle ends the
/// attempt. Volumes derive from a global gain multiplied by small constants,
/// clamped at zero from below; positions come from a locator callee through
/// out-slots. The float operation order is the original's.
///
/// Frame model: every stack slot the original touches is a byte offset in a
/// zeroed 256-byte array (canonical slot = original esp offset minus depth),
/// matching the checker's constant zero stack fill. The scratch registers do
/// not survive stub calls, so every callee input held in a local is set fresh
/// from one; the one stale push is overwritten with a constant before its
/// call and needs no preserved register.
///
/// Original: thiscall, two stack words (the second is unread), al result.
lf_checker_rt::export!(thiscall, rw_0099F9A0(this: u32, kind: u32, _unused: u32) -> u32 {
    unsafe {
        const ENABLE: u32 = 0x115dce6;
        const STATE: u32 = 0x11f7060;
        const TICK_A: u32 = 0x12088b4;
        const TICK_B: u32 = 0xf1c040;
        const READY: u32 = 0x1037720;
        const READY_SKIP: u32 = 0x12;
        const LIMIT: u32 = 0x11618fc;
        const LIMIT_BIAS: u32 = 0x1038e00;
        const STASH: u32 = 0x12843b4;
        const GAIN: u32 = 0x12831d8;
        const ID_TABLE: u32 = 0x128859c;
        const SLOT_TABLE: u32 = 0x12843e8;
        const DIST_MAX: f32 = f32::from_bits(1109393408);
        const C4: f32 = f32::from_bits(1082130432);
        const CN7: f32 = f32::from_bits(3235905536);
        const C3: f32 = f32::from_bits(1077936128);
        const C6: f32 = f32::from_bits(1086324736);
        const C2: f32 = f32::from_bits(1073741824);
        const UNIT: f32 = f32::from_bits(1065353216);
        const PITCH: f32 = f32::from_bits(1061752795);
        const POS_SEED: u32 = 0x46abe000;
        const VOICE_OBJ: u32 = 0x1288780;
        const RANGE_OBJ: u32 = 0x115def0;
        const LOC_OBJ: u32 = 0x1165880;
        const THIS_OFF: u32 = 8;
        const SLOT0: u32 = 0xb0;
        const SLOT1: u32 = 0xb4;
        const COUNT: u32 = 0xb8;
        const HI: u32 = 0xbc;
        const PLAYING: u32 = 0xc0;
        const BUSY0: u32 = 0x188;
        const BUSY1: u32 = 0x1a4;
        const BASE_VOL: u32 = 0x1f0;
        const AUX: u32 = 0x1f4;
        const CHAN: u32 = 0xc;
        const HANDLE: u32 = 0x204;
        const FMT0: u32 = 0xe9114c;
        const FMT1: u32 = 0xe911c0;
        const FMT2: u32 = 0xe911d8;
        const FMT3: u32 = 0xe911e8;
        const C_RELEASE: u32 = 1;
        const C_RANGE: u32 = 2;
        const C_QUIET: u32 = 3;
        const C_AUDIBLE: u32 = 4;
        const C_PROBE: u32 = 5;
        const C_IDS: u32 = 6;
        const C_VOL: u32 = 7;
        const C_PICK: u32 = 8;
        const C_PREP: u32 = 9;
        const C_FORMAT: u32 = 10;
        const C_FILL: u32 = 11;
        const C_CHECK: u32 = 12;
        const C_SUBMIT: u32 = 13;
        const C_LOCATE: u32 = 14;
        const C_TOUCH: u32 = 15;
        const C_APPLY: u32 = 16;
        const C_SETVOL: u32 = 17;
        const C_RENDER: u32 = 18;
        const C_COMMIT0: u32 = 19;
        const C_COMMIT1: u32 = 20;
        const C_SETW: u32 = 21;
        const C_SETX: u32 = 22;
        const C_STORE: u32 = 23;
        const C_CLOSE: u32 = 24;
        const C_ROUTE: u32 = 25;
        const C_FIN: u32 = 26;
        const C_BEGIN: u32 = 27;
        const C_EAR: u32 = 28;
        const C_PAN: u32 = 29;
        const C_COOKIE: u32 = 30;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
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
        let mut frame = [0u8; 256];
        #[inline(always)]
        fn rf(f: &[u8; 256], c: i32) -> u32 {
            let i = (c + 256) as usize;
            u32::from_ne_bytes([f[i], f[i + 1], f[i + 2], f[i + 3]])
        }
        #[inline(always)]
        fn rff(f: &[u8; 256], c: i32) -> f32 {
            f32::from_bits(rf(f, c))
        }
        #[inline(always)]
        fn rb(f: &[u8; 256], c: i32) -> u8 {
            f[(c + 256) as usize]
        }
        #[inline(always)]
        fn wf(f: &mut [u8; 256], c: i32, v: u32) {
            let i = (c + 256) as usize;
            let b = v.to_ne_bytes();
            f[i] = b[0];
            f[i + 1] = b[1];
            f[i + 2] = b[2];
            f[i + 3] = b[3];
        }
        #[inline(always)]
        fn wff(f: &mut [u8; 256], c: i32, v: f32) {
            wf(f, c, v.to_bits())
        }
        #[inline(always)]
        fn wb(f: &mut [u8; 256], c: i32, v: u8) {
            f[(c + 256) as usize] = v;
        }
        #[inline(always)]
        fn fp(f: &mut [u8; 256], c: i32) -> u32 {
            unsafe { f.as_mut_ptr().add((c + 256) as usize) as u32 }
        }
        macro_rules! g32 {
            ($va:expr) => {
                rd32(lf_checker_rt::relocated($va))
            };
        }

        if rd8(lf_checker_rt::relocated(ENABLE)) == 0 {
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32);
            return 0;
        }
        wf(&mut frame, -216, kind);
        if g32!(STATE) != 1 {
            if g32!(TICK_A) != g32!(TICK_B) || g32!(READY) == READY_SKIP || kind > rd32(this.wrapping_add(COUNT)) {
            } else if rd32(this.wrapping_add(HI)) > g32!(LIMIT) {
                lf_checker_rt::callee_cdecl!(C_COOKIE, u32);
                return 0;
            }
        }
        let s0 = rd32(this.wrapping_add(SLOT0));
        if s0 != 0 && rd32(this.wrapping_add(COUNT)) <= 2 && kind >= 4 {
            lf_checker_rt::callee_thiscall!(C_RELEASE, u32, s0, 0u32);
        }
        let s1 = rd32(this.wrapping_add(SLOT1));
        if s1 != 0 && rd32(this.wrapping_add(COUNT)) <= 2 && kind >= 4 {
            lf_checker_rt::callee_thiscall!(C_RELEASE, u32, s1, 0u32);
        }
        if rd32(this.wrapping_add(SLOT0)) != 0 {
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32);
            return 0;
        }
        if rd32(this.wrapping_add(SLOT1)) != 0 {
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32);
            return 0;
        }
        let inner = rd32(this.wrapping_add(THIS_OFF));
        let range_arg = rd32(inner.wrapping_add(0x20)).wrapping_add(0x30);
        let dist: f32 = lf_checker_rt::callee_thiscall!(C_RANGE, f32, RANGE_OBJ, range_arg);
        wff(&mut frame, -208, dist);
        if dist > DIST_MAX {
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32);
            return 0;
        }
        let mode: u32;
        if rd8(inner.wrapping_add(0x218)) == 0 && rd8(inner.wrapping_add(0x219)) != 0 {
            let q: u32 = lf_checker_rt::callee_thiscall!(C_QUIET, u32, RANGE_OBJ);
            if (q as u8) == 0 {
                mode = 0;
            } else {
                let a: u32 = lf_checker_rt::callee_thiscall!(C_AUDIBLE, u32, inner);
                mode = if (a as u8) == 0 { 2 } else { 1 };
            }
        } else {
            let a: u32 = lf_checker_rt::callee_thiscall!(C_AUDIBLE, u32, inner);
            mode = if (a as u8) == 0 { 2 } else { 1 };
        }
        wf(&mut frame, -212, mode);
        let slot = if kind == 2 {
            1
        } else if kind == 4 {
            2
        } else if kind == 5 {
            3
        } else {
            0
        };
        wf(&mut frame, -192, slot);
        let ok: u32 = lf_checker_rt::callee_thiscall!(C_PROBE, u32, VOICE_OBJ, mode);
        if (ok as u8) == 0 {
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32);
            return 0;
        }
        let mut esi = rf(&frame, -212);
        let ids: u32 = lf_checker_rt::callee_thiscall!(C_IDS, u32, VOICE_OBJ, esi);
        wf(&mut frame, -180, ids);
        let vol: u32 = lf_checker_rt::callee_thiscall!(C_VOL, u32, VOICE_OBJ, rf(&frame, -192), esi);
        wf(&mut frame, -208, vol);
        esi = lf_checker_rt::callee_thiscall!(C_PICK, u32, VOICE_OBJ, esi);
        wb(&mut frame, -72, 0);
        lf_checker_rt::callee_cdecl!(C_PREP, u32);
        let idval = rd32(
            lf_checker_rt::relocated(ID_TABLE).wrapping_add(rf(&frame, -212).wrapping_mul(4)),
        );
        let f1 = fp(&mut frame, -71);
        let f2 = fp(&mut frame, -72);
        lf_checker_rt::callee_cdecl!(C_FORMAT, u32, f2, 0x40u32, FMT0, idval, esi, f1, 0u32, 0x3fu32);
        let gain = f32::from_bits(g32!(GAIN));
        let mut x0 = fmul(C4, gain);
        let mut x2 = fsub(UNIT, gain);
        x0 = fadd(x0, CN7);
        x2 = fmul(x2, C3);
        wff(&mut frame, -188, x0);
        x0 = fmul(C6, gain);
        x2 = fadd(x2, x0);
        wff(&mut frame, -220, x2);
        let fa = fp(&mut frame, -144);
        lf_checker_rt::callee_thiscall!(C_FILL, u32, fa);
        let e180 = rf(&frame, -180);
        wf(&mut frame, -120, e180);
        wb(&mut frame, -74, (rb(&frame, -74) & 0xef) | 8);
        wf(&mut frame, -116, g32!(STASH));
        wb(
            &mut frame,
            -184,
            if rf(&frame, -216) >= 2 { 1 } else { 0 },
        );
        wr32(this.wrapping_add(BASE_VOL), 0);
        let chk: u32 = lf_checker_rt::callee_thiscall!(C_CHECK, u32, this, inner);
        if (chk as u8) != 0 {
            wb(&mut frame, -74, rb(&frame, -74) & 0xdf);
            let fa2 = fp(&mut frame, -144);
            lf_checker_rt::callee_thiscall!(
                C_SUBMIT,
                u32,
                this,
                FMT1,
                this.wrapping_add(SLOT0),
                fa2,
                0xffffffffu32,
                0u32,
                0u32
            );
            wb(&mut frame, -184, 0);
        } else {
            let la = fp(&mut frame, -233);
            let lb = fp(&mut frame, -232);
            wf(&mut frame, -232, 0);
            wb(&mut frame, -233, 0);
            lf_checker_rt::callee_thiscall!(C_LOCATE, u32, LOC_OBJ, lb, la);
            if rb(&frame, -233) == 0 {
                let f1v = rff(&frame, -232);
                let mut y0 = fmul(C2, f1v);
                let mut y2 = fsub(UNIT, f1v);
                y0 = fadd(y0, y2);
                y2 = rff(&frame, -220);
                y0 = fmul(y0, y2);
                wff(&mut frame, -220, y0);
                y0 = fmul(C3, f1v);
                y0 = fadd(y0, rff(&frame, -188));
                wff(&mut frame, -188, y0);
            }
            let t: u32 = lf_checker_rt::callee_thiscall!(C_TOUCH, u32, inner.wrapping_add(0x3c0));
            wf(&mut frame, -112, t);
            wb(&mut frame, -74, rb(&frame, -74) & 0xdf);
            let ap: u32 = lf_checker_rt::callee_thiscall!(C_APPLY, u32, this, rff(&frame, -220).to_bits());
            wf(&mut frame, -116, ap);
            let fa3 = fp(&mut frame, -144);
            lf_checker_rt::callee_thiscall!(
                C_SUBMIT,
                u32,
                this,
                FMT2,
                this.wrapping_add(SLOT0),
                fa3,
                0xffffffffu32,
                0u32,
                0u32
            );
        }
        let handle = rd32(this.wrapping_add(SLOT0));
        if handle == 0 {
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32);
            return 0;
        }
        let mut vx = rff(&frame, -188);
        if !(0.0f32 > vx) {
            vx = 0.0;
            wff(&mut frame, -188, vx);
        }
        lf_checker_rt::callee_thiscall!(C_SETVOL, u32, handle, vx.to_bits());
        let k = rf(&frame, -216);
        let tbl1 = rd32(lf_checker_rt::relocated(SLOT_TABLE).wrapping_add(k.wrapping_mul(4)));
        let g1 = fp(&mut frame, -72);
        let r1: u32 = lf_checker_rt::callee_cdecl!(C_RENDER, u32, g1, 0u32);
        let cm0: u32 =
            lf_checker_rt::callee_thiscall!(C_COMMIT0, u32, handle, r1, tbl1, rf(&frame, -208), 0u32);
        if (cm0 as u8) == 0 {
            lf_checker_rt::callee_thiscall!(C_RELEASE, u32, handle, 0u32);
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32);
            return 0;
        }
        wr8(this.wrapping_add(BUSY0), rd8(this.wrapping_add(BUSY0)) | BUSY_BIT);
        wr8(this.wrapping_add(BUSY1), rd8(this.wrapping_add(BUSY1)) | BUSY_BIT);
        wf(&mut frame, -232, 0);
        wf(&mut frame, -220, POS_SEED);
        let pa = fp(&mut frame, -160);
        let pb = fp(&mut frame, -220);
        let pc = fp(&mut frame, -232);
        lf_checker_rt::callee_thiscall!(C_POS, u32, this, pc, pb, pa);
        lf_checker_rt::callee_thiscall!(C_SETW, u32, handle, rff(&frame, -220).to_bits());
        lf_checker_rt::callee_thiscall!(C_SETX, u32, handle, rff(&frame, -232).to_bits());
        let sa = fp(&mut frame, -160);
        lf_checker_rt::callee_thiscall!(C_STORE, u32, handle, sa);
        let h = rd32(this.wrapping_add(HANDLE));
        wr32(this.wrapping_add(CHAN), 0);
        if h != 0 {
            lf_checker_rt::callee_cdecl!(C_CLOSE, u32, h);
            wr32(this.wrapping_add(HANDLE), 0);
        }
        lf_checker_rt::callee_thiscall!(C_ROUTE, u32, handle, rf(&frame, -180), 0u32, 0u32);
        let fin: u32 =
            lf_checker_rt::callee_thiscall!(C_FIN, u32, VOICE_OBJ, rf(&frame, -212), rf(&frame, -192));
        esi = rf(&frame, -216);
        wr32(this.wrapping_add(COUNT), esi);
        wr32(
            this.wrapping_add(HI),
            g32!(LIMIT).wrapping_add(rd32(lf_checker_rt::relocated(LIMIT_BIAS))),
        );
        if esi >= 4 {
            wr8(this.wrapping_add(PLAYING), 1);
        }
        wf(&mut frame, -216, 0);
        wf(&mut frame, -212, fin);
        let ba = fp(&mut frame, -216);
        lf_checker_rt::callee_thiscall!(
            C_BEGIN,
            u32,
            this,
            rf(&frame, -184),
            this.wrapping_add(AUX),
            ba,
            0u32
        );
        wf(&mut frame, -104, rf(&frame, -216));
        let fa4 = fp(&mut frame, -144);
        lf_checker_rt::callee_thiscall!(
            C_SUBMIT,
            u32,
            this,
            FMT3,
            this.wrapping_add(SLOT1),
            fa4,
                0xffffffffu32,
            0u32,
            0u32
        );
        let h2 = rd32(this.wrapping_add(SLOT1));
        if h2 == 0 {
            // esi still holds kind here: the original reads through it and
            // faults on both sides.
            let dead = rd32(esi);
            lf_checker_rt::callee_thiscall!(C_RELEASE, u32, dead, 0u32);
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32);
            return 1;
        }
        lf_checker_rt::callee_thiscall!(C_SETVOL, u32, h2, rff(&frame, -188).to_bits());
        let tbl2 = rd32(lf_checker_rt::relocated(SLOT_TABLE).wrapping_add(esi.wrapping_mul(4)));
        let g2 = fp(&mut frame, -72);
        let r2: u32 = lf_checker_rt::callee_cdecl!(C_RENDER, u32, g2, 0u32);
        esi = this.wrapping_add(SLOT1);
        let cm1: u32 =
            lf_checker_rt::callee_thiscall!(C_COMMIT1, u32, h2, r2, tbl2, rf(&frame, -208), 0u32);
        if (cm1 as u8) == 0 {
            lf_checker_rt::callee_thiscall!(C_RELEASE, u32, rd32(esi), 0u32);
            lf_checker_rt::callee_cdecl!(C_COOKIE, u32);
            return 1;
        }
        lf_checker_rt::callee_thiscall!(C_SETW, u32, rd32(esi), rff(&frame, -220).to_bits());
        let mut z = rff(&frame, -232);
        z = fadd(z, f32::from_bits(rd32(this.wrapping_add(BASE_VOL))));
        z = fadd(z, f32::from_bits(rd32(rf(&frame, -212))));
        lf_checker_rt::callee_thiscall!(C_SETX, u32, rd32(esi), z.to_bits());
        let ear: u32 = lf_checker_rt::callee_thiscall!(C_EAR, u32, RANGE_OBJ, 0u32);
        let e0 = f32::from_bits(rd32(ear));
        let e1 = f32::from_bits(rd32(ear.wrapping_add(4)));
        let e2 = f32::from_bits(rd32(ear.wrapping_add(8)));
        let mut p = fsub(rff(&frame, -160), e0);
        wff(&mut frame, -208, e0);
        wff(&mut frame, -176, p);
        p = fsub(rff(&frame, -156), e1);
        wff(&mut frame, -184, e1);
        wff(&mut frame, -232, e2);
        wff(&mut frame, -172, p);
        p = fsub(rff(&frame, -152), e2);
        wff(&mut frame, -168, p);
        let na = fp(&mut frame, -176);
        lf_checker_rt::callee_thiscall!(C_PAN, u32, na, PITCH.to_bits(), 0x7au32);
        let mut q = rff(&frame, -184);
        q = fadd(q, rff(&frame, -208));
        wff(&mut frame, -176, q);
        q = rff(&frame, -176);
        q = fadd(q, rff(&frame, -184));
        wff(&mut frame, -172, q);
        q = rff(&frame, -168);
        q = fadd(q, rff(&frame, -232));
        wff(&mut frame, -168, q);
        let ra = fp(&mut frame, -176);
        lf_checker_rt::callee_thiscall!(C_STORE, u32, rd32(esi), ra);
        lf_checker_rt::callee_thiscall!(C_ROUTE, u32, rd32(esi), rf(&frame, -180), 0u32, 0u32);
        lf_checker_rt::callee_cdecl!(C_COOKIE, u32);
        1
    }
});
