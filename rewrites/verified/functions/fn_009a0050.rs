// original: 0x009A0050 audio_start_positioned (proposed)

/// Start a positioned audio event with full parameter setup, or reject it.
///
/// `this` is the audio controller. Stack args: `a0` a mode id, `a1` a name
/// string, `a2` a priority (non-positive values go through a preparation
/// callee first and may exit early), `a3` a flags word, `a4` a word whose low
/// byte selects the volume flavour, `a5` a word whose low byte selects the
/// routing path. Returns the low byte of the last callee answer on the exit
/// path taken (0 on most paths).
///
/// After the same enable/state gates as its sibling, the controller clears
/// two voice slots, initialises four bus bindings through the render callee
/// in a loop, and prepares the request. A flavour dispatch on a byte (from a
/// constant or the request, whichever the select constant picks) computes the
/// base volume and pan through several float paths. Two voice slots are then
/// submitted through the submit callee exactly like the sibling function:
/// handles written back into the slots, a table lookup mapping handle bytes
/// to a voice entry, volume/pan/position blocks, an ear-calibrated refinement
/// and a commit probe per slot. The tail stores configuration, routes the
/// voice (two routing callees or a dial/ping pair), scans and hashes the
/// voice, emits and broadcasts a setup packet, allocates a name buffer sized
/// from the name string length, formats into it, renders and frees it.
///
/// Frame model: every stack slot is a byte offset in a zeroed 448-byte array
/// (canonical slot = original esp offset minus depth), matching the checker's
/// constant zero stack fill. The two incoming argument slots the original
/// rewrites are held as shadow locals; the physical store to the incoming
/// stack is not replicated (see the contract's narrowed list if the stack
/// comparison needs it switched off).
///
/// Original: thiscall, six stack words, al result.
lf_checker_rt::export!(thiscall, rw_009A0050(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0x11f7060;
        const TICK_A: u32 = 0x12088b4;
        const TICK_B: u32 = 0xf1c040;
        const READY: u32 = 0x1037720;
        const READY_SKIP: u32 = 0x12;
        const SELECT: u32 = 0x1038d44;
        const BUS_TABLE: u32 = 0x1038e04;
        const GAIN: u32 = 0x12831d8;
        const G1: u32 = 0x115d968;
        const G2: u32 = 0x115d988;
        const ROW_STRIDE: u32 = 0x6f40;
        const ROW_BIAS: u32 = 0x6f14;
        const FLAG_A: u32 = 0x1283049;
        const STASH: u32 = 0x1284398;
        const VOICE_A: u32 = 0x128456c;
        const VOICE_B: u32 = 0x1284570;
        const SUB_FMT: u32 = 0x12844e4;
        const CFG_A: u32 = 0x11735a4;
        const CFG_B: u32 = 0x11735b4;
        const QUERY_ARG: u32 = 0x1038d10;
        const C4: f32 = f32::from_bits(1082130432);
        const CN4: f32 = f32::from_bits(3229614080);
        const CN7: f32 = f32::from_bits(3235905536);
        const C04: f32 = f32::from_bits(1053609165);
        const CN2: f32 = f32::from_bits(3221225472);
        const C6: f32 = f32::from_bits(1086324736);
        const C3: f32 = f32::from_bits(1077936128);
        const CN3: f32 = f32::from_bits(3225419776);
        const C12: f32 = f32::from_bits(1094713344);
        const UNIT: f32 = f32::from_bits(1065353216);
        const PITCH: f32 = f32::from_bits(1061752795);
        const POS_SEED: u32 = 0x46abe000;
        const VOICE_OBJ: u32 = 0x1288780;
        const RANGE_OBJ: u32 = 0x115def0;
        const PROBE_OBJ: u32 = 0x128e400;
        const FMT_A: u32 = 0xe91158;
        const FMT_B: u32 = 0xe911d0;
        const FMT_C: u32 = 0xe91200;
        const SLOT0: u32 = 0x60;
        const SLOT1: u32 = 0x64;
        const BUSY0: u32 = 0x188;
        const BUSY1: u32 = 0x1a4;
        const BUSY_BIT: u8 = 2;
        const BASE_VOL: u32 = 0x1f0;
        const AUX: u32 = 0x1f4;
        const CHAN: u32 = 0xc;
        const HANDLE: u32 = 0x204;
        const NAME_LEN_OFF: u32 = 0x90;
        const I_RENDER_A: u32 = 1;
        const I_RENDER_B: u32 = 2;
        const I_PREP2: u32 = 3;
        const I_FMT5: u32 = 4;
        const I_FMT4F: u32 = 5;
        const I_FMT4H: u32 = 6;
        const I_PROBE2: u32 = 7;
        const I_FILL: u32 = 8;
        const I_CHECK: u32 = 9;
        const I_CHECK2: u32 = 10;
        const I_APPLY: u32 = 11;
        const I_SUBMIT: u32 = 12;
        const I_ROUTE2: u32 = 13;
        const I_RELEASE: u32 = 14;
        const I_COMMIT_A: u32 = 15;
        const I_COMMIT_B: u32 = 16;
        const I_POS: u32 = 17;
        const I_SETW: u32 = 18;
        const I_SETX: u32 = 19;
        const I_STORE: u32 = 20;
        const I_BEGIN: u32 = 21;
        const I_SUBMIT2: u32 = 22;
        const I_SETVOL: u32 = 23;
        const I_EAR: u32 = 24;
        const I_PAN: u32 = 25;
        const I_CLOSE: u32 = 26;
        const I_QUERY: u32 = 27;
        const I_ROUTE: u32 = 28;
        const I_DIAL: u32 = 29;
        const I_PING: u32 = 30;
        const I_SCAN: u32 = 31;
        const I_LOOKUP: u32 = 32;
        const I_HASH: u32 = 33;
        const I_EMIT: u32 = 34;
        const I_SEND_A: u32 = 35;
        const I_SEND_B: u32 = 36;
        const I_BROADCAST: u32 = 37;
        const I_VOICE4: u32 = 38;
        const I_ALLOC: u32 = 39;
        const I_COOKIE: u32 = 40;

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
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read() }
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
        let mut frame = [0u8; 448];
        #[inline(always)]
        fn rf(f: &[u8; 448], c: i32) -> u32 {
            let i = (c + 448) as usize;
            u32::from_ne_bytes([f[i], f[i + 1], f[i + 2], f[i + 3]])
        }
        #[inline(always)]
        fn rff(f: &[u8; 448], c: i32) -> f32 {
            f32::from_bits(rf(f, c))
        }
        #[inline(always)]
        fn rb(f: &[u8; 448], c: i32) -> u8 {
            f[(c + 448) as usize]
        }
        #[inline(always)]
        fn wf(f: &mut [u8; 448], c: i32, v: u32) {
            let i = (c + 448) as usize;
            let b = v.to_ne_bytes();
            f[i] = b[0];
            f[i + 1] = b[1];
            f[i + 2] = b[2];
            f[i + 3] = b[3];
        }
        #[inline(always)]
        fn wff(f: &mut [u8; 448], c: i32, v: f32) {
            wf(f, c, v.to_bits())
        }
        #[inline(always)]
        fn wb(f: &mut [u8; 448], c: i32, v: u8) {
            f[(c + 448) as usize] = v;
        }
        #[inline(always)]
        fn fp(f: &mut [u8; 448], c: i32) -> u32 {
            unsafe { f.as_mut_ptr().add((c + 448) as usize) as u32 }
        }
        macro_rules! g32 {
            ($va:expr) => {
                rd32(lf_checker_rt::relocated($va))
            };
        }
        let range_obj = lf_checker_rt::relocated(RANGE_OBJ);
        let voice_obj = lf_checker_rt::relocated(VOICE_OBJ);
        let probe_obj = lf_checker_rt::relocated(PROBE_OBJ);
        let fmt_a = lf_checker_rt::relocated(FMT_A);
        let fmt_b = lf_checker_rt::relocated(FMT_B);
        let fmt_c = lf_checker_rt::relocated(FMT_C);

        wf(&mut frame, -380, a0);
        wf(&mut frame, -420, a1);
        wf(&mut frame, -376, this);
        wf(&mut frame, -372, a3);
        if g32!(STATE) == 1
            || g32!(TICK_A) != g32!(TICK_B)
            || g32!(READY) == READY_SKIP
        {
            lf_checker_rt::callee_cdecl!(I_COOKIE, u32,);
            return 0;
        }
        let sel = g32!(SELECT);
        let flavour = if (sel as i32) >= 0 { sel & 0xff } else { a4 & 0xff };
        wf(&mut frame, -400, flavour);
        let mut a4s = (a4 & !0xff) | (flavour & 0xff);
        if rd32(this.wrapping_add(SLOT0)) != 0 || rd32(this.wrapping_add(SLOT1)) != 0 {
            lf_checker_rt::callee_cdecl!(I_COOKIE, u32,);
            return 0;
        }
        let mut i = 0u32;
        while i < 4 {
            let tv = rd32(lf_checker_rt::relocated(BUS_TABLE).wrapping_add(i.wrapping_mul(4)));
            let r: u32 = lf_checker_rt::callee_cdecl!(I_RENDER_A, u32, tv, 0u32);
            wr32(this.wrapping_add(0x70).wrapping_add(i.wrapping_mul(4)), r);
            wr32(this.wrapping_add(0x80).wrapping_add(i.wrapping_mul(4)), 0xffffffffu32);
            i += 1;
        }
        wr8(this.wrapping_add(0x209), 0);
        let r0: u32 = lf_checker_rt::callee_cdecl!(I_RENDER_A, u32, rf(&frame, -380), 0u32);
        wf(&mut frame, -376, r0);
        wf(&mut frame, -416, 0);
        let mut a2s = a2;
        let mut esi_v = a2;
        if (a2 as i32) <= 0 {
            let f = fp(&mut frame, -416);
            let r: u32 = lf_checker_rt::callee_thiscall!(I_PREP2, u32, this, rf(&frame, -376), rf(&frame, -420), f, 0u32, 0u32);
            a2s = r;
            if (r as i32) <= 0 {
                let g = fp(&mut frame, -264);
                let rr: u32 = lf_checker_rt::callee_cdecl!(I_FMT5, u32, g, fmt_a, rf(&frame, -420), rf(&frame, -380), r);
                lf_checker_rt::callee_cdecl!(I_COOKIE, u32,);
                return rr;
            }
            esi_v = r;
        } else {
            let g = fp(&mut frame, -264);
            lf_checker_rt::callee_cdecl!(I_FMT4F, u32, g, fmt_b, rf(&frame, -420), esi_v);
            let g2 = fp(&mut frame, -264);
            lf_checker_rt::callee_cdecl!(I_RENDER_B, u32, g2, 0u32);
        }
        let gain = f32::from_bits(g32!(GAIN));
        wff(&mut frame, -424, 0.0);
        wff(&mut frame, -432, UNIT);
        wff(&mut frame, -436, gain);
        let t: u32 = lf_checker_rt::callee_thiscall!(I_PROBE2, u32, probe_obj);
        if (t as u8) == 0 {
            wb(&mut frame, -425, 0);
            if rd8(lf_checker_rt::relocated(FLAG_A)) != 0 {
                wb(&mut frame, -425, 1);
            }
        } else {
            wb(&mut frame, -425, 1);
        }
        let fa = fp(&mut frame, -336);
        lf_checker_rt::callee_thiscall!(I_FILL, u32, fa);
        wf(&mut frame, -308, g32!(STASH));
        wr32(this.wrapping_add(BASE_VOL), 0);
        let mut eax_v = rf(&frame, -400);
        wb(&mut frame, -392, 0);
        wb(&mut frame, -416, 0);
        let mut flav = (eax_v & 0xff) as u8;
        let const_path = flav == 3 || rb(&frame, -425) != 0;
        if !const_path {
            let inner = rd32(this.wrapping_add(8));
            let mut checked = false;
            let mut join_b = false;
            let mut join_a = false;
            if inner != 0 {
                let c1: u32 = lf_checker_rt::callee_thiscall!(I_CHECK, u32, this, inner);
                if (c1 as u8) != 0 {
                    let c2: u32 = lf_checker_rt::callee_thiscall!(I_CHECK2, u32, this, inner);
                    if (c2 as u8) != 0 {
                        join_b = true;
                    } else {
                        eax_v = rf(&frame, -400);
                        checked = true;
                    }
                } else {
                    eax_v = rf(&frame, -400);
                    checked = true;
                }
            }
            if !join_b {
                let esi2 = rd32(this.wrapping_add(8));
                if esi2 != 0 {
                    let c3: u32 = lf_checker_rt::callee_thiscall!(I_CHECK2, u32, this, esi2);
                    if (c3 as u8) != 0 {
                        join_a = true;
                    } else {
                        eax_v = rf(&frame, -400);
                    }
                }
                if !join_a {
                    flav = (eax_v & 0xff) as u8;
                    if flav == 1 {
                        let mut x = fmul(C4, gain);
                        x = fadd(x, CN3);
                        wff(&mut frame, -424, x);
                    } else if esi2 == 0 {
                        // fall to flavour dispatch on eax_v
                        flav = (eax_v & 0xff) as u8;
                        if flav == 2 {
                            let mut x = fmul(C4, gain);
                            wb(&mut frame, -392, 1);
                            x = fadd(x, CN4);
                            wff(&mut frame, -424, x);
                            join_a = true;
                        } else if flav == 6 {
                            let mut x = fmul(C4, gain);
                            wb(&mut frame, -392, 1);
                            wb(&mut frame, -416, 1);
                            x = fadd(x, CN4);
                            wff(&mut frame, -424, x);
                            join_a = true;
                        } else if flav == 4 {
                            wff(&mut frame, -424, CN7);
                            wff(&mut frame, -432, C04);
                            join_b = true;
                        } else if flav == 5 {
                            wff(&mut frame, -424, CN2);
                            join_b = true;
                        } else {
                            let mut x2 = fsub(UNIT, gain);
                            let mut x1 = fmul(C4, gain);
                            let mut x0 = fmul(C6, gain);
                            x2 = fmul(x2, C3);
                            x1 = fadd(x1, CN7);
                            x2 = fadd(x2, x0);
                            wff(&mut frame, -424, x1);
                            wff(&mut frame, -432, x2);
                            join_b = true;
                        }
                    } else {
                        let c4: u32 = lf_checker_rt::callee_thiscall!(I_CHECK, u32, this, esi2);
                        if (c4 as u8) != 0 {
                            let mut x = fmul(C4, gain);
                            x = fadd(x, CN3);
                            wff(&mut frame, -424, x);
                        } else {
                            eax_v = rf(&frame, -400);
                            flav = (eax_v & 0xff) as u8;
                            if flav == 2 {
                                let mut x = fmul(C4, gain);
                                wb(&mut frame, -392, 1);
                                x = fadd(x, CN4);
                                wff(&mut frame, -424, x);
                                join_a = true;
                            } else if flav == 6 {
                                let mut x = fmul(C4, gain);
                                wb(&mut frame, -392, 1);
                                wb(&mut frame, -416, 1);
                                x = fadd(x, CN4);
                                wff(&mut frame, -424, x);
                                join_a = true;
                            } else if flav == 4 {
                                wff(&mut frame, -424, CN7);
                                wff(&mut frame, -432, C04);
                                join_b = true;
                            } else if flav == 5 {
                                wff(&mut frame, -424, CN2);
                                join_b = true;
                            } else {
                                let mut x2 = fsub(UNIT, gain);
                                let mut x1 = fmul(C4, gain);
                                let mut x0 = fmul(C6, gain);
                                x2 = fmul(x2, C3);
                                x1 = fadd(x1, CN7);
                                x2 = fadd(x2, x0);
                                wff(&mut frame, -424, x1);
                                wff(&mut frame, -432, x2);
                                join_b = true;
                            }
                        }
                    }
                    let _ = checked;
                }
            }
            if join_a {
                wff(&mut frame, -432, C12);
            }
            let _ = join_b;
        } else {
            wff(&mut frame, -424, CN4);
            wff(&mut frame, -432, C12);
        }
        let b266 = rb(&frame, -266);
        wb(&mut frame, -266, b266 & 0xdf);
        esi_v = g32!(VOICE_A);
        wf(&mut frame, -436, esi_v);
        let e8 = rd32(this.wrapping_add(8));
        if e8 != 0 {
            let c: u32 = lf_checker_rt::callee_thiscall!(I_CHECK, u32, this, e8);
            if (c as u8) == 0 {
                if rb(&frame, -400) != 1 {
                    esi_v = g32!(VOICE_B);
                }
                wf(&mut frame, -436, esi_v);
            }
        }
        let ap: u32 = lf_checker_rt::callee_thiscall!(I_APPLY, u32, this, rff(&frame, -432).to_bits());
        let mut vx = rff(&frame, -424);
        if !(0.0f32 > vx) {
            vx = 0.0;
            wff(&mut frame, -424, vx);
        }
        wf(&mut frame, -308, ap);
        let fa2 = fp(&mut frame, -336);
        lf_checker_rt::callee_thiscall!(I_SUBMIT, u32, this, rf(&frame, -436), this.wrapping_add(SLOT0), fa2, 0xffffffffu32, 0u32, 0u32);
        let handle = rd32(this.wrapping_add(SLOT0));
        if handle != 0 {
            let idx = rd8(handle.wrapping_add(4));
            wf(&mut frame, -396, idx as u32);
            let entry = if idx != 0xff {
                let row = rd8(handle.wrapping_add(0x40));
                let stride = g32!(G1);
                let table = g32!(G2);
                let base = rd32(table.wrapping_add((row as u32).wrapping_mul(ROW_STRIDE)).wrapping_add(ROW_BIAS));
                stride.wrapping_mul(idx as u32).wrapping_add(base)
            } else {
                0
            };
            lf_checker_rt::callee_thiscall!(I_ROUTE2, u32, entry, rff(&frame, -424).to_bits());
        }
        let h = rd32(this.wrapping_add(SLOT0));
        if h == 0 {
            lf_checker_rt::callee_cdecl!(I_COOKIE, u32,);
            return 0;
        }
        if rd8(h.wrapping_add(0x3b)) != 4 {
            let rr: u32 = lf_checker_rt::callee_thiscall!(I_RELEASE, u32, h, 0u32);
            lf_checker_rt::callee_cdecl!(I_COOKIE, u32,);
            return rr;
        }
        let cm0: u32 = lf_checker_rt::callee_thiscall!(I_COMMIT_A, u32, h, rf(&frame, -380), rf(&frame, -420), a2s);
        if (cm0 as u8) == 0 {
            let rr: u32 = lf_checker_rt::callee_thiscall!(I_RELEASE, u32, h, 0u32);
            lf_checker_rt::callee_cdecl!(I_COOKIE, u32,);
            return rr;
        }
        wr8(this.wrapping_add(BUSY0), rd8(this.wrapping_add(BUSY0)) | BUSY_BIT);
        wr8(this.wrapping_add(BUSY1), rd8(this.wrapping_add(BUSY1)) | BUSY_BIT);
        let pa = fp(&mut frame, -352);
        let pb = fp(&mut frame, -432);
        let pc = fp(&mut frame, -396);
        wf(&mut frame, -396, 0);
        wf(&mut frame, -432, POS_SEED);
        lf_checker_rt::callee_thiscall!(I_POS, u32, this, pc, pb, pa);
        lf_checker_rt::callee_thiscall!(I_SETW, u32, h, rff(&frame, -432).to_bits());
        let mut z = f32::from_bits(rd32(this.wrapping_add(BASE_VOL)));
        z = fadd(z, rff(&frame, -396));
        lf_checker_rt::callee_thiscall!(I_SETX, u32, h, z.to_bits());
        let sa = fp(&mut frame, -352);
        lf_checker_rt::callee_thiscall!(I_STORE, u32, h, sa);
        if rf(&frame, -436) == g32!(VOICE_B) {
            let ba = fp(&mut frame, -436);
            wf(&mut frame, -436, 0);
            lf_checker_rt::callee_thiscall!(I_BEGIN, u32, this, rf(&frame, -392), this.wrapping_add(AUX), ba, rf(&frame, -416));
            let w296 = rf(&frame, -436);
            wf(&mut frame, -296, w296);
            let fa4 = fp(&mut frame, -336);
            lf_checker_rt::callee_thiscall!(I_SUBMIT2, u32, this, g32!(SUB_FMT), this.wrapping_add(SLOT1), fa4, 0xffffffffu32, 0u32, 0u32);
            let h2 = rd32(this.wrapping_add(SLOT1));
            if h2 != 0 {
                lf_checker_rt::callee_thiscall!(I_SETVOL, u32, h2, rff(&frame, -424).to_bits());
                let cm1: u32 = lf_checker_rt::callee_thiscall!(I_COMMIT_B, u32, h2, rf(&frame, -380), rf(&frame, -420), a2s);
                if (cm1 as u8) == 0 {
                    lf_checker_rt::callee_thiscall!(I_RELEASE, u32, h2, 0u32);
                } else {
                    lf_checker_rt::callee_thiscall!(I_SETW, u32, h2, rff(&frame, -432).to_bits());
                    let mut z2 = f32::from_bits(rd32(this.wrapping_add(BASE_VOL)));
                    z2 = fadd(z2, rff(&frame, -396));
                    z2 = fadd(z2, f32::from_bits(rd32(this.wrapping_add(AUX))));
                    lf_checker_rt::callee_thiscall!(I_SETX, u32, h2, z2.to_bits());
                    let ear: u32 = lf_checker_rt::callee_thiscall!(I_EAR, u32, range_obj, 0u32);
                    let e0 = f32::from_bits(rd32(ear));
                    let e1 = f32::from_bits(rd32(ear.wrapping_add(4)));
                    let e2 = f32::from_bits(rd32(ear.wrapping_add(8)));
                    let mut p = fsub(rff(&frame, -352), e0);
                    wff(&mut frame, -416, e0);
                    wff(&mut frame, -368, p);
                    p = fsub(rff(&frame, -348), e1);
                    wff(&mut frame, -396, e1);
                    wff(&mut frame, -392, e2);
                    wff(&mut frame, -364, p);
                    p = fsub(rff(&frame, -344), e2);
                    wff(&mut frame, -360, p);
                    let na = fp(&mut frame, -368);
                    lf_checker_rt::callee_thiscall!(I_PAN, u32, na, PITCH.to_bits(), 0x7au32);
                    let mut q = rff(&frame, -368);
                    q = fadd(q, rff(&frame, -416));
                    wff(&mut frame, -368, q);
                    q = rff(&frame, -364);
                    q = fadd(q, rff(&frame, -396));
                    wff(&mut frame, -364, q);
                    q = rff(&frame, -360);
                    q = fadd(q, rff(&frame, -392));
                    wff(&mut frame, -360, q);
                    let ra = fp(&mut frame, -368);
                    lf_checker_rt::callee_thiscall!(I_STORE, u32, h2, ra);
                }
            }
        }
        let hv = rd32(this.wrapping_add(HANDLE));
        wr32(this.wrapping_add(CHAN), 0);
        if hv != 0 {
            lf_checker_rt::callee_cdecl!(I_CLOSE, u32, hv);
            wr32(this.wrapping_add(HANDLE), 0);
        }
        let mut lim = rf(&frame, -372);
        wf(&mut frame, -436, lim);
        if lim == 0 {
            lim = lf_checker_rt::callee_cdecl!(I_QUERY, u32, g32!(QUERY_ARG));
            wf(&mut frame, -436, lim);
        }
        if (a5 & 0xff) != 0 {
            lf_checker_rt::callee_thiscall!(I_ROUTE, u32, h, lim, 1u32, 0x1388u32);
            let c64 = rd32(this.wrapping_add(SLOT1));
            if c64 != 0 {
                lf_checker_rt::callee_thiscall!(I_ROUTE, u32, c64, rf(&frame, -436), 1u32, 0x1388u32);
            }
        } else {
            lf_checker_rt::callee_thiscall!(I_DIAL, u32, h, lim, 0u32);
            lf_checker_rt::callee_thiscall!(I_PING, u32, h);
            let c64 = rd32(this.wrapping_add(SLOT1));
            if c64 != 0 {
                lf_checker_rt::callee_thiscall!(I_DIAL, u32, c64, rf(&frame, -436), 0u32);
                lf_checker_rt::callee_thiscall!(I_PING, u32, c64);
            }
        }
        wr32(this.wrapping_add(NAME_LEN_OFF), a2s);
        wf(&mut frame, -416, 0);
        if h != 0 {
            let r: u32 = lf_checker_rt::callee_thiscall!(I_SCAN, u32, h);
            let r2: u32 = lf_checker_rt::callee_cdecl!(I_LOOKUP, u32, rf(&frame, -436), r);
            if r2 != 0 {
                let w = rd16(r2.wrapping_add(0x18)) as u32;
                let v = rd32(r2.wrapping_add(0x10));
                let r3: u32 = lf_checker_rt::callee_cdecl!(I_HASH, u32, v, w);
                wf(&mut frame, -416, r3);
            }
        }
        wr32(this.wrapping_add(0x5c), g32!(CFG_A));
        wr32(this.wrapping_add(0x10), rf(&frame, -376));
        lf_checker_rt::callee_cdecl!(I_EMIT, u32, this.wrapping_add(0x14), rf(&frame, -420), 0x3fu32);
        esi_v = rf(&frame, -372);
        let s1: u32 = lf_checker_rt::callee_cdecl!(I_SEND_A, u32, esi_v);
        wr8(this.wrapping_add(0x54), s1 as u8);
        wr8(this.wrapping_add(0x55), rf(&frame, -400) as u8);
        let ev = rf(&frame, -416);
        let s2: u32 = lf_checker_rt::callee_cdecl!(I_SEND_B, u32, esi_v, a4s, ev);
        wr32(this.wrapping_add(0x58), ev);
        esi_v = a2s;
        let b0: u32 = lf_checker_rt::callee_cdecl!(I_BROADCAST, u32, rd32(this.wrapping_add(8)), rf(&frame, -376), rf(&frame, -420), esi_v, s2, a4s, ev, rd32(this.wrapping_add(0x5c)));
        let r1: u32 = lf_checker_rt::callee_cdecl!(I_RENDER_A, u32, rf(&frame, -420), 0u32);
        wr32(this.wrapping_add(0x68), r1);
        let r2: u32 = lf_checker_rt::callee_cdecl!(I_RENDER_A, u32, rf(&frame, -380), 0u32);
        lf_checker_rt::callee_thiscall!(I_VOICE4, u32, voice_obj, r2, r1, esi_v, g32!(CFG_B));
        let sp = rf(&frame, -420);
        let mut len = 0u32;
        while rd8(sp.wrapping_add(len)) != 0 {
            len += 1;
        }
        let buf: u32 = lf_checker_rt::callee_cdecl!(I_ALLOC, u32, len.wrapping_add(4));
        lf_checker_rt::callee_cdecl!(I_FMT4H, u32, buf, fmt_c, rf(&frame, -420), a2s);
        let r3: u32 = lf_checker_rt::callee_cdecl!(I_RENDER_A, u32, buf, 0u32);
        wr32(this.wrapping_add(0x6c), r3);
        let rr: u32 = lf_checker_rt::callee_cdecl!(I_CLOSE, u32, buf);
        lf_checker_rt::callee_cdecl!(I_COOKIE, u32,);
        let _ = b0;
        rr
    }
});
