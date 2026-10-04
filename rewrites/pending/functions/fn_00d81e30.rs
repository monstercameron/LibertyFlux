// original: 0x00d81e30 DODGE
// Rewrite of the dodge-event update: walks a linked list of candidate nodes,
// and for each node whose bounds pass a cascade of float gates, drives the
// ped/task callee sequence (direction query, range tests, event posting).
// Always returns 0.

/// Dodge-event update over a candidate list (cdecl, ten stack words).
///
/// `a0` points to a word holding the list head (each node: `+0` the secondary
/// object for that iteration, `+4` the next node, null ends the walk). `a1`
/// is the main object: `+0` vtable (slots `0x60`, `0x64`, `0xec`, `0x1a8`
/// queried here), `+0x20` the first secondary object, `+0x28`/`+0x2c` flag
/// words, `+0xf50` an
/// optional helper (byte at `+0x219` gates two call sites), `+0x12ec` and
/// `+0x1300` small mode words, byte `+0xef8` (bit 2 set when the event path
/// runs) and word `+0xf0c` (scratch slot, overwritten with the secondary
/// object). `a2`..`a5` are float bounds tested against the node's position
/// triple, `a6` points at a float read and rewritten with a clamped blend
/// factor, `a7` is a float multiplier, `a8` is a small flag object (bytes at
/// `+0x28`/`+0x2b` steer the callee choice) and `a9`, when non-null, receives
/// two timer words and a state byte.
///
/// Behaviour: three virtual queries seed two running floats (a dot product
/// and a direct read); each list node whose stamp differs and whose triple
/// passes the bound gates runs the direction callee, a scalar range call
/// returning through the x87 top, and — past more gates — the task/event
/// callees. All float arithmetic keeps the original's operand order (see the
/// helpers); every `comiss` gate is an ordered Rust comparison, ordered so a
/// NaN takes the same side as the original's unordered flags. The one
/// flags-through-`lahf` test passes exactly when the running value is
/// positive or NaN. The per-index table entry is read through the relocated
/// image base. Always returns 0.
lf_checker_rt::export!(cdecl, rw_00d81e30(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, a9: u32) -> u32 {
    unsafe {
        // Callees by contract id (direct sites are patched; indirect slots
        // are called through the fabricated objects like the original).
        const C_TASK_FIND: u32 = 1;
        const C_HELPER_PROBE: u32 = 2;
        const C_POOL_ALLOC: u32 = 3;
        const C_TASK_BY_TYPE: u32 = 4;
        const C_NOTIFY: u32 = 5;
        const C_UNLINK: u32 = 6;
        const C_LINK: u32 = 7;
        const C_SPEECH: u32 = 8;
        const C_STATE_POLL: u32 = 9;
        const C_FALLBACK: u32 = 10;
        const C_DIRECTION: u32 = 11;
        const C_RANGE_STATE: u32 = 12;
        const C_RANGE_SCALAR: u32 = 13;
        const C_EVENT_NEW: u32 = 14;
        const C_EVENT_POST: u32 = 15;
        const C_EVENT_FREE_A: u32 = 16;
        const C_EVENT_FREE_B: u32 = 17;
        const C_EVENT_FILL: u32 = 18;
        const C_SPAWN: u32 = 19;
        const C_GATE_CHECK: u32 = 20;
        const C_COUNTER: u32 = 21;
        // Indirect slots.
        const SLOT_POS: u32 = 0x64;
        const SLOT_ALT: u32 = 0x60;
        const SLOT_VEC: u32 = 0xec;
        const SLOT_SUB_A: u32 = 0x128;
        const SLOT_SUB_B: u32 = 0xfc;
        const SLOT_FLAG: u32 = 0x1a8;
        // Main object layout.
        const O_VT: u32 = 0x00;
        const O_SUB: u32 = 0x20;
        const O_FLAGS: u32 = 0x28;
        const O_SEQ: u32 = 0x2c;
        const O_MARK: u32 = 0xef8;
        const O_SCRATCH: u32 = 0xf0c;
        const O_HELPER: u32 = 0xf50;
        const O_MODE_A: u32 = 0x12ec;
        const O_MODE_B: u32 = 0x1300;
        // Secondary object layout.
        const S_VT: u32 = 0x00;
        const S_D0: u32 = 0x10;
        const S_D1: u32 = 0x14;
        const S_D2: u32 = 0x18;
        const S_VEC: u32 = 0x20;
        const S_LIVE: u32 = 0x24;
        const S_KIND: u32 = 0x28;
        const S_INDEX: u32 = 0x2e;
        const S_M0: u32 = 0x30;
        const S_M1: u32 = 0x34;
        const S_M2: u32 = 0x38;
        const S_STAMP: u32 = 0x3c;
        const S_GATE_B: u32 = 0x219;
        const S_TASK: u32 = 0x224;
        const S_PARENT: u32 = 0x228;
        const S_SPEECH_THIS: u32 = 0x570;
        const S_READY: u32 = 0x7b8;
        const S_GATE_C: u32 = 0xa60;
        const S_OWNER: u32 = 0xab0;
        // Globals (file VAs; read through the relocated image).
        const G_TWO: u32 = 0x00FE8A24;
        const G_BLEND_A: u32 = 0x00FE8788;
        const G_BLEND_B: u32 = 0x00FE8AFC;
        const G_NEG_TWO: u32 = 0x00FE8DB0;
        const G_Z_LIMIT: u32 = 0x00FE8AE0;
        const G_ONE: u32 = 0x00FE88E8;
        const G_DEVIATION: u32 = 0x00FE8A60;
        const G_WEIGHT: u32 = 0x00FE897C;
        const G_HALF: u32 = 0x00FE8830;
        const G_FAR: u32 = 0x00FE8B20;
        const G_NEAR: u32 = 0x00FE8B14;
        const G_HIGH: u32 = 0x00FE8AB8;
        const G_TINY: u32 = 0x00FE8734;
        const G_SMALL: u32 = 0x00FE881C;
        const G_BIG: u32 = 0x00FE8B08;
        const G_QUARTER: u32 = 0x00FE87E4;
        const G_TICK: u32 = 0x00FE8684;
        const G_EPS: u32 = 0x00ECAA24;
        const G_CFG_A: u32 = 0x01056D3C;
        const G_CFG_B: u32 = 0x01056D44;
        const G_CFG_C: u32 = 0x01056D48;
        const G_THRESHOLD: u32 = 0x01056D4C;
        const G_ALT_A: u32 = 0x01056D50;
        const G_ALT_B: u32 = 0x01056D54;
        const G_ENABLE: u32 = 0x010521A0;
        const G_CLOCK: u32 = 0x011735B4;
        const G_FRAME: u32 = 0x01173604;
        const G_STAMP: u32 = 0x011A8908;
        const G_POOL: u32 = 0x0167E2A0;
        const G_TABLE: u32 = 0x01295CD8;
        const S_SPEECH_A: u32 = 0x00EEC8C4;
        const S_SPEECH_B: u32 = 0x00EEC8D8;
        const S_FALLBACK: u32 = 0x00EEC8E0;
        const SIGN_BIT: u32 = 0x8000_0000;

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
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn rgf(va: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(va))) }
        }
        #[inline(always)]
        unsafe fn rg32(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn rg16(va: u32) -> u32 {
            unsafe { rd16(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn rg8(va: u32) -> u8 {
            unsafe { rd8(lf_checker_rt::relocated(va)) }
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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn neg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ SIGN_BIT)
        }
        #[inline(always)]
        unsafe fn call_slot(this: u32, slot: u32, arg: Option<u32>) -> u32 {
            unsafe {
                let addr = rd32(rd32(this + O_VT) + slot);
                if let Some(v) = arg {
                    let f: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(addr as usize);
                    f(this, v)
                } else {
                    let f: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(addr as usize);
                    f(this)
                }
            }
        }
        #[inline(always)]
        unsafe fn call_slot_f32(this: u32, slot: u32) -> f32 {
            unsafe {
                let addr = rd32(rd32(this + S_VT) + slot);
                let f: extern "thiscall" fn(u32) -> f32 =
                    core::mem::transmute(addr as usize);
                f(this)
            }
        }

        let a2f = f32::from_bits(a2);
        let a3f = f32::from_bits(a3);
        let a4f = f32::from_bits(a4);
        let a5f = f32::from_bits(a5);
        let a7f = f32::from_bits(a7);

        // Seed queries: position float, dot product, direct read.
        let f_pos = rdf(call_slot(a1, SLOT_POS, None));
        let mut esi = rd32(a1 + O_SUB);
        let r_vec = call_slot(a1, SLOT_VEC, Some(0));
        let dot = {
            let p0 = mul(rdf(r_vec), rdf(esi + S_D0));
            let p1 = mul(rdf(r_vec + 4), rdf(esi + S_D1));
            let s = add(p1, p0);
            add(s, mul(rdf(r_vec + 8), rdf(esi + S_D2)))
        };
        let helper = rd32(a1 + O_HELPER);
        let helper_live = helper != 0 && rd8(helper + 0x219) != 0;
        let flag_b = helper_live && rd32(a1 + O_MODE_A) != 0;
        let flag_a = if helper_live {
            true
        } else {
            let mode = rd32(a1 + O_MODE_B);
            if mode == 3 || mode == 5 || mode == 4 {
                true
            } else {
                let b = rd8(a8 + 0x28);
                if b != 0 && b != 1 && b != 6 {
                    true
                } else {
                    rd32(a1 + O_FLAGS) & 0x7c00 == 0x400
                }
            }
        };
        let mut node = rd32(a0);
        let r_pos2 = call_slot(a1, SLOT_POS, None);
        let run_b = rdf(r_pos2 + 4);
        let r_alt = call_slot(a1, SLOT_ALT, None);
        let cell_90 = rdf(r_alt + 4);
        let run_a = dot;

        // Pre-loop shaping of the blend inputs.
        let two = rgf(G_TWO);
        let mut x0 = sub(run_a, two);
        if x0 < 0.0 {
            x0 = 0.0;
        }
        if run_a < 0.0 {
            x0 = neg(run_a);
        }
        let mut blend = 0.0f32;
        if x0 > 0.0 {
            blend = add(mul(mul(x0, rgf(G_BLEND_A)), rgf(G_BLEND_B)), two);
        }
        let mut cell_74 = add(rgf(G_CFG_A), f_pos);
        let mut x2 = mul(rgf(G_CFG_C), run_a);
        let mut x6 = two;
        if two > x2 {
            x6 = x2;
        }
        let mut cell_7c = add(blend, run_b);
        let mut cell_78 = add(rgf(G_CFG_B), f_pos);
        x6 = add(x6, run_b);
        let mut cell_80 = x6;
        // Negative running value: reshuffle the cells (rare path).
        if run_a < 0.0 {
            x0 = cell_74;
            let x3 = cell_90;
            cell_80 = x0;
            let e1 = cell_80.to_bits();
            x0 = x3;
            x0 = sub(x0, blend);
            let cell_38n = f32::from_bits(e1);
            let mut cell_84 = x0;
            x0 = rgf(G_NEG_TWO);
            let e2 = cell_84.to_bits();
            let cell_3c = f32::from_bits(e2);
            if !(x2 > x0) {
                x2 = x0;
            }
            x0 = cell_78;
            cell_80 = x0;
            let e3 = cell_80.to_bits();
            x0 = x3;
            x0 = sub(x0, x2);
            let _cell_48n = f32::from_bits(e3);
            cell_84 = x0;
            x0 = cell_3c;
            let e4 = cell_84.to_bits();
            cell_7c = x0;
            x0 = cell_38n;
            cell_74 = x0;
            let cell_4c = f32::from_bits(e4);
            x0 = cell_4c;
            cell_80 = x0;
            x0 = _cell_48n;
            cell_78 = x0;
        }

        if node == 0 {
            return 0;
        }
        let stamp = rg16(G_STAMP);
        // The secondary object hung off the main one. The loop walks nodes
        // whose head word happens to alias it here, but the range math below
        // always reads this pointer, so it is kept separate from `esi`.
        let pa1e = rd32(a1 + O_SUB);
        loop {
            esi = rd32(node);
            node = rd32(node + 4);
            if rd32(esi + S_STAMP) == stamp {
                if node == 0 {
                    return 0;
                }
                continue;
            }
            if rd8(esi + S_LIVE) & 1 == 0 {
                if node == 0 {
                    return 0;
                }
                continue;
            }
            wr32(esi + S_STAMP, stamp);
            let vp = rd32(esi + S_VEC);
            let q = if vp != 0 {
                vp.wrapping_add(0x30)
            } else {
                esi.wrapping_add(0x10)
            };
            let n0 = rdf(q);
            if !(n0 > a2f) {
                if node == 0 {
                    return 0;
                }
                continue;
            }
            let mut cell_64 = rdf(q + 4);
            let mut cell_30 = rdf(q + 8);
            let mut cell_50 = n0;
            if !(a4f > n0) {
                if node == 0 {
                    return 0;
                }
                continue;
            }
            if !(cell_64 > a3f) {
                if node == 0 {
                    return 0;
                }
                continue;
            }
            if !(a5f > cell_64) {
                if node == 0 {
                    return 0;
                }
                continue;
            }
            let dev = sub(cell_30, rdf(pa1e + S_M2)).abs();
            if !(rgf(G_Z_LIMIT) > dev) {
                if node == 0 {
                    return 0;
                }
                continue;
            }
            // Direction query (patched direct callee; its frame argument is
            // skipped by the contract); mirrored when the flag byte is set.
            let r_dir: u32 = lf_checker_rt::callee_thiscall!(C_DIRECTION, u32, a1, 0u32);
            let (mut dx, mut dy, mut dz) = (rdf(r_dir), rdf(r_dir + 4), rdf(r_dir + 8));
            if rd8(a8 + 0x2b) & 1 != 0 {
                dx = neg(dx);
                dy = neg(dy);
                dz = neg(dz);
            }
            let range: f32 = lf_checker_rt::callee_cdecl!(
                C_RANGE_SCALAR,
                f32,
                rd32(pa1e + S_M0),
                rd32(pa1e + S_M1),
                dx.to_bits(),
                dy.to_bits(),
                cell_50.to_bits(),
                cell_64.to_bits()
            );
            let mut cell_38 = range;
            let y0 = add(mul(dz, cell_38), rdf(pa1e + S_M2));
            let y1 = sub(cell_30, y0).abs();
            if !(rgf(G_DEVIATION) > y1) {
                if node == 0 {
                    return 0;
                }
                continue;
            }
            let mut z0 = sub(cell_64, rdf(pa1e + S_M1));
            let mut z1 = sub(cell_50, rdf(pa1e + S_M0));
            let z3 = sub(cell_30, rdf(pa1e + S_M2));
            cell_64 = z0;
            z0 = mul(z0, dy);
            cell_50 = z1;
            z1 = mul(z1, dx);
            cell_30 = z3;
            z1 = add(z1, z0);
            z0 = mul(z3, dz);
            z1 = add(z1, z0);
            let cell_10 = z1;
            let a6f = rdf(a6);

            // Dispatch: secondary poll, or straight to the tail gates.
            let mut cell_40: u32 = 0;
            let mut via_poll = false;
            if a6f > 0.0 {
                let ab = rd8(a8 + 0x28);
                if ab == 0 || ab == 1 || ab == 4 || ab == 6 {
                    let sub_vt = rd32(esi + S_VT);
                    let f_poll: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(rd32(sub_vt + SLOT_SUB_A) as usize);
                    let ans = f_poll(esi);
                    if ans & 0xff == 0 {
                        cell_40 = 0;
                        via_poll = true;
                    } else if rd32(rd32(esi + S_PARENT) + 0x59c) != a1 {
                        cell_40 = esi;
                        via_poll = true;
                    }
                }
            }
            if via_poll {
                'poll: {
                    if !(cell_10 > run_b) {
                        break 'poll;
                    }
                    let w2 = sub(cell_10, run_b);
                    cell_38 = w2;
                    if !(run_a > w2) {
                        break 'poll;
                    }
                    let mut x7 = f_pos;
                    if rd32(a1 + O_MODE_B) == 1 {
                        x7 = mul(x7, rgf(G_WEIGHT));
                    }
                    let pa = pa1e;
                    let mut v1 = mul(rdf(pa + 4), cell_64);
                    v1 = add(v1, mul(rdf(pa), cell_50));
                    v1 = add(v1, mul(rdf(pa + 8), cell_30));
                    v1 = v1.abs();
                    let u0 = add(x7, rgf(G_HALF));
                    if u0 < v1 {
                        break 'poll;
                    }
                    let u0b = if cell_40 != 0 {
                        rgf(G_FAR)
                    } else {
                        rgf(G_NEAR)
                    };
                    let cell_48 = u0b;
                    if !(u0b > w2) {
                        break 'poll;
                    }
                    let task = rd32(esi + S_TASK);
                    let rs: u32 = lf_checker_rt::callee_thiscall!(
                        C_RANGE_STATE,
                        u32,
                        task.wrapping_add(0x2e0),
                        0u32
                    );
                    if rs == 0x51 || rs == 0x52 || rs == 0x54 || rs == 0x5a {
                        break 'poll;
                    }
                    let k1 = sub(cell_38, rgf(G_ONE));
                    let k1p = if k1 < 0.0 { 0.0 } else { k1 };
                    let k3 = div(rgf(G_ONE), cell_48);
                    // The original recomputes this scaled value four times
                    // from the same inputs; one computation is bit-identical.
                    let scaled = mul(mul(k1p, k3), a7f);
                    let one = rgf(G_ONE);
                    let big = one <= scaled || scaled.is_nan();
                    let cand = if big { scaled } else { one };
                    if cand > a6f {
                        // Writes the slot back unchanged (no observable diff).
                        wrf(a6, a6f);
                    } else if big {
                        wrf(a6, scaled);
                    } else {
                        wrf(a6, one);
                    }
                    wr8(a1 + O_MARK, rd8(a1 + O_MARK) | 4);
                    let link_arg = a1.wrapping_add(O_SCRATCH);
                    if rd32(a1 + O_SCRATCH) != 0 {
                        let prev: u32 = rd32(a1 + O_SCRATCH);
                        let _: u32 = lf_checker_rt::callee_thiscall!(C_UNLINK, u32, prev, link_arg);
                    }
                    wr32(a1 + O_SCRATCH, esi);
                    let _: u32 =
                        lf_checker_rt::callee_thiscall!(C_LINK, u32, esi, link_arg);
                    if a9 == 0 {
                        break 'poll;
                    }
                    if rgf(G_HIGH) > cell_38 {
                        let e = if cell_40 != 0 { 0x1770u32 } else { 0xfa0u32 };
                        wr8(a9 + 0x2a, 1);
                        wr32(a9 + 0x10, e.wrapping_add(rg32(G_CLOCK)));
                    }
                    if rgf(G_DEVIATION) > cell_38 {
                        let e = if cell_40 != 0 { 0x1770u32 } else { 0xfa0u32 };
                        wr8(a9 + 0x2a, 0x18);
                        wr32(a9 + 0x10, e.wrapping_add(rg32(G_CLOCK)));
                    }
                    let seq = rd16(a1 + O_SEQ);
                    if seq.wrapping_add(rg32(G_FRAME)) & 3 == 2 {
                        let _: u32 = call_slot(a1, SLOT_FLAG, Some(1));
                    }
                }
            }

            // Tail gates shared by both paths.
            if rd32(esi + S_KIND) & 0x3c0 != 0xc0 {
                if node == 0 {
                    return 0;
                }
                continue;
            }
            if !flag_a {
                if node == 0 {
                    return 0;
                }
                continue;
            }
            if rd32(esi + S_OWNER) == a1 {
                if node == 0 {
                    return 0;
                }
                continue;
            }
            // lahf gate: pass when the running value is positive or NaN.
            if run_a <= 0.0 {
                if node == 0 {
                    return 0;
                }
                continue;
            }
            let c1 = !(0.0 > cell_10);
            let c2 = !(0.0 > run_a);
            if c1 != c2 {
                if node == 0 {
                    return 0;
                }
                continue;
            }
            let ax4 = run_a.abs();
            cell_38 = ax4;
            if !(ax4 > rgf(G_EPS)) {
                if node == 0 {
                    return 0;
                }
                continue;
            }
            let ax1 = cell_10.abs();
            let mut cell_48 = ax1;
            if !(ax1 > run_b) {
                if node == 0 {
                    return 0;
                }
                continue;
            }
            let ctr: u32 = lf_checker_rt::callee_cdecl!(C_COUNTER, u32,);
            let m5 = cell_30;
            let m2 = mul((ctr as i32) as f32, rgf(G_TICK));
            let pa = pa1e;
            let mut m1 = mul(rdf(pa + 4), cell_64);
            let mut m0 = mul(rdf(pa), cell_50);
            m1 = add(m1, m0);
            m0 = mul(rdf(pa + 8), m5);
            m1 = add(m1, m0);
            let pb = rd32(esi + S_VEC).wrapping_add(0x10);
            m0 = mul(rdf(pb), cell_50);
            m1 = m1.abs();
            cell_30 = m1;
            m1 = mul(rdf(pb + 4), cell_64);
            m1 = add(m1, m0);
            m0 = mul(rdf(pb + 8), m5);
            m1 = add(m1, m0);
            let al: u8 = if 0.0 > m1 || flag_b || run_b > cell_48 {
                1
            } else {
                0
            };
            let mut cell_0f = al;
            let mut cell_2e = al;
            if al == 0 {
                cell_0f = if rgf(G_TINY) > m2 { 1 } else { 0 };
            } else {
                cell_2e = if rgf(G_SMALL) > m2 { 1 } else { 0 };
            }
            let task = rd32(esi + S_TASK);
            let found: u32 = lf_checker_rt::callee_thiscall!(
                C_TASK_FIND,
                u32,
                task.wrapping_add(0x2e0),
                0x1f6u32,
                0u32
            );
            let ecx_v: u32;
            if found & 0xff == 0 {
                ecx_v = 0;
            } else {
                let by_type: u32 = lf_checker_rt::callee_thiscall!(
                    C_TASK_BY_TYPE,
                    u32,
                    task.wrapping_add(0x44),
                    0x1f6u32
                );
                if by_type == 0 {
                    ecx_v = 0;
                } else if rd8(by_type + 0x45) != 0 {
                    ecx_v = 1;
                } else {
                    ecx_v = 0;
                }
            }
            if cell_0f == 0 {
                if node == 0 {
                    return 0;
                }
                continue;
            }
            let q2 = cell_10;
            let q5 = cell_7c;
            let mut al2: u8 = 0;
            if q5 > q2 {
                let q0 = add(run_b, two);
                if q2 > q0 {
                    al2 = 1;
                }
            }
            if 0.0 > run_a {
                if q2 > q5 {
                    let q0 = sub(cell_90, two);
                    al2 = if q0 > q2 { 1 } else { 0 };
                } else {
                    al2 = 0;
                }
            }
            let q0b = cell_80;
            let mut dl: u8 = if q0b > q2 { 1 } else { 0 };
            if 0.0 > run_a {
                dl = if q2 > q0b { 1 } else { 0 };
            }
            let r1 = cell_38;
            let mut run_path_d = ecx_v & 0xff != 0;
            if !run_path_d {
                'deep: {
                    if !(rgf(G_FAR) > r1) {
                        run_path_d = true;
                        break 'deep;
                    }
                    if dl == 0 {
                        run_path_d = true;
                        break 'deep;
                    }
                    if cell_78 < cell_30 {
                        run_path_d = true;
                        break 'deep;
                    }
                    if rd32(esi + S_READY) != 2 {
                        break 'deep;
                    }
                    let gate: u32 =
                        lf_checker_rt::callee_cdecl!(C_GATE_CHECK, u32, esi, 8u32, a1, 0u32);
                    if gate & 0xff == 0 {
                        break 'deep;
                    }
                    let pooled: u32 =
                        lf_checker_rt::callee_thiscall!(C_POOL_ALLOC, u32, rg32(G_POOL));
                    let spawned: u32;
                    if pooled == 0 {
                        spawned = 0;
                    } else {
                        spawned = lf_checker_rt::callee_thiscall!(
                            C_SPAWN,
                            u32,
                            pooled,
                            0x7d0u32,
                            0x7530u32,
                            a1,
                            0u32
                        );
                    }
                    let e219 = rd8(esi + S_GATE_B);
                    let idx = rd16(esi + S_INDEX) as u16 as i16 as i32;
                    let tent = rd32(
                        lf_checker_rt::relocated(G_TABLE)
                            .wrapping_add((idx as u32).wrapping_mul(4)),
                    );
                    let tbyte = rd8(tent.wrapping_add(0xef));
                    cell_38 = if e219 != 0 {
                        rgf(G_ALT_A)
                    } else if rd8(esi + S_GATE_C) == 2 {
                        rgf(G_ALT_A)
                    } else {
                        rgf(G_ALT_B)
                    };
                    if rg8(G_ENABLE) != 0 {
                        if (tbyte as i32) < rg32(G_THRESHOLD) as i32 {
                            wr32(spawned.wrapping_add(0x58), 1);
                        } else {
                            let got = call_slot_f32(esi, SLOT_SUB_B);
                            cell_48 = got;
                            if cell_38 > cell_48 {
                                wr32(spawned.wrapping_add(0x58), 1);
                            }
                        }
                    }
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        C_EVENT_POST,
                        u32,
                        0u32,
                        0x7530u32,
                        spawned,
                        0u32
                    );
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        C_EVENT_FILL,
                        u32,
                        task.wrapping_add(0x84),
                        0u32,
                        0u32,
                        1u32
                    );
                    let polled: u32 = lf_checker_rt::callee_thiscall!(C_STATE_POLL, u32, esi);
                    if polled & 0xff == 0 {
                        let _: u32 = lf_checker_rt::callee_thiscall!(
                            C_FALLBACK,
                            u32,
                            esi,
                            lf_checker_rt::relocated(S_FALLBACK),
                            0x3e000000u32,
                            0u32,
                            0u32
                        );
                    } else {
                        let helper2 = rd32(a1 + O_HELPER);
                        if helper2 == 0 {
                            let _: u32 = lf_checker_rt::callee_thiscall!(
                                C_FALLBACK,
                                u32,
                                esi,
                                lf_checker_rt::relocated(S_FALLBACK),
                                0x3e000000u32,
                                0u32,
                                0u32
                            );
                        } else {
                            let probe: u32 = lf_checker_rt::callee_thiscall!(
                                C_HELPER_PROBE,
                                u32,
                                helper2
                            );
                            if probe & 0xff == 0 {
                                let _: u32 = lf_checker_rt::callee_thiscall!(
                                    C_FALLBACK,
                                    u32,
                                    esi,
                                    lf_checker_rt::relocated(S_FALLBACK),
                                    0x3e000000u32,
                                    0u32,
                                    0u32
                                );
                            } else {
                                let ctr2: u32 =
                                    lf_checker_rt::callee_cdecl!(C_COUNTER, u32,);
                                let yy = mul((ctr2 as i32) as f32, rgf(G_TICK));
                                if rgf(G_QUARTER) > yy {
                                    let say: u32 = lf_checker_rt::callee_thiscall!(
                                        C_SPEECH,
                                        u32,
                                        esi.wrapping_add(S_SPEECH_THIS),
                                        lf_checker_rt::relocated(S_SPEECH_A),
                                        0u32,
                                        0u32,
                                        0u32,
                                        0xffffffffu32,
                                        0u32,
                                        0u32,
                                        0x3f800000u32,
                                        0u32,
                                        0u32
                                    );
                                    if say & 0xff == 0 {
                                        let _: u32 = lf_checker_rt::callee_thiscall!(
                                            C_SPEECH,
                                            u32,
                                            esi.wrapping_add(S_SPEECH_THIS),
                                            lf_checker_rt::relocated(S_SPEECH_B),
                                            0u32,
                                            0u32,
                                            0u32,
                                            0xffffffffu32,
                                            0u32,
                                            0u32,
                                            0x3f800000u32,
                                            0u32,
                                            0u32
                                        );
                                    }
                                }
                            }
                        }
                    }
                    let _: u32 = lf_checker_rt::callee_thiscall!(C_EVENT_FREE_B, u32, 0u32);
                }
            }
            if run_path_d {
                if cell_2e != 0 && !(cell_74 < cell_30) && r1 > rgf(G_BIG) && al2 != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(C_EVENT_NEW, u32, 0u32, a1);
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        C_EVENT_FILL,
                        u32,
                        task.wrapping_add(0x84),
                        0u32,
                        0u32,
                        1u32
                    );
                    let _: u32 = lf_checker_rt::callee_thiscall!(C_EVENT_FREE_A, u32, 0u32);
                }
            }
            let helper3 = rd32(a1 + O_HELPER);
            if helper3 != 0 && rd8(helper3 + 0x219) != 0 {
                let _: u32 =
                    lf_checker_rt::callee_thiscall!(C_NOTIFY, u32, task, 2u32);
            }
            if node == 0 {
                return 0;
            }
        }
    }
});
