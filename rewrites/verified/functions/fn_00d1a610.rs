// original: 0x00d1a610 ped_task_posture_update (proposed)

/// Refresh the posture record: gate on the task kind, then solve and store.
///
/// `this` is the posture record, `arg0` the task. Callee 1 (null-checked
/// first word of `this+0x3c` chain head) returning nonzero, or the kind
/// predicate failing (`[arg0+0x21c]+0x12c != 2`, with the `+0xa60 == 1` or
/// `+0x264 & 0x400000` alternative), clears bit `0x1000` in `this+0x60` and
/// returns the entry answer with the predicate in the low byte.
///
/// Otherwise the solver runs: up to five probe calls through vtable slot
/// `+0x128` of the object at `this+0x3c` (callee 7), an optional kind call
/// through slot `+0xc` of `this+8` (callee 8, wanting `0x772`), helper calls
/// (callees 2-6), and float work: the squared distance of `this+0x80` from
/// the pose base, an accumulator at `this+0x68` (zeroed or advanced by the
/// game rate at file `0x11735bc`), and cap selection between the constants
/// at file `0xfe8b80`/`0xfe8ad8`. The mode byte at file `0x12fa6b2` steers
/// the tail fork. Every exit stores the solved 4-word pose (`+0x80`) from
/// the pose base and returns whatever value sits in eax on its path (a call
/// answer, a loaded pointer, or a pose word), reproduced exactly.
///
/// Original: thiscall, one stack word (the callee pops 4 bytes).
lf_checker_rt::export!(thiscall, rw_00d1a610(this: u32, arg0: u32) -> u32 {
    unsafe {
        const OBJ_OFF: u32 = 0x3c;
        const PED_OFF: u32 = 0x21c;
        const KIND_OFF: u32 = 0x12c;
        const KIND_WANT: u32 = 2;
        const FLAG_A60: u32 = 0xa60;
        const FLAG_264: u32 = 0x264;
        const MASK_264: u32 = 0x400000;
        const VT_PROBE: u32 = 0x128;
        const VT_KIND: u32 = 0x0c;
        const KIND2_WANT: u32 = 0x772;
        const CHAIN_OFF: u32 = 0x228;
        const CHAIN_ADD: u32 = 0x70;
        const STATE_OFF: u32 = 0x60;
        const STATE_BIT: u32 = 0x1000;
        const ACC_OFF: u32 = 0x68;
        const LATCH_OFF: u32 = 0x6c;
        const POSE_OFF: u32 = 0x80;
        const SRC_OFF: u32 = 0x20;
        const TRIPLE_OFF: u32 = 0x30;
        const E8_OFF: u32 = 8;
        const BIT118_OFF: u32 = 0x118;
        const A70_OFF: u32 = 0xa70;
        const LOOKUP_OFF: u32 = 0x224;
        const LOOKUP_ADD: u32 = 0x2e0;
        const HELPER_ADD: u32 = 0x2b0;
        const DIST_CONST: u32 = 0xfe8b40;
        const RATE_GLOBAL: u32 = 0x11735bc;
        const CAP_A: u32 = 0xfe8b80;
        const CAP_B: u32 = 0xfe8ad8;
        const MODE_GLOBAL: u32 = 0x12fa6b2;
        const LOOKUP_PROBE: u32 = 0x1af;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            ((a) as *const u32).read_unaligned()
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            ((a) as *const f32).read_unaligned()
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            ((a) as *mut u32).write_unaligned(v)
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            ((a) as *mut f32).write_unaligned(v)
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            ((a) as *const u8).read()
        }
        #[inline(always)]
        unsafe fn vcall0(o: u32, slot: u32) -> u32 {
            let vt = ((o) as *const u32).read_unaligned();
            let h: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                (((vt + slot)) as *const u32).read_unaligned() as usize,
            );
            h(o)
        }
        #[inline(always)]
        unsafe fn copy_pose(this: u32, o: u32) -> u32 {
            let b = rd32(o.wrapping_add(SRC_OFF));
            let w0 = rd32(b.wrapping_add(TRIPLE_OFF));
            let w1 = rd32(b.wrapping_add(TRIPLE_OFF + 4));
            let w2 = rd32(b.wrapping_add(TRIPLE_OFF + 8));
            let w3 = rd32(b.wrapping_add(TRIPLE_OFF + 12));
            wr32(this.wrapping_add(POSE_OFF), w0);
            wr32(this.wrapping_add(POSE_OFF + 4), w1);
            wr32(this.wrapping_add(POSE_OFF + 8), w2);
            wr32(this.wrapping_add(POSE_OFF + 12), w3);
            w3
        }
        let a1: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);
        let mut eax = a1;
        let bl0 = a1 as u8;
        let edx = rd32(arg0.wrapping_add(PED_OFF));
        let mut pred: u8 = 0;
        if rd32(edx.wrapping_add(KIND_OFF)) == KIND_WANT {
            if rd8(arg0.wrapping_add(FLAG_A60)) == 1 {
                pred = 1;
            } else if rd32(arg0.wrapping_add(FLAG_264)) & MASK_264 != 0 {
                pred = 1;
            }
        }
        eax = (eax & 0xffffff00) | (pred as u32);
        if bl0 != 0 || pred == 0 {
            wr32(this.wrapping_add(STATE_OFF), rd32(this.wrapping_add(STATE_OFF)) & !STATE_BIT);
            return eax;
        }
        let obj = rd32(this.wrapping_add(OBJ_OFF));
        let v1a = vcall0(obj, VT_PROBE);
        eax = v1a;
        let mut edi = 0u32;
        if v1a as u8 != 0 {
            let e = rd32(obj.wrapping_add(CHAIN_OFF));
            eax = e;
            if e != 0 {
                eax = e.wrapping_add(CHAIN_ADD);
                if eax != 0 {
                    edi = eax;
                }
            }
        }
        let bb = core::hint::black_box;
        let base = rd32(obj.wrapping_add(SRC_OFF));
        let dy = bb(rdf(this.wrapping_add(0x84))) - bb(rdf(base.wrapping_add(0x34)));
        let dx = bb(rdf(this.wrapping_add(0x80))) - bb(rdf(base.wrapping_add(0x30)));
        let dz = bb(rdf(this.wrapping_add(0x88))) - bb(rdf(base.wrapping_add(0x38)));
        let d2 = bb(dy) * bb(dy) + bb(dx) * bb(dx) + bb(dz) * bb(dz);
        let v1b = vcall0(obj, VT_PROBE);
        eax = v1b;
        let mut to_bh1 = false;
        if v1b as u8 == 0 {
            if rd8(obj.wrapping_add(FLAG_A60)) == 2 {
                to_bh1 = true;
            } else {
                let b1: u32 = lf_checker_rt::callee_thiscall!(
                    2,
                    u32,
                    obj.wrapping_add(HELPER_ADD),
                    1u32
                );
                eax = b1;
                if b1 as u8 != 0 {
                    to_bh1 = true;
                }
            }
        }
        let mut bh: u8 = 0;
        if to_bh1 {
            bh = 1;
        } else if rd8(obj.wrapping_add(BIT118_OFF)) & 1 != 0 {
            bh = 1;
        } else if edi != 0 {
            let n1: u32 = lf_checker_rt::callee_thiscall!(3, u32, edi);
            eax = n1;
            if (n1 as i32) >= 2 {
                bh = 1;
            }
        }
        let n2_is_one = if edi != 0 {
            let n2: u32 = lf_checker_rt::callee_thiscall!(3, u32, edi);
            eax = n2;
            n2 == 1
        } else {
            false
        };
        let mut bl: u8;
        if edi != 0 && n2_is_one {
            bl = 1;
        } else {
            let v1c = vcall0(obj, VT_PROBE);
            eax = v1c;
            if v1c as u8 != 0 {
                bl = 0;
            } else if rd8(obj.wrapping_add(FLAG_A60)) != 1 {
                bl = 0;
            } else {
                let b2: u32 = lf_checker_rt::callee_thiscall!(
                    2,
                    u32,
                    obj.wrapping_add(HELPER_ADD),
                    1u32
                );
                eax = b2;
                bl = if b2 as u8 != 0 { 0 } else { 1 };
            }
        }
        let dist_gt = d2 > lf_checker_rt::global::<f32>(DIST_CONST).read_unaligned();
        let m60 = rd32(this.wrapping_add(STATE_OFF));
        eax = m60 >> 12;
        if eax & 1 == 0 {
            eax = copy_pose(this, obj);
            wrf(this.wrapping_add(ACC_OFF), 0.0);
            let v1d = vcall0(obj, VT_PROBE);
            eax = v1d;
            if v1d as u8 == 0 {
                let b3: u32 = lf_checker_rt::callee_thiscall!(
                    2,
                    u32,
                    obj.wrapping_add(HELPER_ADD),
                    1u32
                );
                eax = b3;
                if b3 as u8 == 0 {
                    return eax;
                }
            }
            if lf_checker_rt::global::<u8>(MODE_GLOBAL).read() != 0 {
                return eax;
            }
            let obj2 = rd32(this.wrapping_add(OBJ_OFF));
            eax = obj2;
            if rd32(obj2.wrapping_add(A70_OFF)) != 1 && bh != 0 {
                return eax;
            }
            wr32(this.wrapping_add(STATE_OFF), m60 | STATE_BIT);
            return eax;
        }
        let e8 = rd32(this.wrapping_add(E8_OFF));
        if e8 == 0 {
            wrf(
                this.wrapping_add(ACC_OFF),
                bb(lf_checker_rt::global::<f32>(RATE_GLOBAL).read_unaligned())
                    + bb(rdf(this.wrapping_add(ACC_OFF))),
            );
        } else {
            let v2 = vcall0(e8, VT_KIND);
            eax = v2;
            if v2 == KIND2_WANT {
                wrf(this.wrapping_add(ACC_OFF), 0.0);
            } else {
                wrf(
                    this.wrapping_add(ACC_OFF),
                    bb(lf_checker_rt::global::<f32>(RATE_GLOBAL).read_unaligned())
                        + bb(rdf(this.wrapping_add(ACC_OFF))),
                );
            }
        }
        let cap_a = lf_checker_rt::global::<f32>(CAP_A).read_unaligned();
        let frame0c = if bl != 0 {
            cap_a
        } else {
            lf_checker_rt::global::<f32>(CAP_B).read_unaligned()
        };
        if rdf(this.wrapping_add(ACC_OFF)) > cap_a {
            let v1e = vcall0(obj, VT_PROBE);
            eax = v1e;
            bl = if v1e as u8 != 0 { 1 } else { 0 };
        } else {
            bl = 0;
        }
        if rd32(obj.wrapping_add(A70_OFF)) == 1 {
            return eax;
        }
        if lf_checker_rt::global::<u8>(MODE_GLOBAL).read() == 0 && bh == 0 {
            let f1: u32 = lf_checker_rt::callee_thiscall!(4, u32, obj);
            eax = f1;
            if f1 as u8 != 0 {
                let lk = rd32(obj.wrapping_add(LOOKUP_OFF)).wrapping_add(LOOKUP_ADD);
                let f2: u32 =
                    lf_checker_rt::callee_thiscall!(5, u32, lk, LOOKUP_PROBE, 0u32);
                eax = f2;
                if f2 as u8 != 0 {
                    wr32(this.wrapping_add(LATCH_OFF), 0);
                    eax = copy_pose(this, obj);
                    wrf(this.wrapping_add(ACC_OFF), 0.0);
                    return eax;
                }
            }
            if dist_gt {
                eax = copy_pose(this, obj);
                wrf(this.wrapping_add(ACC_OFF), 0.0);
                return eax;
            }
            if !(rdf(this.wrapping_add(ACC_OFF)) > frame0c) {
                // Joins the copy at 0x818, past the state-bit clear: no write.
                eax = copy_pose(this, obj);
                return eax;
            }
            if bl == 0 {
                return eax;
            }
            let e = rd32(obj.wrapping_add(CHAIN_OFF));
            eax = e;
            if e != 0 {
                eax = e.wrapping_add(CHAIN_ADD);
                if eax != 0 {
                    let parg = rd32(obj.wrapping_add(SRC_OFF)).wrapping_add(TRIPLE_OFF);
                    let g1: u32 = lf_checker_rt::callee_thiscall!(
                        6, u32, eax, parg, 2u32, 0u32, 1u32
                    );
                    eax = g1;
                }
            }
            eax = rd32(this.wrapping_add(OBJ_OFF));
            wr32(this.wrapping_add(STATE_OFF), m60 & !STATE_BIT);
            eax = copy_pose(this, eax);
            return eax;
        }
        wr32(this.wrapping_add(STATE_OFF), m60 & !STATE_BIT);
        eax = copy_pose(this, obj);
        eax
    }
});
