// original: 0x00997910 ROTOR_VOLUME

/// Update a helicopter's rotor audio from the vehicle's state (audio subsystem).
///
/// `this` is the audio object; `params` points at the caller's parameter
/// record. Reads vehicle state through the object at `this + 0x820`, a
/// run-time pointer table, several shared words and float constants, drives
/// eighteen stubbed callees (channel setup, equalizer, mixers, two name
/// lookups whose results select per-rotor level slots, and the neighbouring
/// voice-slot update), and writes the mixed levels back into `params` and
/// `this + 0xad4`.
///
/// Behaviour: returns early with a flag word when any of seven sub-object
/// slots is null. Otherwise resolves a table entry from a word of the
/// vehicle object, builds a scratch block through callees 1 and 2, combines
/// three of its floats with a 3x4 matrix plus translation (operand order as
/// below) and hands the result to callee 3. A mode word decides whether a
/// level slot is silenced. A second scratch block is processed through
/// callees 6 and 7 (the latter answers in the floating-point register);
/// three vehicle floats compared against zero select further slots. Seven
/// mixer calls and three level calls consume the parameter record and the
/// scratch; a second mode word can force two record fields to -100.0 and run
/// an extra setup call. Two name lookups ("ROTOR_VOLUME", "BLADE_VOLUME")
/// feed indirect calls whose non-null answers receive record fields. Ends by
/// calling the voice-slot update and storing one record field plus a scratch
/// word at `this + 0xad4`. Returns that call's answer.
///
/// All floating-point arithmetic keeps the original's operand order (pinned
/// with `black_box`); ordered comparisons reproduce the original's branch
/// conditions exactly, including unordered (NaN) inputs falling through.
/// Callee ids 4 and 10 receive the previous call's object in ECX on paper;
/// the original actually passes whatever its callee left there, which under
/// the checker is stub scratch on both sides, so those registers are
/// uncompared (see the contract). The same holds for the two indirect calls
/// and for the stack-block pointers, whose contents are compared instead.
lf_checker_rt::export!(thiscall, rw_00997910(this: u32, params: u32) -> u32 {
    unsafe {
        const G_SAVED: u32 = 0x011735b4;
        const G_REF: u32 = 0x0103234c;
        const G_ALT: u32 = 0x01038c8c;
        const G_MODE_A: u32 = 0x01168a58;
        const G_SIX: u32 = 0x01038c80;
        const G_LIMIT: u32 = 0x01038cb8;
        const G_MODE_B: u32 = 0x011f70cc;
        const K_ONE: u32 = 0x00fe88e8;
        const K_M100: u32 = 0x00fe8df8;
        const TBL: u32 = 0x01295cd8;
        const S_SLO: u32 = 0x00e8ff1c;
        const S_ROTOR: u32 = 0x00e8ff34;
        const S_BLADE: u32 = 0x00e8ff44;

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
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        let mut frame: [u32; 64] = [0; 64];
        let sptr = frame.as_mut_ptr() as u32;

        let saved = rd32(lf_checker_rt::relocated(G_SAVED));
        let one = rdf(lf_checker_rt::relocated(K_ONE));
        let flag: u32 = if one > rdf(lf_checker_rt::relocated(G_REF)) {
            rd32(lf_checker_rt::relocated(G_ALT))
        } else {
            0
        };
        if rd32(this.wrapping_add(0xa94)) == 0
            || rd32(this.wrapping_add(0xa90)) == 0
            || rd32(this.wrapping_add(0xa8c)) == 0
            || rd32(this.wrapping_add(0xa88)) == 0
            || rd32(this.wrapping_add(0xa98)) == 0
            || rd32(this.wrapping_add(0xa9c)) == 0
            || rd32(this.wrapping_add(0xaa0)) == 0
        {
            return flag;
        }

        let veh = rd32(this.wrapping_add(0x820));
        let idx = rd16(veh.wrapping_add(0x2e)) as i16 as i32;
        let p1 = rd32(lf_checker_rt::relocated(TBL).wrapping_add((idx << 2) as u32));
        let p2 = rd32(p1.wrapping_add(0xcc));
        let mut sel = rd32(p2.wrapping_add(0x16c));
        let r1: u32 = lf_checker_rt::callee_thiscall!(1, u32, veh);
        sel = sel.wrapping_shl(6).wrapping_add(rd32(r1.wrapping_add(0x10)));
        let s30 = sptr.wrapping_add(0x30);
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, s30, sel);
        let m = rd32(rd32(this.wrapping_add(0x820)).wrapping_add(0x20));
        let x5 = rdf(sptr.wrapping_add(0x64));
        let x2 = rdf(sptr.wrapping_add(0x60));
        let mut x0 = rdf(m);
        let mut x6 = rdf(m.wrapping_add(0x10));
        let x3 = rdf(sptr.wrapping_add(0x68));
        let mut x4 = rdf(m.wrapping_add(4));
        x0 = mul(x0, x2);
        let mut x1 = rdf(m.wrapping_add(8));
        x6 = mul(x6, x5);
        x4 = mul(x4, x2);
        x6 = add(x6, x0);
        x0 = mul(rdf(m.wrapping_add(0x20)), x3);
        let ch_a94 = rd32(this.wrapping_add(0xa94));
        x1 = mul(x1, x2);
        x6 = add(x6, x0);
        x0 = mul(rdf(m.wrapping_add(0x14)), x5);
        x6 = add(x6, rdf(m.wrapping_add(0x30)));
        x4 = add(x4, x0);
        x0 = mul(rdf(m.wrapping_add(0x24)), x3);
        x4 = add(x4, x0);
        x0 = mul(rdf(m.wrapping_add(0x18)), x5);
        x4 = add(x4, rdf(m.wrapping_add(0x34)));
        x1 = add(x1, x0);
        x0 = mul(rdf(m.wrapping_add(0x28)), x3);
        x1 = add(x1, x0);
        x0 = rdf(sptr.wrapping_add(0xcc));
        x1 = add(x1, rdf(m.wrapping_add(0x38)));
        wrf(sptr.wrapping_add(0x60), x6);
        wrf(sptr.wrapping_add(0x64), x4);
        wrf(sptr.wrapping_add(0x68), x1);
        wrf(sptr.wrapping_add(0x6c), x0);
        let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, ch_a94, sptr.wrapping_add(0x60));
        let m100 = rdf(lf_checker_rt::relocated(K_M100));
        wrf(sptr.wrapping_add(0x1c), m100);
        // ECX here is really the previous stub's scratch (uncompared); the
        // channel value below stands in for it.
        let r4: u32 = lf_checker_rt::callee_thiscall!(4, u32, ch_a94);
        let al = (r4 & 0xff) as u8;
        (sptr.wrapping_add(0x13) as *mut u8).write(al);
        wrf(sptr.wrapping_add(0x18), 0.0);
        if al != 0 {
            if rd8(this.wrapping_add(0xb3e)) != 0
                && rd32(lf_checker_rt::relocated(G_MODE_A)) == 2
            {
                wrf(sptr.wrapping_add(0x1c), 0.0);
            }
        } else {
            let c = rd32(rd32(this.wrapping_add(0x820)).wrapping_add(0xf50));
            if c != 0 {
                let r5: u32 = lf_checker_rt::callee_thiscall!(5, u32, c);
                if (r5 & 0xff) != 0 && rd32(lf_checker_rt::relocated(G_MODE_A)) == 2 {
                    wrf(sptr.wrapping_add(0x1c), 0.0);
                }
            }
        }

        let veh3 = rd32(this.wrapping_add(0x820));
        let low_gain = rd8(veh3.wrapping_add(0xf17)) & 8 == 0;
        let grp = this.wrapping_add(0x3d8);
        if low_gain {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                6, u32, grp, 0u32, 0x3e4ccccdu32, 0x3f4ccccdu32, 0x3f800000u32
            );
        } else {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                6, u32, grp, 0x3f800000u32, 0x3f800000u32, 0x3f800000u32, 0x3f800000u32
            );
        }
        let f7: f32 = lf_checker_rt::callee_thiscall!(7, f32, grp);
        wrf(sptr.wrapping_add(0x14), f7);
        let six = rdf(lf_checker_rt::relocated(G_SIX));
        let mut t1 = sub(one, rdf(params.wrapping_add(0x30)));
        let t2 = mul(six, f7);
        let t0 = sub(six, t2);
        wrf(sptr.wrapping_add(0x2c), t2);
        t1 = mul(t1, t0);
        let veh4 = rd32(this.wrapping_add(0x820));
        wrf(sptr.wrapping_add(0x20), t1);
        let mut x1b: f32 = 0.0;
        wrf(sptr.wrapping_add(0x14), 0.0);
        if 0.0 >= rdf(veh4.wrapping_add(0x1ef4)) {
            x1b = m100;
            wrf(sptr.wrapping_add(0x14), m100);
        }
        if 0.0 >= rdf(veh4.wrapping_add(0x1ef8)) {
            wrf(sptr.wrapping_add(0x18), m100);
        } else if 0.0 >= rdf(veh4.wrapping_add(0x1efc)) {
            wrf(sptr.wrapping_add(0x18), m100);
        }

        let _: u32 =
            lf_checker_rt::callee_thiscall!(8, u32, rd32(this.wrapping_add(0xa90)), x1b.to_bits());
        let lim_arg = rd32(params.wrapping_add(8)).wrapping_add(flag);
        let lim_arg = if rd32(lf_checker_rt::relocated(G_LIMIT)) as i32 > lim_arg as i32 {
            rd32(lf_checker_rt::relocated(G_LIMIT))
        } else {
            lim_arg
        };
        let _: u32 =
            lf_checker_rt::callee_thiscall!(9, u32, rd32(this.wrapping_add(0xa90)), lim_arg);
        let t = add(rdf(params.wrapping_add(0x20)), rdf(sptr.wrapping_add(0x18)));
        let _: u32 =
            lf_checker_rt::callee_thiscall!(8, u32, rd32(this.wrapping_add(0xa94)), t.to_bits());
        let t = add(rdf(params.wrapping_add(0x10)), rdf(sptr.wrapping_add(0x20)));
        let _: u32 =
            lf_checker_rt::callee_thiscall!(8, u32, rd32(this.wrapping_add(0xa8c)), t.to_bits());
        let _: u32 =
            lf_checker_rt::callee_thiscall!(9, u32, rd32(this.wrapping_add(0xa8c)), rd32(params.wrapping_add(0x14)));
        let t = rdf(params.wrapping_add(0x18));
        let _: u32 =
            lf_checker_rt::callee_thiscall!(8, u32, rd32(this.wrapping_add(0xa88)), t.to_bits());
        let _: u32 =
            lf_checker_rt::callee_thiscall!(9, u32, rd32(this.wrapping_add(0xa88)), rd32(params.wrapping_add(0x1c)));
        let t = add(rdf(params.wrapping_add(0x34)), rdf(sptr.wrapping_add(0x2c)));
        let _: u32 =
            lf_checker_rt::callee_thiscall!(8, u32, rd32(this.wrapping_add(0xa98)), t.to_bits());
        let t = rdf(params.wrapping_add(0x34));
        let _: u32 =
            lf_checker_rt::callee_thiscall!(8, u32, rd32(this.wrapping_add(0xa9c)), t.to_bits());
        let t = rdf(sptr.wrapping_add(0x1c));
        let ch_aa0 = rd32(this.wrapping_add(0xaa0));
        let _: u32 = lf_checker_rt::callee_thiscall!(8, u32, ch_aa0, t.to_bits());
        // ECX here is really the previous stub's scratch (uncompared).
        let r10: u32 = lf_checker_rt::callee_thiscall!(10, u32, ch_aa0);
        if (r10 & 0xff) != 0 && rd32(lf_checker_rt::relocated(G_MODE_B)) == 4 {
            wr32(params.wrapping_add(0xc), 0xc2c80000);
            wr32(params.wrapping_add(4), 0xc2c80000);
            let slot = this.wrapping_add(0xaa8);
            if rd32(this.wrapping_add(0xaa8)) == 0 {
                let s78 = sptr.wrapping_add(0x78);
                let _: u32 = lf_checker_rt::callee_thiscall!(11, u32, s78);
                wr32(
                    sptr.wrapping_add(0x84),
                    rd32(this.wrapping_add(0x820)).wrapping_add(0xdb8),
                );
                wr32(sptr.wrapping_add(0x78), 0x41100000);
                wr32(sptr.wrapping_add(0xa4), 0);
                wr32(sptr.wrapping_add(0xb0), 4);
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    12, u32, this, lf_checker_rt::relocated(S_SLO), slot, sptr.wrapping_add(0x78),
                    0xffffffffu32, 0u32, 0u32
                );
            }
            let c3 = rd32(slot);
            if c3 != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(13, u32, c3, 0u32);
            }
        } else {
            let c4 = rd32(this.wrapping_add(0xaa8));
            if c4 != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(14, u32, c4, 0x64u32);
                let c5 = rd32(this.wrapping_add(0xaa8));
                let _: u32 = lf_checker_rt::callee_thiscall!(15, u32, c5, 0u32);
            }
        }

        let o1 = rd32(this.wrapping_add(0xa90));
        let r16a: u32 =
            lf_checker_rt::callee_cdecl!(16, u32, lf_checker_rt::relocated(S_ROTOR), 0u32);
        let f1: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(o1)) as usize);
        let v1 = f1(o1, r16a);
        if v1 != 0 {
            wr32(v1, rd32(params.wrapping_add(0xc)));
        }
        let o2 = rd32(this.wrapping_add(0xa90));
        let r16b: u32 =
            lf_checker_rt::callee_cdecl!(16, u32, lf_checker_rt::relocated(S_BLADE), 0u32);
        let f2: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(o2)) as usize);
        let v2 = f2(o2, r16b);
        if v2 != 0 {
            wr32(v2, rd32(params.wrapping_add(4)));
        }
        let r18: u32 = lf_checker_rt::callee_thiscall!(18, u32, this, saved, params);
        let fout = add(rdf(params.wrapping_add(0xc)), rdf(sptr.wrapping_add(0x14)));
        wrf(this.wrapping_add(0xad4), fout);
        r18
    }
});
