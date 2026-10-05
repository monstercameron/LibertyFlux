// original: 0x00cb9d30 track_target_update (proposed)

/// Advance a tracking move task one update towards the entity it follows.
///
/// `this` is the task object (tracked-entity record at `+0x28`, speed at
/// `+0x54`, heading at `+0x50`, goal snapshot at `+0x18`/`+0x40`/`+0x44`,
/// flags at `+0x58`) and `ped` the pedestrian (entity at `+0x20`).
///
/// Behaviour: return 1 at once when no record is attached or the task is
/// suspended (flag bit 1). Otherwise run the init helper, look the record
/// up (selector 0xf) and, when found, default a zero speed to 1. Measure
/// the planar distance to the goal snapshot, hand it to the record's
/// direction helper (virtual slot 59) and blend the returned direction
/// with the record's frame and the entity position through a four-way
/// ladder that picks a new heading, with a fallthrough arm that calls the
/// arc helper instead. Clamp the result against the old heading and
/// speed, optionally run the mix/final helpers (which set the lateral
/// offset at `+0x20`), decay a positive speed by the frame step, copy the
/// heading to the goal snapshot and return 0.
///
/// The float operation order is the original's, pinned through `black_box`
/// helpers; every branch is an ordered comparison (NaN takes the
/// fallthrough side). The record-construction block appears twice; the
/// second copy only runs when the first helper answers null, which would
/// fault downstream, so no trial takes it. Original: 0x00cb9d30
/// (thiscall, one stack word, low byte of the result).
lf_checker_rt::export!(thiscall, rw_00cb9d30(this: u32, ped: u32) -> u32 {
    unsafe {
        const RECORD: u32 = 0x28;
        const FLAGS: u32 = 0x58;
        const SUSPENDED: u8 = 0x02;
        const SPEED: u32 = 0x54;
        const HEADING: u32 = 0x50;
        const GOAL_SNAP: u32 = 0x18;
        const GOAL_X: u32 = 0x40;
        const GOAL_Y: u32 = 0x44;
        const GOAL_Z: u32 = 0x48;
        const LATERAL: u32 = 0x20;
        const PED_ENT: u32 = 0x20;
        const PED_LOOKUP: u32 = 0x78;
        const LOOKUP_SELECTOR: u32 = 0x0f;
        const REC_FRAME: u32 = 0x20;
        const REC_ATTACH: u32 = 0x10;
        const VT_DIR_SLOT: u32 = 0xec;
        const SIGN: u32 = 0x8000_0000;
        const ID_INIT: u32 = 1;
        const ID_LOOKUP: u32 = 2;
        const ID_CTOR: u32 = 4;
        const ID_ATTACH: u32 = 5;
        const ID_ARC: u32 = 6;
        const ID_MIX: u32 = 7;
        const ID_FINAL: u32 = 8;
        // File VAs of the frame-step global and the float constants.
        const G_STEP: u32 = 0x011735bc;
        const C_TWO: u32 = 0x00fe8a24; // 2.0
        const C_LADDER_A: u32 = 0x00fe8d7c; // -0.5
        const C_NEAR_LO: u32 = 0x00fe876c; // 0.05
        const C_FLOOR: u32 = 0x00fe87e8; // 0.3
        const C_LADDER_B: u32 = 0x00fe8d70; // -0.25
        const C_NEAR_HI: u32 = 0x00fe8734; // 0.02
        const C_CEIL: u32 = 0x00fe8a94; // 3.0
        const C_HEAD_STEP: u32 = 0x00fe87d0; // 0.2
        const C_MIX_LIM: u32 = 0x00fe8d94; // -1.0
        const C_ONE: u32 = 0x00fe88e8; // 1.0

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits(lf_checker_rt::global::<u32>(va).read_unaligned()) }
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
        fn neg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ SIGN)
        }

        if rd32(this + RECORD) == 0 {
            return 1;
        }
        if rd8(this + FLAGS) & SUSPENDED != 0 {
            return 1;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(ID_INIT, u32, this);
        let found: u32 =
            lf_checker_rt::callee_thiscall!(ID_LOOKUP, u32, rd32(ped + PED_LOOKUP), LOOKUP_SELECTOR);
        if found != 0 && rdf(this + SPEED) == 0.0 {
            wrf(this + SPEED, 1.0);
        }
        let ent = rd32(ped + PED_ENT);
        let dx0 = sub(rdf(this + GOAL_Y), rdf(ent + 0x34));
        let dx1 = sub(rdf(this + GOAL_X), rdf(ent + 0x30));
        let obj = rd32(this + RECORD);
        let dist = core::hint::black_box(add(mul(dx0, dx0), mul(dx1, dx1))).sqrt();
        let dir: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(obj) + VT_DIR_SLOT) as usize);
        // The address handed over was computed before the argument push
        // while the distance is stored after it, so the helper receives a
        // pointer to an unwritten frame slot (the zero fill), not to the
        // distance; the distance itself is read back near the end.
        let dist_slot = 0u32;
        let ret = dir(obj, &dist_slot as *const u32 as u32);
        let rq1 = rdf(ret + 4);
        let rq2 = rdf(ret);
        let rq0 = rdf(ret + 8);
        let n2 = add(
            add(mul(rq2, rq2), mul(rq1, rq1)),
            mul(rq0, rq0),
        );
        let reach = core::hint::black_box(n2).sqrt();
        let rec = rd32(this + RECORD);
        if rd32(rec + REC_FRAME) == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(ID_CTOR, u32, rec);
            let _: u32 =
                lf_checker_rt::callee_thiscall!(ID_ATTACH, u32, rec + REC_ATTACH, rd32(rec + REC_FRAME));
        }
        let frame = rd32(rec + REC_FRAME);
        let mut bx0 = rdf(this + GOAL_X);
        let mut bx1 = rdf(frame + 0x14);
        bx1 = mul(bx1, rdf(this + GOAL_Y));
        bx0 = mul(bx0, rdf(frame + 0x10));
        let rec2 = rd32(this + RECORD);
        if rd32(rec2 + REC_FRAME) == 0 {
            // Second copy of the construction block. It only runs when the
            // first helper answers null, which faults a few instructions
            // later, so no trial reaches it; kept for completeness.
            let _: u32 = lf_checker_rt::callee_thiscall!(ID_CTOR, u32, rec2);
            let _: u32 = lf_checker_rt::callee_thiscall!(
                ID_ATTACH,
                u32,
                rec2 + REC_ATTACH,
                rd32(rec2 + REC_FRAME)
            );
        }
        bx1 = add(bx1, bx0);
        let mut t0 = rdf(frame + 0x18);
        t0 = mul(t0, rdf(this + GOAL_Z));
        bx1 = add(bx1, t0);
        bx1 = neg(bx1);
        let mut fr = rd32(rec2 + REC_FRAME);
        let arg = ped;
        let mut cx0 = rdf(fr + 0x10);
        let ent2 = rd32(arg + PED_ENT);
        let mut cx2 = rdf(fr + 0x14);
        cx0 = mul(cx0, rdf(ent2 + 0x30));
        cx2 = mul(cx2, rdf(ent2 + 0x34));
        fr += 0x10;
        let two = gf(C_TWO);
        cx2 = add(cx2, cx0);
        cx0 = rdf(fr + 8);
        cx0 = mul(cx0, rdf(ent2 + 0x38));
        cx2 = add(cx2, cx0);
        cx0 = gf(C_LADDER_A);
        cx2 = add(cx2, bx1);
        bx1 = neg(cx2);
        let save_blend = cx2;
        // Four-way ladder over the blended alignment, then the arc-helper
        // fallthrough. Every condition is an ordered comparison.
        if cx0 > bx1 {
            let t = sub(rdf(this + GOAL_SNAP), gf(C_NEAR_LO));
            let floor = gf(C_FLOOR);
            wrf(this + HEADING, if floor > t { floor } else { t });
        } else if gf(C_LADDER_B) > bx1 {
            let t = sub(rdf(this + GOAL_SNAP), gf(C_NEAR_HI));
            let floor = gf(C_FLOOR);
            wrf(this + HEADING, if floor > t { floor } else { t });
        } else if bx1 > two {
            let t = add(rdf(this + GOAL_SNAP), gf(C_NEAR_LO));
            let ceil = gf(C_CEIL);
            wrf(this + HEADING, if t > ceil { ceil } else { t });
        } else if bx1 > gf(C_ONE) {
            let t = add(rdf(this + GOAL_SNAP), gf(C_NEAR_HI));
            let ceil = gf(C_CEIL);
            wrf(this + HEADING, if t > ceil { ceil } else { t });
        } else {
            let arc: f32 = lf_checker_rt::callee_cdecl!(ID_ARC, f32, arg, reach.to_bits());
            let floor = gf(C_FLOOR);
            let mut v1 = arc;
            if floor > v1 {
                v1 = floor;
            }
            let snap = rdf(this + GOAL_SNAP);
            if v1 > snap {
                let x0 = add(rdf(this + HEADING), gf(C_HEAD_STEP));
                wrf(this + HEADING, x0);
                if x0 > v1 {
                    wrf(this + HEADING, v1);
                }
            } else if snap > v1 {
                let x0 = sub(rdf(this + HEADING), gf(C_HEAD_STEP));
                wrf(this + HEADING, x0);
                if v1 > x0 {
                    wrf(this + HEADING, v1);
                }
            }
        }
        // The mix block runs when the blend is below -1, or else when
        // the planar distance exceeds 2.
        let run_mix = gf(C_MIX_LIM) > save_blend || dist > two;
        if run_mix {
            let t40 = rdf(this + GOAL_X);
            let t44 = rdf(this + GOAL_Y);
            let m: f32 = lf_checker_rt::callee_cdecl!(
                ID_MIX,
                f32,
                t40.to_bits(),
                t44.to_bits(),
                rdf(ent + 0x30).to_bits(),
                rdf(ent + 0x34).to_bits()
            );
            wrf(this + LATERAL, m);
            let f2: f32 = lf_checker_rt::callee_cdecl!(ID_FINAL, f32, m.to_bits());
            wrf(this + LATERAL, f2);
        }
        let speed = rdf(this + SPEED);
        if speed > 0.0 {
            wrf(this + HEADING, 1.0);
            wrf(this + SPEED, sub(speed, gf(G_STEP)));
        }
        wr32(this + GOAL_SNAP, rd32(this + HEADING));
        0
    }
});
