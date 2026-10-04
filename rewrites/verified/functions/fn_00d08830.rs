// original: 0x00D08830 CTaskComplexStandGuard::vf20 (symbols)
//
// Task tick for the "stand guard" complex task: count down a timer, then
// dispatch on the current subtask's state word to advance the guard post.
//
// Arguments (thiscall): `this` is the task, `ped` the ped it runs on. A
// positive float at this+0x74 is reduced by a global tick delta; while it
// stays positive the tick ends at the main dispatch, and once it reaches
// zero or below the subtask at this+8 must accept (flag bit 1 set, or its
// virtual slot 0x14 answering probe 1, which then sets flag bit 2) or the
// timer is reseeded and the tick ends at the main dispatch; an accepted
// subtask runs a ped check (helper 2 when the dword at ped+0xd68 is set)
// and ends the tick with 0. The main dispatch flags bit 0x10000000 at
// ped+0x29c and reads the subtask's state (virtual slot 0xc): anything but
// 0x11d returns the subtask. For 0x11d the sub-state (slot 0xc on the object
// at subtask+0x14, or 0xc8 when null) selects: 0x11b probes helper 5 and on
// success tags this+0x88 and returns helper 6's answer for 0x11a; 0x11a
// picks a radius from this+0x68/0x6c by the sign-extended low 3 bits of
// this+0x90 and queries helper 7 twice (whose third word the distance check
// below ignores); a true first answer, or a near-enough position (squared
// delta below a global limit, summed (dy^2+dx^2)+dz^2) followed by a true
// second answer, copies the ped's position into this+0x40..0x4c, requires
// helper 5, and returns helper 6's answer for 0x11a (0x3ae after a false
// second answer or a failed distance check, both via helper 8); otherwise
// the old position is committed to this+0x50..0x5c and the subtask is
// returned. Sub-state 0x386 resolves an anchor through helper 10 and
// equalises two vectors to (picked float, 1.0, ~0.02) unless already equal
// (NaN counts as unequal); above 0x386 helper 11 must find and helper 12
// must confirm (answer 2) before the mode byte at this+0x90 is consulted:
// a word at this+0x86 above 5 selects helper 13 over two words from
// this+0x7c..0x82 (helper 6 for 0x11a after), 6 selects a reset (helper 9,
// helper 6 for 0x3ae after), and 5 or below selects helper 8 (helper 6 for
// 0x3ae after, with a helper 9 reset first when helper 8 declines); each
// path returns helper 6's answer. Float operation order is the original's.
lf_checker_rt::export!(thiscall, rw_00D08830(this: u32, ped: u32) -> u32 {
    unsafe {
        const SUB_AT: u32 = 0x08;
        const SUB_FLAG: u32 = 0x0c;
        const SUB_ACCEPT_SLOT: u32 = 0x14;
        const SUB_SUB: u32 = 0x14;
        const SUB_STATE_SLOT: u32 = 0x0c;
        const TIMER_AT: u32 = 0x74;
        const TIMER_RESEED: u32 = 0x38d1b717;
        const TAG_AT: u32 = 0x88;
        const MODE_AT: u32 = 0x90;
        const ALT_AT: u32 = 0x94;
        const POS_AT: u32 = 0x40;
        const OLD_AT: u32 = 0x50;
        const VEC0_AT: u32 = 0x60;
        const VEC1_AT: u32 = 0x64;
        const RAD0_AT: u32 = 0x68;
        const RAD1_AT: u32 = 0x6c;
        const SEL_AT: u32 = 0x86;
        const RES_AT: u32 = 0x78;
        const W0A_AT: u32 = 0x7c;
        const W0B_AT: u32 = 0x7e;
        const W1A_AT: u32 = 0x80;
        const W1B_AT: u32 = 0x82;
        const PED_POS: u32 = 0x20;
        const PED_BUSY: u32 = 0xd68;
        const PED_FLAG: u32 = 0x29c;
        const PED_INFO: u32 = 0x224;
        const ST_GUARD: u32 = 0x11d;
        const ST_WAIT: u32 = 0x11a;
        const ST_PROBE: u32 = 0x11b;
        const ST_ANCHOR: u32 = 0x386;
        const ST_ALERT: u32 = 0x3ae;
        const ST_NULL: u32 = 0xc8;
        const VEC_ONE: u32 = 0x3f800000;
        const VEC_SMALL: u32 = 0x3ca3d70a;
        const RAD_OVERRIDE: u32 = 0x40200000;
        const G_TICK: u32 = 0x11735bc;
        const G_NEAR: u32 = 0xfe8bb0;
        const G_VY: u32 = 0xfe88e8;
        const G_VZ: u32 = 0xfe8734;
        const C_SUB_ACCEPT: u32 = 1;
        const C_PED_CHECK: u32 = 2;
        const C_STATE: u32 = 3;
        const C_SUBSTATE: u32 = 4;
        const C_PROBE: u32 = 5;
        const C_ADVANCE: u32 = 6;
        const C_QUERY: u32 = 7;
        const C_BEGIN: u32 = 8;
        const C_RESET: u32 = 9;
        const C_ANCHOR: u32 = 10;
        const C_FIND: u32 = 11;
        const C_CONFIRM: u32 = 12;
        const C_PICK: u32 = 13;

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
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
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
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
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
        /// Virtual slot with no stack arguments on an object.
        unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
            unsafe {
                let addr = rd32(rd32(obj).wrapping_add(slot));
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(addr as usize);
                f(obj)
            }
        }
        /// The helper-5 probe with the shared (ped, 1, 0) arguments.
        unsafe fn probe(sub: u32, ped: u32) -> u32 {
            unsafe { lf_checker_rt::callee_thiscall!(C_PROBE, u32, sub, ped, 1, 0) }
        }
        /// Advance helper: tag this+0x88 and return helper 6's answer.
        unsafe fn advance(this: u32, ped: u32, tag: u16, arg: u32) -> u32 {
            unsafe {
                wr16(this.wrapping_add(TAG_AT), tag);
                lf_checker_rt::callee_thiscall!(C_ADVANCE, u32, this, arg, ped)
            }
        }

        let sub_a = rd32(this.wrapping_add(SUB_AT));
        // Timer gate.
        let old = rdf(this.wrapping_add(TIMER_AT));
        if old > 0.0 {
            let tick = f32::from_bits(rd32(lf_checker_rt::relocated(G_TICK)));
            let new = sub(old, tick);
            wrf(this.wrapping_add(TIMER_AT), new);
            if new <= 0.0 {
                // Timer ran out: the subtask must accept.
                let mut accepted = rd8(sub_a.wrapping_add(SUB_FLAG)) & 1 != 0;
                if !accepted {
                    let slot = rd32(rd32(sub_a).wrapping_add(SUB_ACCEPT_SLOT));
                    let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                        core::mem::transmute(slot as usize);
                    if f(sub_a, ped, 1, 0) as u8 != 0 {
                        wr8(sub_a.wrapping_add(SUB_FLAG), rd8(sub_a.wrapping_add(SUB_FLAG)) | 2);
                        accepted = true;
                    }
                }
                if !accepted {
                    wr32(this.wrapping_add(TIMER_AT), TIMER_RESEED);
                } else {
                    if rd32(ped.wrapping_add(PED_BUSY)) != 0 {
                        let _: u32 = lf_checker_rt::callee_thiscall!(C_PED_CHECK, u32, ped);
                    }
                    return 0;
                }
            }
        }
        // Main dispatch.
        wr32(ped.wrapping_add(PED_FLAG), rd32(ped.wrapping_add(PED_FLAG)) | 0x10000000);
        if vcall0(sub_a, SUB_STATE_SLOT) != ST_GUARD {
            return rd32(this.wrapping_add(SUB_AT));
        }
        let sub2 = rd32(sub_a.wrapping_add(SUB_SUB));
        let w = if sub2 == 0 { ST_NULL } else { vcall0(sub2, SUB_STATE_SLOT) } as i32;
        if w > ST_ANCHOR as i32 {
            // Alert path (only the exact alert state gets here).
            if w != ST_ALERT as i32 {
                return rd32(this.wrapping_add(SUB_AT));
            }
            let info = rd32(ped.wrapping_add(PED_INFO)).wrapping_add(0x44);
            let found: u32 = lf_checker_rt::callee_thiscall!(C_FIND, u32, info, 1, w as u32);
            if found != 0 {
                let conf: u32 = lf_checker_rt::callee_thiscall!(C_CONFIRM, u32, found);
                if conf != 2 {
                    return rd32(this.wrapping_add(SUB_AT));
                }
            }
            if rd8(this.wrapping_add(MODE_AT)) & 7 == 0 {
                return rd32(this.wrapping_add(SUB_AT));
            }
            let sel = rd16(this.wrapping_add(SEL_AT)) as i16 as i32;
            if probe(sub_a, ped) as u8 == 0 {
                return rd32(this.wrapping_add(SUB_AT));
            }
            if sel > 5 {
                if sel != 6 {
                    let (a0, a1) = if rd8(this.wrapping_add(ALT_AT)) & 1 != 0 {
                        (
                            rd16(this.wrapping_add(W1A_AT)) as i16 as i32 as u32,
                            rd16(this.wrapping_add(W1B_AT)) as i16 as i32 as u32,
                        )
                    } else {
                        (
                            rd16(this.wrapping_add(W0A_AT)) as i16 as i32 as u32,
                            rd16(this.wrapping_add(W0B_AT)) as i16 as i32 as u32,
                        )
                    };
                    let r: u32 = lf_checker_rt::callee_cdecl!(C_PICK, u32, a0, a1);
                    wr16(this.wrapping_add(RES_AT), (r & 0xFFFF) as u16);
                    return advance(this, ped, ST_ALERT as u16, ST_WAIT);
                }
                let _: u32 = lf_checker_rt::callee_thiscall!(C_RESET, u32, this, ped);
                return advance(this, ped, ST_ALERT as u16, ST_ALERT);
            }
            let b: u32 = lf_checker_rt::callee_thiscall!(C_BEGIN, u32, this, ped);
            if (b as u8) == 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(C_RESET, u32, this, ped);
            }
            return advance(this, ped, ST_ALERT as u16, ST_ALERT);
        }
        if w == ST_ANCHOR as i32 {
            // Anchor path: equalise two vectors unless already equal.
            let pick_vec = |this: u32| -> f32 {
                unsafe {
                    if rd8(this.wrapping_add(MODE_AT)) & 7 != 0
                        && rd8(this.wrapping_add(ALT_AT)) & 1 == 0
                    {
                        rdf(this.wrapping_add(VEC1_AT))
                    } else {
                        rdf(this.wrapping_add(VEC0_AT))
                    }
                }
            };
            let vy = f32::from_bits(rd32(lf_checker_rt::relocated(G_VY)));
            let vz = f32::from_bits(rd32(lf_checker_rt::relocated(G_VZ)));
            let anchored: u32 = lf_checker_rt::callee_thiscall!(C_ANCHOR, u32, sub_a, ped);
            if anchored != 0 {
                let pick = pick_vec(this);
                if !(rdf(anchored.wrapping_add(0x20)) == pick
                    && rdf(anchored.wrapping_add(0x24)) == vy
                    && rdf(anchored.wrapping_add(0x28)) == vz)
                {
                    wr32(anchored.wrapping_add(0x28), VEC_SMALL);
                    wr32(anchored.wrapping_add(0x24), VEC_ONE);
                    wrf(anchored.wrapping_add(0x20), pick);
                }
            }
            let pick = pick_vec(this);
            if !(rdf(sub2.wrapping_add(0x20)) == pick
                && rdf(sub2.wrapping_add(0x24)) == vy
                && rdf(sub2.wrapping_add(0x28)) == vz)
            {
                wrf(sub2.wrapping_add(0x20), pick);
                wr32(sub2.wrapping_add(0x24), VEC_ONE);
                wr32(sub2.wrapping_add(0x28), VEC_SMALL);
            }
            return rd32(this.wrapping_add(SUB_AT));
        }
        let e = (w as u32).wrapping_sub(ST_WAIT);
        if e == 0 {
            // Wait path.
            let sel = ((rd32(this.wrapping_add(MODE_AT)) << 29) as i32 >> 29) as i32;
            let rad = if sel == 1 || sel == 2 {
                rdf(this.wrapping_add(RAD1_AT))
            } else {
                rdf(this.wrapping_add(RAD0_AT))
            };
            let q1: u32 = lf_checker_rt::callee_thiscall!(
                C_QUERY, u32, this, this.wrapping_add(POS_AT), rad.to_bits(), ped
            );
            if (q1 as u8) == 0 {
                let dx = sub(rdf(this.wrapping_add(0x40)), rdf(this.wrapping_add(0x50)));
                let dy = sub(rdf(this.wrapping_add(0x44)), rdf(this.wrapping_add(0x54)));
                let dz = sub(rdf(this.wrapping_add(0x48)), rdf(this.wrapping_add(0x58)));
                let dd = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                let near = f32::from_bits(rd32(lf_checker_rt::relocated(G_NEAR)));
                if near >= dd {
                    let q2: u32 = lf_checker_rt::callee_thiscall!(
                        C_QUERY, u32, this, this.wrapping_add(POS_AT), RAD_OVERRIDE, ped
                    );
                    if (q2 as u8) != 0 {
                        let m = rd32(ped.wrapping_add(PED_POS));
                        wr32(this.wrapping_add(0x40), rd32(m.wrapping_add(0x30)));
                        wrf(this.wrapping_add(0x44), rdf(m.wrapping_add(0x34)));
                        wrf(this.wrapping_add(0x48), rdf(m.wrapping_add(0x38)));
                        wr32(this.wrapping_add(0x4c), rd32(m.wrapping_add(0x3c)));
                        if probe(sub_a, ped) as u8 != 0 {
                            let _: u32 =
                                lf_checker_rt::callee_thiscall!(C_BEGIN, u32, this, ped);
                            return advance(this, ped, ST_WAIT as u16, ST_WAIT);
                        }
                    } else if probe(sub_a, ped) as u8 != 0 {
                        let _: u32 =
                            lf_checker_rt::callee_thiscall!(C_BEGIN, u32, this, ped);
                        return advance(this, ped, ST_WAIT as u16, ST_ALERT);
                    }
                }
                // Fail path: straight to the advance, past the helper-8 call.
                if probe(sub_a, ped) as u8 != 0 {
                    return advance(this, ped, ST_WAIT as u16, ST_ALERT);
                }
            }
            // Commit the current position as the old one.
            wr32(this.wrapping_add(0x50), rd32(this.wrapping_add(0x40)));
            wrf(this.wrapping_add(0x54), rdf(this.wrapping_add(0x44)));
            wrf(this.wrapping_add(0x58), rdf(this.wrapping_add(0x48)));
            wr32(this.wrapping_add(0x5c), rd32(this.wrapping_add(0x4c)));
            return rd32(this.wrapping_add(SUB_AT));
        }
        if e.wrapping_sub(1) != 0 {
            return rd32(this.wrapping_add(SUB_AT));
        }
        // Probe path (w == 0x11b).
        if probe(sub_a, ped) as u8 == 0 {
            return rd32(this.wrapping_add(SUB_AT));
        }
        advance(this, ped, ST_PROBE as u16, ST_WAIT)
    }
});
