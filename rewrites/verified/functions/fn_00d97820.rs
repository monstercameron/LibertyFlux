// original: 0x00d97820 CPedFormation_Arrowhead::vf2

/// Arrowhead-formation slot update: refresh the leader direction block and
/// lay out seven follower slots from the group's heading.
///
/// `this` is the formation object (flag word at `+0x14`, a float at `+0x8`,
/// direction block at `+0xc0`, seven 0x18-byte slots starting at `+0x20`).
/// `group_arg` points 8 bytes below a group handle; callee 1 resolves it to
/// the group, whose position pointer (`+0x20`, floats at `+0x30/0x34/0x38`)
/// and heading (`+0xaa4`) drive the computation.
///
/// When flag bit 0 is set the heading goes through the cosine and sine
/// callees (float in and out of `xmm0`); otherwise the direction cosine is
/// 1 and the sine terms are 0. Two process-wide floats lazily capture the
/// `+0x8` value on first use (flag bits 0 and 1 of the shared flag word say
/// which are already stored). The loop then polls the group once per slot
/// (callee 4, skipped when it answers 0) and blends position, heading
/// trigonometry and the shared floats into three coordinates per slot with
/// the original's exact operation order. Flag bits 1 and 2 select one of
/// two four-float calls to the angle callee (callee 5, result on the x87
/// stack) with swapped argument order, or a group call (callee 6, also x87);
/// a wrapping callee (callee 7, x87) normalises that result into the slot's
/// fourth word. Always returns 1 (in `al`).
///
/// Original: 0x00d97820 (thiscall, one stack word, callee pops 4).
lf_checker_rt::export!(thiscall, rw_00d97820(this: u32, group_arg: u32) -> u32 {
    unsafe {
        const FORM_FLAGS: u32 = 0x14;
        const FORM_FLOAT8: u32 = 0x08;
        const FORM_DIR: u32 = 0xc0;
        const FORM_SLOTS: u32 = 0x20;
        const SLOT_STRIDE: u32 = 0x18;
        const GROUP_POS: u32 = 0x20;
        const GROUP_HEADING: u32 = 0xaa4;
        const POS_X: u32 = 0x30;
        const POS_Y: u32 = 0x34;
        const POS_Z: u32 = 0x38;
        const SHARED_A: u32 = 0x017A390C;
        const SHARED_FLAG: u32 = 0x017A3910;
        const SHARED_B: u32 = 0x017A3914;
        const ONE: f32 = 1.0;
        const HALF: f32 = 0.5;
        const ZERO: f32 = 0.0;
        const SIGN: u32 = 0x8000_0000;
        const ID_GET_GROUP: u32 = 1;
        const ID_COS: u32 = 2;
        const ID_SIN: u32 = 3;
        const ID_POLL: u32 = 4;
        const ID_ANGLE: u32 = 5;
        const ID_ALT: u32 = 6;
        const ID_WRAP: u32 = 7;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
            unsafe { wr32(a, v.to_bits()) }
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

        let group_ref = group_arg.wrapping_add(8);
        let group: u32 = lf_checker_rt::callee_thiscall!(ID_GET_GROUP, u32, group_ref);
        let flags = rd32(this.wrapping_add(FORM_FLAGS));
        let pos = rd32(group.wrapping_add(GROUP_POS));
        let pos_x = rdf(pos.wrapping_add(POS_X));
        let pos_y = rdf(pos.wrapping_add(POS_Y));
        let pos_z = rdf(pos.wrapping_add(POS_Z));
        let heading = rdf(group.wrapping_add(GROUP_HEADING));
        let (cos_v, sin_v, neg_sin_v) = if flags & 1 != 0 {
            // f32xmm0 stubs answer in xmm0 and mirror the bits in eax; the
            // Rust side reads the eax mirror (an f32 return would read ST0).
            let c = f32::from_bits(lf_checker_rt::callee_cdecl!(ID_COS, u32, heading.to_bits()));
            let s = f32::from_bits(lf_checker_rt::callee_cdecl!(ID_SIN, u32, heading.to_bits()));
            (c, s, f32::from_bits(s.to_bits() ^ SIGN))
        } else {
            (ONE, ZERO, ZERO)
        };
        wrf(this.wrapping_add(FORM_DIR), pos_x);
        wrf(this.wrapping_add(FORM_DIR + 4), pos_y);
        wrf(this.wrapping_add(FORM_DIR + 8), pos_z);
        wrf(this.wrapping_add(FORM_DIR + 12), heading);
        wr32(this.wrapping_add(FORM_DIR + 16), ONE.to_bits());
        let g_a = lf_checker_rt::global::<u32>(SHARED_A);
        let g_flag = lf_checker_rt::global::<u32>(SHARED_FLAG);
        let g_b = lf_checker_rt::global::<u32>(SHARED_B);
        let form8 = rdf(this.wrapping_add(FORM_FLOAT8));
        let mut flag = g_flag.read_unaligned();
        if flag & 1 == 0 {
            flag |= 1;
            g_flag.write_unaligned(flag);
            g_a.write_unaligned(form8.to_bits());
        }
        let scale_base: f32;
        if flag & 2 == 0 {
            flag |= 2;
            g_flag.write_unaligned(flag);
            g_b.write_unaligned(form8.to_bits());
            scale_base = form8;
        } else {
            scale_base = f32::from_bits(g_b.read_unaligned());
        }
        // Shared floats are never written inside the loop; read once.
        let g_a_v = f32::from_bits(g_a.read_unaligned());
        let g_b_v = f32::from_bits(g_b.read_unaligned());
        let scale = mul(scale_base, HALF);
        let t1 = mul(cos_v, scale);
        let t2 = mul(sin_v, scale);
        let t0 = mul(scale, ZERO);
        let mut k0: u32 = 1;
        let mut k1: u32 = 3;
        let mut count: u32 = 0;
        let mut slot = this.wrapping_add(FORM_SLOTS);
        for index in 0..7u32 {
            let poll: u32 = lf_checker_rt::callee_thiscall!(ID_POLL, u32, group_ref, index);
            if poll != 0 {
                let k0a = mul(k0 as f32, g_a_v);
                let v1 = mul(cos_v, k0a);
                let v0 = mul(neg_sin_v, k0a);
                let t = mul(k0a, ZERO);
                let mut v6 = sub(pos_y, v1);
                let u1 = mul(cos_v, g_a_v);
                let mut v7 = sub(pos_z, t);
                let u2 = mul(sin_v, g_a_v);
                let g_a0 = mul(g_a_v, ZERO);
                let mut v5 = sub(pos_x, v0);
                let f_k1_half = mul(k1 as f32, HALF);
                let w0 = mul(f_k1_half, u1);
                let w1 = mul(f_k1_half, u2);
                let w3 = mul(f_k1_half, g_a0);
                v6 = sub(v6, w1);
                v5 = sub(v5, w0);
                v7 = sub(v7, w3);
                let u1b = mul(cos_v, g_b_v);
                let u0b = mul(sin_v, g_b_v);
                v7 = add(v7, t0);
                let g_b0 = mul(g_b_v, ZERO);
                let f_count = count as f32;
                let v4 = add(t1, v5);
                let v5b = add(t2, v6);
                let out0 = add(mul(f_count, u1b), v4);
                let out1 = add(mul(f_count, u0b), v5b);
                let out2 = add(mul(f_count, g_b0), v7);
                wrf(slot.wrapping_sub(8), out0);
                wrf(slot.wrapping_sub(4), out1);
                wrf(slot, out2);
                wr32(slot.wrapping_add(8), ONE.to_bits());
                let branch_val: f32;
                if (flags >> 1) & 1 != 0 {
                    branch_val = lf_checker_rt::callee_cdecl!(
                        ID_ANGLE, f32,
                        pos_x.to_bits(), pos_y.to_bits(), out0.to_bits(), out1.to_bits()
                    );
                } else if (flags >> 2) & 1 != 0 {
                    branch_val = lf_checker_rt::callee_cdecl!(
                        ID_ANGLE, f32,
                        out0.to_bits(), out1.to_bits(), pos_x.to_bits(), pos_y.to_bits()
                    );
                } else {
                    branch_val = lf_checker_rt::callee_thiscall!(ID_ALT, f32, group);
                }
                let wrap_val: f32 =
                    lf_checker_rt::callee_cdecl!(ID_WRAP, f32, branch_val.to_bits());
                count += 1;
                wrf(slot.wrapping_add(4), wrap_val);
                if count == k1 {
                    k0 += 1;
                    k1 += 2;
                    count = 0;
                }
            }
            slot = slot.wrapping_add(SLOT_STRIDE);
        }
        1
    }
});
