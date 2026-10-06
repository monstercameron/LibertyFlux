// original: 0x00C67BE0 drain_scored_stream_candidates (proposed)

/// Collect stream candidates, score each by proximity to a point, then drain
/// the best ones through notify/teardown calls.
///
/// `this` is a controller object (two loop-gate dwords at `+0x708`/`+0x70c`,
/// table bases at `+0x0`/`+0x100`/`+0x300`/`+0x404`/`+0x504`/`+0x604`);
/// `arg0` points at two floats (the reference x/y). The function runs two
/// gather phases (A over pools A/B, C over pool C) and two drain phases
/// (B, D):
///
/// Gather: reset a 64-slot id array, bind the tables, count the entries.
/// For each entry: fetch its id, store it with a zero score, ask the field
/// helper; entries whose answer has bits 0x44 set are skipped. Otherwise
/// scan every object of each pool: skip flagged (`0x80`) or null objects and
/// (pools A/B) objects whose signed key word at `+0x2e` differs from the id;
/// survivors must also pass a check call (vtable slot `+0x34` for A/C, a
/// helper plus a zero byte for B, two extra helpers for C). Each survivor
/// adds `K / (K + dist)` to the entry's score, where `dist` is the planar
/// distance between the reference point and the object's position
/// (`[obj+0x20]` holder, floats at `+0x30`/`+0x34`) and `K` is a global
/// float. An entry is kept only if every tested object passed.
///
/// Drain: while the gate word exceeds its threshold, pick the kept entry
/// with the lowest score (ties keep the earliest; marked `-1` entries are
/// skipped), mark it `-1`, and walk the pools for objects with that key:
/// phase B calls the notify helper (or a vtable slot at `+0x148` when the
/// object carries the flag byte and a holder), phase D calls the teardown
/// helper. All loop bounds and gate comparisons are SIGNED; the score
/// minimum uses an ordered float greater-than (NaN never wins).
///
/// Original: 0x00C67BE0 (thiscall, one stack word; returns `this`).
lf_checker_rt::export!(thiscall, rw_00C67BE0(this: u32, arg0: u32) -> u32 {
    unsafe {
        const POOL_A: u32 = 0x018B_6F10;
        const POOL_B: u32 = 0x018B_6F1C;
        const POOL_C: u32 = 0x012E_22A4;
        const WANTED_ID: u32 = 0x012F_A014;
        const FIELD_FLAGS: u32 = 0x012B_4138;
        const FALLBACK_K: u32 = 0x00FE_8B08;
        const INIT_BEST: u32 = 0x00E8_33D8;
        const GATE_B: i32 = 0x262_5A00;
        const GATE_D: i32 = 0x18C_BA80;
        const HDR_BASE: u32 = 0;
        const HDR_FLAGS: u32 = 4;
        const HDR_COUNT: u32 = 8;
        const HDR_STRIDE: u32 = 12;
        const OBJ_VTABLE: u32 = 0;
        const OBJ_POS: u32 = 0x20;
        const OBJ_KEY: u32 = 0x2E;
        const SLOT_VCHECK: u32 = 0x34;
        const SLOT_NOTIFY: u32 = 0x148;
        const SKIP_BIT: u8 = 0x80;
        const GATE_BITS: u8 = 0x44;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16s(a: u32) -> i32 {
            unsafe { (a as *const i16).read_unaligned() as i32 }
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
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fdiv(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        /// Planar proximity increment `K / (K + dist)`; `x_first` selects
        /// the exact add order of the original site.
        unsafe fn proximity_inc(px: f32, py: f32, qx: f32, qy: f32, x_first: bool) -> f32 {
            unsafe {
                let dx = fsub(px, qx);
                let dy = fsub(py, qy);
                let dx2 = fmul(dx, dx);
                let dy2 = fmul(dy, dy);
                let s = if x_first { fadd(dx2, dy2) } else { fadd(dy2, dx2) };
                let dist = s.sqrt();
                let k = (lf_checker_rt::global::<f32>(FALLBACK_K) as *const f32).read();
                let t = fadd(dist, k);
                fdiv(k, t)
            }
        }
        unsafe fn vcall0(slot_addr: u32, obj: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(slot_addr as usize);
                f(obj)
            }
        }
        #[inline(always)]
        unsafe fn g32(a: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(a) as *const u32).read() }
        }

        let slots = [0u32; 64];
        let mut ids = [0u32; 64];
        let mut acc = [0f32; 64];
        let slots_p = slots.as_ptr() as u32;
        let qx = rdf(arg0);
        let qy = rdf(arg0 + 4);

        // ---- Phase A: gather over pools A and B ----
        lf_checker_rt::callee_thiscall!(1, u32, slots_p);
        lf_checker_rt::callee_thiscall!(2, u32, slots_p, this + 0x404, this + 0x504, this + 0x604, 0);
        lf_checker_rt::callee_thiscall!(3, u32, slots_p, g32(WANTED_ID));
        let count = lf_checker_rt::callee_thiscall!(4, u32, slots_p) as i32;
        let mut kept = 0usize;
        if count > 0 {
            let mut i = 0i32;
            while i < count {
                let id = lf_checker_rt::callee_thiscall!(5, u32, slots_p, i as u32);
                ids[kept] = id;
                acc[kept] = 0.0;
                let g = lf_checker_rt::callee_cdecl!(6, u32, id, g32(FIELD_FLAGS));
                if (g as u8) & GATE_BITS != 0 {
                    i += 1;
                    continue;
                }
                let mut dl = 1u8;
                // Pool A scan.
                let ha = g32(POOL_A);
                let na = rd32(ha + HDR_COUNT) as i32;
                if na > 0 {
                    let mut j = 0i32;
                    while j < na {
                        let fl = rd32(ha + HDR_FLAGS);
                        if rd8(fl + j as u32) & SKIP_BIT == 0 {
                            let stride = rd32(ha + HDR_STRIDE);
                            let obj = rd32(ha + HDR_BASE).wrapping_add(stride.wrapping_mul(j as u32));
                            if obj != 0 && rd16s(obj + OBJ_KEY) == id as i32 {
                                let vt = rd32(obj + OBJ_VTABLE);
                                let r = vcall0(rd32(vt + SLOT_VCHECK), obj);
                                if (r as u8) == 0 {
                                    dl = 0;
                                } else {
                                    let pp = rd32(obj + OBJ_POS);
                                    let inc = proximity_inc(rdf(pp + 0x30), rdf(pp + 0x34), qx, qy, true);
                                    acc[kept] = fadd(inc, acc[kept]);
                                }
                            }
                        }
                        j += 1;
                    }
                }
                // Pool B scan.
                let hb = g32(POOL_B);
                let nb = rd32(hb + HDR_COUNT) as i32;
                if nb > 0 {
                    let mut j = 0i32;
                    while j < nb {
                        let fl = rd32(hb + HDR_FLAGS);
                        if rd8(fl + j as u32) & SKIP_BIT == 0 {
                            let stride = rd32(hb + HDR_STRIDE);
                            let obj = rd32(hb + HDR_BASE).wrapping_add(stride.wrapping_mul(j as u32));
                            if obj != 0 && rd16s(obj + OBJ_KEY) == id as i32 {
                                let r = lf_checker_rt::callee_thiscall!(7, u32, obj, 1, 1);
                                if (r as u8) == 0 || rd8(obj + 0x219) != 0 {
                                    dl = 0;
                                } else {
                                    let pp = rd32(obj + OBJ_POS);
                                    let inc = proximity_inc(rdf(pp + 0x30), rdf(pp + 0x34), qx, qy, false);
                                    acc[kept] = fadd(inc, acc[kept]);
                                }
                            }
                        }
                        j += 1;
                    }
                }
                if dl != 0 {
                    kept += 1;
                }
                i += 1;
            }
        }

        // ---- Phase B: drain best-first through notify ----
        if (rd32(this + 0x708) as i32) > GATE_B {
            loop {
                let mut best = -1i32;
                let mut cur = (lf_checker_rt::global::<f32>(INIT_BEST) as *const f32).read();
                let mut k = 0usize;
                while k < kept {
                    if (ids[k] as i32) >= 0 {
                        let a = acc[k];
                        if cur > a {
                            cur = a;
                            best = k as i32;
                        }
                    }
                    k += 1;
                }
                if best < 0 {
                    break;
                }
                let chosen = ids[best as usize];
                ids[best as usize] = 0xffff_ffff;
                // Notify scan over pool A.
                let ha = g32(POOL_A);
                let na = rd32(ha + HDR_COUNT) as i32;
                if na > 0 {
                    let mut j = 0i32;
                    while j < na {
                        let fl = rd32(ha + HDR_FLAGS);
                        if rd8(fl + j as u32) & SKIP_BIT == 0 {
                            let stride = rd32(ha + HDR_STRIDE);
                            let obj = rd32(ha + HDR_BASE).wrapping_add(stride.wrapping_mul(j as u32));
                            if obj != 0 && rd16s(obj + OBJ_KEY) == chosen as i32 {
                                lf_checker_rt::callee_cdecl!(8, u32, obj, 1);
                            }
                        }
                        j += 1;
                    }
                }
                // Notify scan over pool B.
                let hb = g32(POOL_B);
                let nb = rd32(hb + HDR_COUNT) as i32;
                if nb > 0 {
                    let mut j = 0i32;
                    while j < nb {
                        let fl = rd32(hb + HDR_FLAGS);
                        if rd8(fl + j as u32) & SKIP_BIT == 0 {
                            let stride = rd32(hb + HDR_STRIDE);
                            let obj = rd32(hb + HDR_BASE).wrapping_add(stride.wrapping_mul(j as u32));
                            if obj != 0 && rd16s(obj + OBJ_KEY) == chosen as i32 {
                                if rd8(obj + 0x26c) & 4 != 0 {
                                    let holder = rd32(obj + 0xb30);
                                    if holder != 0 {
                                        let vt = rd32(holder);
                                        vcall0(rd32(vt + SLOT_NOTIFY), holder);
                                    } else {
                                        lf_checker_rt::callee_cdecl!(8, u32, obj, 1);
                                    }
                                } else {
                                    lf_checker_rt::callee_cdecl!(8, u32, obj, 1);
                                }
                            }
                        }
                        j += 1;
                    }
                }
                if !((rd32(this + 0x708) as i32) > GATE_B) {
                    break;
                }
            }
        }

        // ---- Phase C: gather over pool C ----
        lf_checker_rt::callee_thiscall!(1, u32, slots_p);
        lf_checker_rt::callee_thiscall!(2, u32, slots_p, this, this + 0x100, this + 0x300, 0);
        let count2 = lf_checker_rt::callee_thiscall!(4, u32, slots_p) as i32;
        let mut kept2 = 0usize;
        if count2 > 0 {
            let mut i = 0i32;
            while i < count2 {
                let id = lf_checker_rt::callee_thiscall!(5, u32, slots_p, i as u32);
                ids[kept2] = id;
                acc[kept2] = 0.0;
                let g = lf_checker_rt::callee_cdecl!(6, u32, id, g32(FIELD_FLAGS));
                if (g as u8) & GATE_BITS != 0 {
                    i += 1;
                    continue;
                }
                let mut dl = 1u8;
                let hc = g32(POOL_C);
                let nc = rd32(hc + HDR_COUNT) as i32;
                if nc > 0 {
                    let mut j = 0i32;
                    while j < nc {
                        let fl = rd32(hc + HDR_FLAGS);
                        if rd8(fl + j as u32) & SKIP_BIT == 0 {
                            let stride = rd32(hc + HDR_STRIDE);
                            let obj = rd32(hc + HDR_BASE).wrapping_add(stride.wrapping_mul(j as u32));
                            if obj != 0 {
                                let vt = rd32(obj + OBJ_VTABLE);
                                let r = vcall0(rd32(vt + SLOT_VCHECK), obj);
                                if (r as u8) == 0 {
                                    dl = 0;
                                } else {
                                    let s = lf_checker_rt::callee_cdecl!(9, u32, obj);
                                    if (s as u8) != 0 {
                                        dl = 0;
                                    } else {
                                        let pp = rd32(obj + OBJ_POS);
                                        let t = lf_checker_rt::callee_cdecl!(10, u32, pp + 0x30);
                                        if (t as u8) != 0 {
                                            dl = 0;
                                        } else {
                                            let inc = proximity_inc(rdf(pp + 0x30), rdf(pp + 0x34), qx, qy, false);
                                            acc[kept2] = fadd(inc, acc[kept2]);
                                        }
                                    }
                                }
                            }
                        }
                        j += 1;
                    }
                }
                if dl != 0 {
                    kept2 += 1;
                }
                i += 1;
            }
        }

        // ---- Phase D: drain best-first through teardown ----
        if (rd32(this + 0x70c) as i32) > GATE_D {
            loop {
                let mut best = -1i32;
                let mut cur = (lf_checker_rt::global::<f32>(INIT_BEST) as *const f32).read();
                let mut k = 0usize;
                while k < kept2 {
                    if (ids[k] as i32) >= 0 {
                        let a = acc[k];
                        if cur > a {
                            cur = a;
                            best = k as i32;
                        }
                    }
                    k += 1;
                }
                if best < 0 {
                    break;
                }
                let chosen = ids[best as usize];
                ids[best as usize] = 0xffff_ffff;
                let hc = g32(POOL_C);
                let nc = rd32(hc + HDR_COUNT) as i32;
                if nc > 0 {
                    let mut j = 0i32;
                    while j < nc {
                        let fl = rd32(hc + HDR_FLAGS);
                        if rd8(fl + j as u32) & SKIP_BIT == 0 {
                            let stride = rd32(hc + HDR_STRIDE);
                            let obj = rd32(hc + HDR_BASE).wrapping_add(stride.wrapping_mul(j as u32));
                            if obj != 0 && rd16s(obj + OBJ_KEY) == chosen as i32 {
                                lf_checker_rt::callee_cdecl!(11, u32, obj);
                            }
                        }
                        j += 1;
                    }
                }
                if !((rd32(this + 0x70c) as i32) > GATE_D) {
                    break;
                }
            }
        }

        lf_checker_rt::callee_cdecl!(12, u32,);
        this
    }
});
