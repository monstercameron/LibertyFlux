// original: 0x00D08470 CTaskComplexSeperate::vf20 (symbols)
//
// Task tick for the "separate" complex task: keep two tracked objects apart
// by steering the ped toward a computed direction, or tear the task down.
//
// Arguments (thiscall): `this` is the task, `ped` the ped it runs on. A null
// word at this+0xac ends the tick with 0. Flag bit 2 at this+0xb8 selects a
// refresh pair (helpers 1 and 2). Two virtual slot-0xc probes (one per
// tracked object at this+8 and this+0xb0) must agree, else the current
// subtask is returned. A nonzero mode byte at this+0x98 runs a five-argument
// query (helper 5) through three scratch slots; an answer of 1 clears the
// counter at this+0x94, and the tick then returns the subtask. Otherwise a
// counter below 8 runs the steering attempt: helper 6 fills a target point,
// helper 7 must accept, and the delta from the ped's position is normalized
// by helper 8 (whose input vector is part of the comparison) and dotted
// against two matrix rows with two global thresholds to pick a direction
// (2, 1, 0 or -1). A full gate chain (kind word 2, tag word matching a
// global, a scaled helper-9 value under a global limit, direction valid)
// ends through helpers 10 and 11, returning helper 11's answer. Any gate
// failure falls back to a ten-argument construction (helper 12) plus a
// virtual slot-4 query whose two argument words stay on the stack for the
// final four-argument call (helper 13), whose answer is returned; a null
// helper-10 answer on the way returns 0. A counter of 8 or more instead runs
// the teardown path (flag bit 4 at this+0xb8, counter reset, threshold bit
// at this+0xbc, helper 14), then helper 7 decides between returning 0 and
// returning the subtask. A failed steering attempt decrements the counter
// and rejoins the counter check. Float operation order is the original's.
lf_checker_rt::export!(thiscall, rw_00D08470(this: u32, ped: u32) -> u32 {
    unsafe {
        const SUB_A: u32 = 0x08;
        const VT_SLOT_C: u32 = 0x0c;
        const VT_SLOT_Q: u32 = 0x04;
        const AC_AT: u32 = 0xac;
        const SUB_B: u32 = 0xb0;
        const FLAG: u32 = 0xb8;
        const MODE: u32 = 0x98;
        const COUNT: u32 = 0x94;
        const ARG_SRC: u32 = 0x90;
        const VEC_SRC: u32 = 0x30;
        const THRESH: u32 = 0xb4;
        const EXTRA: u32 = 0xbc;
        const COUNT_LIVE: i32 = 8;
        const PED_MTX: u32 = 0x20;
        const PED_TAG: u32 = 0x2e;
        const PED_KIND: u32 = 0x21c;
        const KIND_OFF: u32 = 0x12c;
        const KIND_WANTED: u32 = 2;
        const G_HI: u32 = 0xfe88bc;
        const G_LO: u32 = 0xe9d14c;
        const G_F1: u32 = 0xee1eb4;
        const G_F2: u32 = 0xee1eb0;
        const G_LIM: u32 = 0xfe8800;
        const G_FAC: u32 = 0xfe8684;
        const G_WORD: u32 = 0x12fa650;
        const G_POOL: u32 = 0x167e2a0;
        const C_REF1: u32 = 1;
        const C_REF2: u32 = 2;
        const C_PROBE_A: u32 = 3;
        const C_PROBE_B: u32 = 4;
        const C_QUERY: u32 = 5;
        const C_TARGET: u32 = 6;
        const C_ACCEPT: u32 = 7;
        const C_NORM: u32 = 8;
        const C_SCALE: u32 = 9;
        const C_POOL: u32 = 10;
        const C_STEER: u32 = 11;
        const C_BUILD: u32 = 12;
        const C_FINISH: u32 = 13;
        const C_DOWN: u32 = 14;
        const C_QRY2: u32 = 15;
        const BUILD_TAG: u32 = 0x4016d0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
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
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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
        /// Dot a matrix row (three consecutive floats at `row`) against the
        /// direction (vx, vy, vz) in the original's order.
        unsafe fn dot3(m: u32, row: u32, vx: f32, vy: f32, vz: f32) -> f32 {
            unsafe {
                add(
                    add(mul(rdf(m.wrapping_add(row + 4)), vy), mul(rdf(m.wrapping_add(row)), vx)),
                    mul(rdf(m.wrapping_add(row + 8)), vz),
                )
            }
        }
        /// Pick the steering direction from the matrix and the normalized
        /// delta: 2 when the second row exceeds the high threshold or falls
        /// below the low one, 1 when the first row exceeds the high one,
        /// else 0 when the first row falls below the low one, else -1.
        unsafe fn direction(m: u32, vx: f32, vy: f32, vz: f32, g6: f32, g5: f32) -> u32 {
            unsafe {
                if dot3(m, 0x10, vx, vy, vz) > g6 {
                    return 2;
                }
                if g5 > dot3(m, 0x10, vx, vy, vz) {
                    return 2;
                }
                if dot3(m, 0x00, vx, vy, vz) > g6 {
                    return 1;
                }
                if g5 > dot3(m, 0x00, vx, vy, vz) {
                    0
                } else {
                    0xFFFF_FFFF
                }
            }
        }
        /// Virtual slot with no stack arguments on a tracked object.
        unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
            unsafe {
                let addr = rd32(rd32(obj).wrapping_add(slot));
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(addr as usize);
                f(obj)
            }
        }
        /// The counter>=8 teardown path; returns the tick result.
        unsafe fn count_path(this: u32, ped: u32, sub_a: u32) -> u32 {
            unsafe {
                if rd8(this.wrapping_add(FLAG)) & 4 == 0 {
                    wr32(this.wrapping_add(COUNT), 0);
                    if rd32(this.wrapping_add(THRESH)) as i32 > 5 {
                        wr32(this.wrapping_add(EXTRA), rd32(this.wrapping_add(EXTRA)) | 1);
                    }
                    let _: u32 = lf_checker_rt::callee_thiscall!(C_DOWN, u32, this, ped, 1);
                    wr8(this.wrapping_add(FLAG), rd8(this.wrapping_add(FLAG)) | 4);
                }
                if rd8(this.wrapping_add(MODE)) != 0 {
                    return rd32(this.wrapping_add(SUB_A));
                }
                let e: u32 = lf_checker_rt::callee_thiscall!(C_ACCEPT, u32, sub_a, ped, 1, 0);
                if (e as u8) != 0 {
                    return 0;
                }
                rd32(this.wrapping_add(SUB_A))
            }
        }

        if rd32(this.wrapping_add(AC_AT)) == 0 {
            return 0;
        }
        if rd8(this.wrapping_add(FLAG)) & 2 != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(C_REF1, u32, ped, 1);
            let inner = rd32(rd32(this.wrapping_add(AC_AT)).wrapping_add(PED_MTX));
            let _: u32 =
                lf_checker_rt::callee_thiscall!(C_REF2, u32, ped, inner.wrapping_add(0x30));
        }
        let sub_a = rd32(this.wrapping_add(SUB_A));
        let sub_b = rd32(this.wrapping_add(SUB_B));
        let v0 = vcall0(sub_a, VT_SLOT_C);
        let v1 = vcall0(sub_b, VT_SLOT_C);
        if v0 != v1 {
            return rd32(this.wrapping_add(SUB_A));
        }
        if rd8(this.wrapping_add(MODE)) != 0 {
            wr32(this.wrapping_add(ARG_SRC), 5);
            let mut dcout = 0u32;
            let mut dummy1 = 0u32;
            let mut dummy2 = 0u32;
            let ok: u32 = lf_checker_rt::callee_thiscall!(
                C_QUERY, u32, this.wrapping_add(MODE),
                &mut dcout as *mut u32 as u32,
                &mut dummy1 as *mut u32 as u32,
                &mut dummy2 as *mut u32 as u32,
                this.wrapping_add(VEC_SRC),
                this.wrapping_add(ARG_SRC)
            );
            if (ok as u8) != 0 && dcout == 1 {
                wr32(this.wrapping_add(COUNT), 0);
            }
            if rd8(this.wrapping_add(MODE)) != 0 {
                return rd32(this.wrapping_add(SUB_A));
            }
        }
        if rd32(this.wrapping_add(COUNT)) as i32 >= COUNT_LIVE {
            return count_path(this, ped, sub_a);
        }
        // Steering attempt.
        let mut outvec = [0u32; 3];
        let ok: u32 = lf_checker_rt::callee_thiscall!(
            C_TARGET, u32, this, outvec.as_mut_ptr() as u32, ped
        );
        if (ok as u8) == 0 {
            if rd32(this.wrapping_add(COUNT)) as i32 >= COUNT_LIVE {
                return count_path(this, ped, sub_a);
            }
            return rd32(this.wrapping_add(SUB_A));
        }
        let e: u32 = lf_checker_rt::callee_thiscall!(C_ACCEPT, u32, sub_a, ped, 1, 0);
        if (e as u8) == 0 {
            wr32(
                this.wrapping_add(COUNT),
                rd32(this.wrapping_add(COUNT)).wrapping_sub(1),
            );
            if rd32(this.wrapping_add(COUNT)) as i32 >= COUNT_LIVE {
                return count_path(this, ped, sub_a);
            }
            return rd32(this.wrapping_add(SUB_A));
        }
        let m = rd32(ped.wrapping_add(PED_MTX));
        let dx = sub(f32::from_bits(outvec[0]), rdf(m.wrapping_add(0x30)));
        let dy = sub(f32::from_bits(outvec[1]), rdf(m.wrapping_add(0x34)));
        let mut buf = [dx.to_bits(), dy.to_bits(), 0u32];
        let _: u32 = lf_checker_rt::callee_thiscall!(C_NORM, u32, buf.as_mut_ptr() as u32);
        let vx = f32::from_bits(buf[0]);
        let vy = f32::from_bits(buf[1]);
        let vz = f32::from_bits(buf[2]);
        let g6 = f32::from_bits(rd32(lf_checker_rt::relocated(G_HI)));
        let g5 = f32::from_bits(rd32(lf_checker_rt::relocated(G_LO)));
        let mut s8 = direction(m, vx, vy, vz, g6, g5);
        // Gate chain toward the steer call.
        let mut skip = false;
        let ko = rd32(ped.wrapping_add(PED_KIND));
        if rd32(ko.wrapping_add(KIND_OFF)) != KIND_WANTED {
            skip = true;
        }
        if !skip
            && rd16(ped.wrapping_add(PED_TAG)) as i16 as i32 as u32
                != rd32(lf_checker_rt::relocated(G_WORD))
        {
            skip = true;
        }
        if !skip {
            let n: u32 = lf_checker_rt::callee_cdecl!(C_SCALE, u32,);
            let lim = f32::from_bits(rd32(lf_checker_rt::relocated(G_LIM)));
            let fac = f32::from_bits(rd32(lf_checker_rt::relocated(G_FAC)));
            if !(lim > mul(n as i32 as f32, fac)) {
                skip = true;
            }
        }
        if !skip && s8 == 0xFFFF_FFFF {
            skip = true;
        }
        if !skip {
            let pool = rd32(lf_checker_rt::relocated(G_POOL));
            let h: u32 = lf_checker_rt::callee_thiscall!(C_POOL, u32, pool);
            if h == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(C_STEER, u32, h, s8);
        }
        // Fallback: build, query, finish.
        let pool = rd32(lf_checker_rt::relocated(G_POOL));
        let h: u32 = lf_checker_rt::callee_thiscall!(C_POOL, u32, pool);
        if h != 0 {
            let f1 = rd32(lf_checker_rt::relocated(G_F1));
            let f2 = rd32(lf_checker_rt::relocated(G_F2));
            let mut dummy = 0u32;
            s8 = lf_checker_rt::callee_thiscall!(
                C_BUILD, u32, h, 3, &mut dummy as *mut u32 as u32, f2, f1,
                0xFFFF_FFFF, 1, 0, lf_checker_rt::relocated(BUILD_TAG), 0, 1
            );
        } else {
            s8 = 0;
        }
        let h2: u32 = lf_checker_rt::callee_thiscall!(C_POOL, u32, pool);
        if h2 == 0 {
            return 0;
        }
        let slot = rd32(rd32(sub_b).wrapping_add(VT_SLOT_Q));
        let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        let i2ans: u32 = f(sub_b, 0, 0);
        lf_checker_rt::callee_thiscall!(C_FINISH, u32, h2, s8, i2ans, 0, 0)
    }
});
