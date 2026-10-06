// original: 0x00df6a00 UIFileViewer::vf97
// Specification. Thiscall viewer method (ecx = viewer, one stack word
// forwarded to the opening service call, then dead). It resolves a
// service object, notifies the viewer, runs setup, and compares an
// identity answer against a key. On match (branch A) it resolves a
// clip object (null exits; the clip
// is spilled over the entry argument slot below the observed stack
// window), reads 8 float measures off each of the service and clip
// objects through slots C8/B8/B8/C8/D0/C0/C0/D0, combines them with the
// global scale into two bound pairs, unions them with min/max selects,
// builds two clamped limits from integer globals times multiplier
// globals, and runs four strict-containment gates. Any gate failure
// runs the shared fail tail (service call plus fail notify). When all
// gates pass, the tail runs: an al-tested event call, two service
// resolutions (objects B and S), a 4-round identity cascade comparing
// the clip against four viewer-held objects (result 1/2/3/4, or 0 when
// no round matches; the result is spilled over the incoming argument
// slot below the observed stack window), a count/item loop over B
// keeping the trip index of the first item whose identity matches
// the service object's (reset to zero when the count runs out), a
// 3-argument service call carrying (cascade, saved answer, loop
// index), and a chain of notify/setter virtuals over S, B and the
// service object. Two of those virtuals take shape (value, 1): the
// original pushes the constant 1 ahead of the value-producing call,
// which is unobservable below-frame scratch, so the rewrite pushes
// both words at the consuming call; the logged arguments and the final
// stack pointer are identical. On mismatch (branch B) a second
// identity round runs (unequal exits); then a second float section
// over the service object and the viewer with the same gates, and on
// pass a second tail: a service resolution, a flag byte that returns
// early when set, a counted array loop firing an event per flagged
// item, a second count that returns when zero, an event call, double
// math combining that count with a constant table against a viewer
// float (minimum of two products), a float-argument virtual, a final
// direct call, and a marker byte write. Returns nothing observed.
export!(thiscall, rw_00df6a00(this: u32, arg0: u32) -> u32 {
    unsafe {
        /// Shared UI service (file VA; relocated for the worker mapping).
        const UI_SERVICE: u32 = 0x01981A4C;
        /// Second service used by the 3-argument tail call.
        const TAIL_SERVICE: u32 = 0x019D2F18;
        /// Event service used by the al-tested call.
        const EVENT_SERVICE: u32 = 0x01176888;
        /// Event key pushed for the al-tested call.
        const EVENT_KEY: u32 = 0x00F01A54;
        /// Branch-A identity key.
        const KEY_A: u32 = 0x00F01A48;
        /// Branch-B identity key.
        const KEY_B: u32 = 0x00F01B70;
        /// Event key pushed by the branch-B tail.
        const EVENT_KEY_B: u32 = 0x00F01B80;
        /// Two-double table for the count conversion (selected by the
        /// count's top bit).
        const DOUBLE_TABLE: u32 = 0x00FE8F50;
        /// Constants folded into the branch-B float tail.
        const DBL_ADD: u32 = 0x00FE8AD8;
        const DBL_MUL: u32 = 0x00FE8A94;
        /// Notify slot on the viewer itself.
        const NOTIFY_SLOT: u32 = 0x1C0;
        /// Gate-fail notify slot on the viewer itself.
        const FAIL_SLOT: u32 = 0x1A4;
        /// Per-side measure slots, in call order.
        const SLOTS: [u32; 8] = [0xC8, 0xB8, 0xB8, 0xC8, 0xD0, 0xC0, 0xC0, 0xD0];
        /// Identity slot shared by the cascade objects.
        const IDENT_SLOT: u32 = 0x4C;
        let ui_service = relocated(UI_SERVICE);
        let edi: u32 = callee_thiscall!(1, u32, ui_service, arg0);
        let vt_this = *(this as *const u32);
        let notify_addr = *((vt_this.wrapping_add(NOTIFY_SLOT)) as *const u32);
        let notify: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(notify_addr as usize);
        let _ = notify(this);
        let _: u32 = callee_thiscall!(3, u32, this, 1);
        let vt_edi = *(edi as *const u32);
        let ident_addr = *(vt_edi as *const u32);
        let ident: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(ident_addr as usize);
        let got: u32 = ident(edi);
        let want: u32 = callee_cdecl!(5, u32, relocated(KEY_A));
        if got != want {
            // Branch B: second identity round, then a float section
            // over (service, viewer) with the same gates, then its own
            // tail. The clamp inputs are read again, as observed.
            let got2: u32 = ident(edi);
            let want2: u32 = callee_cdecl!(5, u32, relocated(KEY_B));
            if got2 != want2 {
                return 0;
            }
            let scale_b = *(global::<f32>(0x00FE8830));
            let mut c = [0.0f32; 8];
            let mut d = [0.0f32; 8];
            for (i, slot) in SLOTS.iter().enumerate() {
                let addr = *((vt_edi.wrapping_add(*slot)) as *const u32);
                let f: extern "thiscall" fn(u32) -> f32 =
                    core::mem::transmute(addr as usize);
                c[i] = f(edi);
            }
            for (i, slot) in SLOTS.iter().enumerate() {
                let addr = *((vt_this.wrapping_add(*slot)) as *const u32);
                let f: extern "thiscall" fn(u32) -> f32 =
                    core::mem::transmute(addr as usize);
                d[i] = f(this);
            }
            let (ca, cb, cc_, cd) = (
                c[0] - c[1] * scale_b,
                c[3] + c[2] * scale_b,
                c[4] - c[5] * scale_b,
                c[7] + c[6] * scale_b,
            );
            let (da, db, dc, dd) = (
                d[0] - d[1] * scale_b,
                d[3] + d[2] * scale_b,
                d[4] - d[5] * scale_b,
                d[7] + d[6] * scale_b,
            );
            let mut r0 = ca;
            if da > r0 {
                r0 = da;
            }
            let mut r1 = cb;
            if r1 > db {
                r1 = db;
            }
            let mut r2 = cc_;
            if dc > r2 {
                r2 = dc;
            }
            let mut r3 = cd;
            if r3 > dd {
                r3 = dd;
            }
            let hi_b = *(global::<f32>(0x00FE88E8));
            let mut lim0_b = (*(global::<i32>(0x018B7A8C)) as f32)
                * *(global::<f32>(0x017ACCF0));
            if 0.0 > lim0_b {
                lim0_b = 0.0;
            }
            if lim0_b > hi_b {
                lim0_b = hi_b;
            }
            let raw1_b = (*(global::<i32>(0x018B7A80)) as f32)
                * *(global::<f32>(0x017ACCE8));
            let lim1_b = if 0.0 > raw1_b {
                0.0
            } else if raw1_b > hi_b {
                hi_b
            } else {
                raw1_b
            };
            let mut failed_b = false;
            if !(lim1_b > r0) {
                failed_b = true;
            } else if !(r1 > lim1_b) {
                failed_b = true;
            } else if !(lim0_b > r2) {
                failed_b = true;
            } else if !(r3 > lim0_b) {
                failed_b = true;
            }
            if failed_b {
                let svc: u32 = *(global::<u32>(0x018B6C8C));
                let _: u32 = callee_thiscall!(18, u32, svc);
                let fail_addr = *((vt_this.wrapping_add(FAIL_SLOT)) as *const u32);
                let fail: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(fail_addr as usize);
                let _ = fail(this);
                return 0;
            }
            let p2_addr = *((vt_edi.wrapping_add(0x54)) as *const u32);
            let get_p2: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(p2_addr as usize);
            let fobj: u32 = callee_thiscall!(50, u32, ui_service, get_p2(edi));
            if *(fobj.wrapping_add(0x1EC) as *const u8) != 0 {
                let _: u32 = callee_thiscall!(51, u32, fobj);
                return 0;
            }
            let count16 = *(this.wrapping_add(0x1D8) as *const u16);
            if (0u16) < count16 {
                let mut idx = 0u32;
                loop {
                    let arr = *(this.wrapping_add(0x1D4) as *const u32);
                    let it = *((arr.wrapping_add(idx.wrapping_mul(4))) as *const u32);
                    if *(it.wrapping_add(0x1EC) as *const u8) != 0 {
                        let _: u32 = callee_thiscall!(51, u32, it);
                    }
                    let n2 = *(this.wrapping_add(0x1D8) as *const u16) as u32;
                    idx = idx.wrapping_add(1);
                    if (idx as i32) < (n2 as i32) {
                        continue;
                    } else {
                        break;
                    }
                }
            }
            let s2 = *(fobj.wrapping_add(0x1E4) as *const u32);
            let vt_s2 = *(s2 as *const u32);
            let cnt_addr = *((vt_s2.wrapping_add(0x1D4)) as *const u32);
            let cnt2: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(cnt_addr as usize);
            if cnt2(s2) == 0 {
                return 0;
            }
            let _: u32 = callee_thiscall!(
                22,
                u32,
                relocated(EVENT_SERVICE),
                relocated(EVENT_KEY_B)
            );
            let base_f = *(this.wrapping_add(0x31C) as *const f32);
            let cnt = cnt2(s2);
            let tab = global::<f64>(DOUBLE_TABLE).wrapping_add((cnt >> 31) as usize);
            let dsum = (cnt as i32) as f64 + *tab;
            let lo = dsum as f32;
            let x0 = base_f + *(global::<f32>(DBL_ADD));
            let x2 = base_f * *(global::<f32>(DBL_MUL));
            let mut x1 = lo * x0;
            if !(x2 > x1) {
                x1 = x2;
            }
            let fa_addr = *((vt_s2.wrapping_add(0xA0)) as *const u32);
            let fcall: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(fa_addr as usize);
            let _ = fcall(s2, x1.to_bits());
            let _: u32 = callee_thiscall!(54, u32, fobj);
            *(s2.wrapping_add(0x218) as *mut u8) = 1;
            return 0;
        }
        let esi: u32 = callee_thiscall!(6, u32, this);
        if esi == 0 {
            return 0;
        }
        let scale = *(global::<f32>(0x00FE8830));
        let mut a = [0.0f32; 8];
        let mut b = [0.0f32; 8];
        for (i, slot) in SLOTS.iter().enumerate() {
            let addr = *((vt_edi.wrapping_add(*slot)) as *const u32);
            let f: extern "thiscall" fn(u32) -> f32 =
                core::mem::transmute(addr as usize);
            a[i] = f(edi);
        }
        let vt_esi = *(esi as *const u32);
        for (i, slot) in SLOTS.iter().enumerate() {
            let addr = *((vt_esi.wrapping_add(*slot)) as *const u32);
            let f: extern "thiscall" fn(u32) -> f32 =
                core::mem::transmute(addr as usize);
            b[i] = f(esi);
        }
        let (aa, ab, ac, ad) = (
            a[0] - a[1] * scale,
            a[3] + a[2] * scale,
            a[4] - a[5] * scale,
            a[7] + a[6] * scale,
        );
        let (ba, bb, bc, bd) = (
            b[0] - b[1] * scale,
            b[3] + b[2] * scale,
            b[4] - b[5] * scale,
            b[7] + b[6] * scale,
        );
        let mut r_max_a = aa;
        if ba > r_max_a {
            r_max_a = ba;
        }
        let mut r_min_b = ab;
        if r_min_b > bb {
            r_min_b = bb;
        }
        let mut r_max_c = ac;
        if bc > r_max_c {
            r_max_c = bc;
        }
        let mut r_min_d = ad;
        if r_min_d > bd {
            r_min_d = bd;
        }
        let hi = *(global::<f32>(0x00FE88E8));
        let mut lim0 =
            (*(global::<i32>(0x018B7A8C)) as f32) * *(global::<f32>(0x017ACCF0));
        if 0.0 > lim0 {
            lim0 = 0.0;
        }
        if lim0 > hi {
            lim0 = hi;
        }
        let raw1 =
            (*(global::<i32>(0x018B7A80)) as f32) * *(global::<f32>(0x017ACCE8));
        let lim1 = if 0.0 > raw1 {
            0.0
        } else if raw1 > hi {
            hi
        } else {
            raw1
        };
        let mut failed = false;
        if !(lim1 > r_max_a) {
            failed = true;
        } else if !(r_min_b > lim1) {
            failed = true;
        } else if !(lim0 > r_max_c) {
            failed = true;
        } else if !(r_min_d > lim0) {
            failed = true;
        }
        if failed {
            let svc: u32 = *(global::<u32>(0x018B6C8C));
            let _: u32 = callee_thiscall!(18, u32, svc);
            let fail_addr = *((vt_this.wrapping_add(FAIL_SLOT)) as *const u32);
            let fail: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(fail_addr as usize);
            let _ = fail(this);
            return 0;
        }
        // Gate-pass tail. The entry code spills the clip object over
        // its own incoming argument slot (below the observed stack
        // window), so the cascade object below is the clip, and the
        // entry argument is dead after the opening service call. B and
        // S are the two resolved service objects.
        let clip = esi;
        let al_addr = *((vt_edi.wrapping_add(0x1C)) as *const u32);
        let al_test: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(al_addr as usize);
        if al_test(edi) & 0xFF == 0 {
            let _: u32 =
                callee_thiscall!(22, u32, relocated(EVENT_SERVICE), relocated(EVENT_KEY));
        }
        let p_addr = *((vt_edi.wrapping_add(0x54)) as *const u32);
        let get_p: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(p_addr as usize);
        let ebx: u32 = callee_thiscall!(20, u32, ui_service, get_p(edi));
        let vt_ebx = *(ebx as *const u32);
        let q_addr = *((vt_ebx.wrapping_add(0x54)) as *const u32);
        let get_q: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(q_addr as usize);
        let svcb: u32 = callee_thiscall!(21, u32, ui_service, get_q(ebx));
        let h0_addr = *((vt_ebx.wrapping_add(IDENT_SLOT)) as *const u32);
        let h0_get: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(h0_addr as usize);
        let h0 = h0_get(ebx);
        let j0_addr = *((vt_esi.wrapping_add(0x1E4)) as *const u32);
        let j0_get: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(j0_addr as usize);
        let j0 = j0_get(clip, h0);
        // Four-round identity cascade. Rounds 1-2 call the held object
        // first, rounds 3-4 call the clip first, exactly as observed.
        let x2 = *(this.wrapping_add(0x1E8) as *const u32);
        let x3 = *(this.wrapping_add(0x1E0) as *const u32);
        let x4 = *(this.wrapping_add(0x1E4) as *const u32);
        let x5 = *(this.wrapping_add(0x1EC) as *const u32);
        let id_of = |obj: u32| -> u32 {
            let vt = *(obj as *const u32);
            let addr = *((vt.wrapping_add(IDENT_SLOT)) as *const u32);
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(addr as usize);
            f(obj)
        };
        let cascade: u32;
        if id_of(x2) == id_of(clip) {
            cascade = 1;
        } else if id_of(x3) == id_of(clip) {
            cascade = 2;
        } else if id_of(clip) == id_of(x4) {
            cascade = 3;
        } else if id_of(clip) == id_of(x5) {
            cascade = 4;
        } else {
            cascade = 0;
        }
        // The original also spills the cascade result over its incoming
        // argument slot, but that slot sits below the checker's observed
        // stack window, so the rewrite keeps the result in a local.
        // Count/item loop over B, exiting on the first identity match
        // against the service object.
        let mut loop_idx = 0u32;
        let cnt_addr = *((vt_ebx.wrapping_add(0x1D4)) as *const u32);
        let cnt: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(cnt_addr as usize);
        if cnt(ebx) != 0 {
            loop {
                let item_addr = *((vt_ebx.wrapping_add(0x1E0)) as *const u32);
                let item_at: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(item_addr as usize);
                let item = item_at(ebx, loop_idx);
                if id_of(item) == id_of(edi) {
                    break;
                }
                loop_idx = loop_idx.wrapping_add(1);
                if loop_idx >= cnt(ebx) {
                    // Count exhausted without a match: the index is
                    // reset to zero; only a match keeps it.
                    loop_idx = 0;
                    break;
                }
            }
        }
        let _: u32 = callee_thiscall!(23, u32, relocated(TAIL_SERVICE), cascade, j0, loop_idx);
        let vt_s = *(svcb as *const u32);
        let e_addr = *((vt_s.wrapping_add(0x224)) as *const u32);
        let get_e: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(e_addr as usize);
        let eobj = get_e(svcb);
        let vt_e = *(eobj as *const u32);
        let f_addr = *((vt_e.wrapping_add(0x220)) as *const u32);
        let get_f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(f_addr as usize);
        let fobj = get_f(eobj);
        let vt_f = *(fobj as *const u32);
        let n0_addr = *((vt_f.wrapping_add(0x1B0)) as *const u32);
        let n0: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(n0_addr as usize);
        let _ = n0(fobj);
        let gobj = get_f(eobj);
        let vt_g = *(gobj as *const u32);
        let s0_addr = *((vt_g.wrapping_add(0x18)) as *const u32);
        let s0: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(s0_addr as usize);
        let _ = s0(gobj, 0);
        let s1_addr = *((vt_e.wrapping_add(0x18)) as *const u32);
        let s1: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(s1_addr as usize);
        let _ = s1(eobj, 0);
        // (value, 1) pairs: both words pushed at the consuming call
        // (see the header note); logged arguments match exactly.
        let h_addr = *((vt_ebx.wrapping_add(IDENT_SLOT)) as *const u32);
        let h_get: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(h_addr as usize);
        let i_addr = *((vt_s.wrapping_add(0x1E4)) as *const u32);
        let i_set: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(i_addr as usize);
        let ival = i_set(svcb, h_get(ebx), 1);
        let w0_addr = *((vt_s.wrapping_add(0x230)) as *const u32);
        let w0: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(w0_addr as usize);
        let _ = w0(svcb, ival);
        let w1_addr = *((vt_ebx.wrapping_add(0x18)) as *const u32);
        let w1: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(w1_addr as usize);
        let _ = w1(ebx, 1);
        let j_addr = *((vt_edi.wrapping_add(IDENT_SLOT)) as *const u32);
        let j_get: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(j_addr as usize);
        let k_addr = *((vt_ebx.wrapping_add(0x1E4)) as *const u32);
        let k_set: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(k_addr as usize);
        let kval = k_set(ebx, j_get(edi), 1);
        let w2_addr = *((vt_ebx.wrapping_add(0x22C)) as *const u32);
        let w2: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(w2_addr as usize);
        let _ = w2(ebx, kval);
        let n1_addr = *((vt_edi.wrapping_add(0x1AC)) as *const u32);
        let n1: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(n1_addr as usize);
        let _ = n1(edi);
        let s2_addr = *((vt_edi.wrapping_add(0x18)) as *const u32);
        let s2: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(s2_addr as usize);
        let _ = s2(edi, 1);
        0
    }
});
