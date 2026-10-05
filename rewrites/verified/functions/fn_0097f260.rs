// original: 0x0097F260 PLAYER_BREATH

/// Drive the player-breath audio update for a ped task object.
///
/// `this` is the task object and `level` a float by bits. After the
/// shared audio-ready guards and a ped-activity byte at `+0x219`, three
/// speech-state callees (1-3, this = `[this+0x120]` biased by 0x570) must
/// all answer false. A scratch buffer (word 8 = `[this+8]`) and two
/// locals go through callee 5 and callee 6 (this = ped, locals and buffer
/// first); callee 6 answers an out-object and a name selector, and a null
/// out-object ends the call. The selected name goes to callee 7, then a
/// virtual call on the out-object (slot 0, global word as argument)
/// yields either null (the float update is skipped) or a float slot that
/// receives `1 - ((level - 1) / 3)` in the original's operation order.
/// Three more virtual-ish callees (9-11) run on the out-object, callee 12
/// posts the ped with the slot's float, callee 13 derives a parameter,
/// both land in the out-object at `+0xA4`/`+0xA8` (with `+0xAC` cleared),
/// and callee 14 finishes with three zero arguments. A null slot faults
/// at the float reload on both sides alike (same fault code).
///
/// Original: 0x0097F260 (thiscall, one stack argument, no return value;
/// the original aligns its stack frame, which the rewrite need not do).
lf_checker_rt::export!(thiscall, rw_0097F260(this: u32, level: u32) -> u32 {
    unsafe {
        const G_QUIT: u32 = 0x011F7060;
        const G_SESS_A: u32 = 0x012088B4;
        const G_SESS_B: u32 = 0x00F1C040;
        const G_MODE: u32 = 0x01037720;
        const G_VARG: u32 = 0x01231570;
        const SKIP_MODE: u32 = 0x12;
        const NAME_QUIET: u32 = 0x00E8CD34;
        const NAME_BREATH: u32 = 0x00E8CD48;
        const CONST_ONE: u32 = 0x00FE88E8;
        const CONST_THIRD: u32 = 0x00FE8808;
        const OFF_PED: u32 = 0x120;
        const OFF_SUB: u32 = 0x08;
        const PED_ACTIVE: u32 = 0x219;
        const PED_SPEECH_BIAS: u32 = 0x570;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn gget(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read() }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        if gget(G_QUIT) == 1 {
            return 0;
        }
        if gget(G_SESS_A) != gget(G_SESS_B) {
            return 0;
        }
        if gget(G_MODE) == SKIP_MODE {
            return 0;
        }
        let ped = rd32(this.wrapping_add(OFF_PED));
        if (ped.wrapping_add(PED_ACTIVE) as *const u8).read() == 0 {
            return 0;
        }
        let speech = ped.wrapping_add(PED_SPEECH_BIAS);
        let s1: u32 = lf_checker_rt::callee_thiscall!(1, u32, speech);
        if (s1 & 0xFF) != 0 {
            return 0;
        }
        let s2: u32 = lf_checker_rt::callee_thiscall!(2, u32, speech, 0, 0);
        if (s2 & 0xFF) != 0 {
            return 0;
        }
        let s3: u32 = lf_checker_rt::callee_thiscall!(3, u32, speech);
        if (s3 & 0xFF) != 0 {
            return 0;
        }
        let mut buf = [0u32; 16];
        let buf_ptr = buf.as_mut_ptr() as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, buf_ptr);
        let mut local_a = [0u32; 4];
        let local_a_ptr = local_a.as_mut_ptr() as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, this, 4, local_a_ptr);
        buf[8] = rd32(this.wrapping_add(OFF_SUB));
        let mut local_c = 0u32;
        let local_c_ptr = &mut local_c as *mut u32 as u32;
        let pick: u32 = lf_checker_rt::callee_thiscall!(
            6, u32, ped, local_c_ptr, buf_ptr, 0xFFFF_FFFF, 0, 0
        );
        let name = if (pick & 0xFF) == 0 { NAME_BREATH } else { NAME_QUIET };
        let _: u32 = lf_checker_rt::callee_thiscall!(
            7,
            u32,
            this,
            lf_checker_rt::relocated(name)
        );
        if local_c == 0 {
            return 0;
        }
        let slot_fn = rd32(rd32(local_c));
        let vf: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot_fn as usize);
        let slot = vf(local_c, gget(G_VARG));
        if slot != 0 {
            let one = f32::from_bits(rd32(lf_checker_rt::relocated(CONST_ONE)));
            let third = f32::from_bits(rd32(lf_checker_rt::relocated(CONST_THIRD)));
            let d = sub(f32::from_bits(level), one);
            let scaled = mul(d, third);
            let v = sub(one, scaled);
            (slot as *mut u32).write_unaligned(v.to_bits());
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(9, u32, local_c, 4);
        let _: u32 = lf_checker_rt::callee_thiscall!(10, u32, local_c, local_a_ptr);
        let _: u32 = lf_checker_rt::callee_thiscall!(11, u32, local_c, 1);
        let fbits = rd32(slot);
        let h: u32 = lf_checker_rt::callee_cdecl!(12, u32, fbits, ped);
        let p: u32 = lf_checker_rt::callee_cdecl!(13, u32, h);
        (local_c.wrapping_add(0xA4) as *mut u32).write_unaligned(h);
        (local_c.wrapping_add(0xA8) as *mut u32).write_unaligned(p);
        (local_c.wrapping_add(0xAC) as *mut u32).write_unaligned(0);
        let _: u32 = lf_checker_rt::callee_thiscall!(14, u32, local_c, 0, 0, 0);
        0
    }
});
