// original: 0x00d97be0 CPedFormation_FollowInLine::vf2

/// Follow-in-line formation slot update: maintain an eight-slot trail behind
/// the leader and lay out seven follower slots from it.
///
/// `this` is the formation object: a float at `+0x8`, a tag at `+0xd8`, the
/// last leader position at `+0xe0`, eight trail slots at `+0xf0` (0x10 bytes
/// each: x, y, z, spare), per-slot headings at `+0x170` and extra values at
/// `+0x190`, a direction block at `+0xc0`, and seven 0x18-byte output slots
/// starting at `+0x20`. `group_arg` points 8 bytes below a group handle;
/// callee 1 counts something on it and callee 2 resolves it to the group,
/// whose position pointer (`+0x20`, floats at `+0x30/0x34/0x38`) and heading
/// (`+0xaa4`) drive the computation.
///
/// If the squared distance from the last leader position exceeds 100 all
/// eight trail slots are re-seeded with the current position, the heading
/// and 6.0. If the (possibly re-seeded) distance then exceeds the squared
/// `+0x8` value the trail shifts one slot down, the current position becomes
/// the new head, and the head slot's heading comes from the angle callee
/// (callee 5) when the new planar distance exceeds 0.05, else the heading.
/// A virtual call through the group's table (slot `+0xec`, callee 3) returns
/// a 3-vector whose length later selects an averaging step. The main block
/// runs unless the distance word is exactly zero and the callee-1 answer
/// still matches the tag; it stores the direction block, derives a blend
/// factor from the saved planar delta, then polls the group once per output
/// slot (callee 4, skipped when it answers 0) and blends consecutive trail
/// slots with the original's exact operation order. Always returns 1 (in
/// `al`). The original reads one stack slot it never wrote (the trail spare
/// words and one padding word); the contract defines that fill as 0 and the
/// rewrite uses 0.
///
/// Original: 0x00d97be0 (thiscall, one stack word, callee pops 4).
lf_checker_rt::export!(thiscall, rw_00d97be0(this: u32, group_arg: u32) -> u32 {
    unsafe {
        const FORM_FLOAT8: u32 = 0x08;
        const FORM_TAG: u32 = 0xd8;
        const FORM_LAST: u32 = 0xe0;
        const FORM_SLOTS: u32 = 0xf0;
        const SLOT_STRIDE: u32 = 0x10;
        const FORM_HEADS: u32 = 0x170;
        const FORM_EXTRA: u32 = 0x190;
        const FORM_DIR: u32 = 0xc0;
        const FORM_OUT: u32 = 0x20;
        const OUT_STRIDE: u32 = 0x18;
        const GROUP_POS: u32 = 0x20;
        const GROUP_HEADING: u32 = 0xaa4;
        const POS_X: u32 = 0x30;
        const POS_Y: u32 = 0x34;
        const POS_Z: u32 = 0x38;
        const VTABLE_SLOT: u32 = 0xec;
        const ONE: f32 = 1.0;
        const ZERO: f32 = 0.0;
        const HALF: f32 = 0.5;
        const HUNDRED: f32 = 100.0;
        const MILLE: f32 = 0.001;
        const FIFTY_MILLI: f32 = 0.05;
        const TENTH: f32 = 0.1;
        const SIX: f32 = 6.0;
        const NEG_OFF: u32 = 0xffffff08;
        const ID_COUNT: u32 = 1;
        const ID_GET_GROUP: u32 = 2;
        const ID_VEC: u32 = 3;
        const ID_POLL: u32 = 4;
        const ID_ANGLE: u32 = 5;

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
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        let group_ref = group_arg.wrapping_add(8);
        let form8 = rdf(this.wrapping_add(FORM_FLOAT8));
        let form8_sq = mul(form8, form8);
        let count: u32 = lf_checker_rt::callee_thiscall!(ID_COUNT, u32, group_ref);
        let group: u32 = lf_checker_rt::callee_thiscall!(ID_GET_GROUP, u32, group_ref);
        let pos = rd32(group.wrapping_add(GROUP_POS));
        let heading = rdf(group.wrapping_add(GROUP_HEADING));
        let px = rdf(pos.wrapping_add(POS_X));
        let py = rdf(pos.wrapping_add(POS_Y));
        let pz = rdf(pos.wrapping_add(POS_Z));
        let dx = sub(px, rdf(this.wrapping_add(FORM_LAST)));
        let dy = sub(py, rdf(this.wrapping_add(FORM_LAST + 4)));
        let dz = sub(pz, rdf(this.wrapping_add(FORM_LAST + 8)));
        let mut dist2 = add(mul(dy, dy), mul(dx, dx));
        dist2 = add(dist2, mul(dz, dz));
        let mut sv_dx = dx;
        let mut sv_dy = dy;
        let mut span = dist2;
        if dist2 > HUNDRED {
            for k in 0..8u32 {
                let slot = this.wrapping_add(FORM_SLOTS).wrapping_add(k.wrapping_mul(SLOT_STRIDE));
                wrf(slot, px);
                wrf(slot.wrapping_add(4), py);
                wrf(slot.wrapping_add(8), pz);
                wr32(slot.wrapping_add(12), ZERO.to_bits());
                wrf(this.wrapping_add(FORM_HEADS).wrapping_add(k.wrapping_mul(4)), heading);
                wr32(this.wrapping_add(FORM_EXTRA).wrapping_add(k.wrapping_mul(4)), SIX.to_bits());
            }
            wrf(this.wrapping_add(FORM_LAST), px);
            wrf(this.wrapping_add(FORM_LAST + 4), py);
            wrf(this.wrapping_add(FORM_LAST + 8), pz);
            wr32(this.wrapping_add(FORM_LAST + 12), ZERO.to_bits());
            sv_dx = ZERO;
            sv_dy = ZERO;
            span = MILLE;
        }
        if span > form8_sq {
            for k in (1..8u32).rev() {
                let src = this.wrapping_add(FORM_SLOTS).wrapping_add(k.wrapping_sub(1).wrapping_mul(SLOT_STRIDE));
                let dst = this.wrapping_add(FORM_SLOTS).wrapping_add(k.wrapping_mul(SLOT_STRIDE));
                wr32(dst, rd32(src));
                wr32(dst.wrapping_add(4), rd32(src.wrapping_add(4)));
                wr32(dst.wrapping_add(8), rd32(src.wrapping_add(8)));
                wr32(dst.wrapping_add(12), ZERO.to_bits());
                wr32(
                    this.wrapping_add(FORM_HEADS).wrapping_add(k.wrapping_mul(4)),
                    rd32(this.wrapping_add(FORM_HEADS).wrapping_add(k.wrapping_sub(1).wrapping_mul(4))),
                );
                wr32(
                    this.wrapping_add(FORM_EXTRA).wrapping_add(k.wrapping_mul(4)),
                    rd32(this.wrapping_add(FORM_EXTRA).wrapping_add(k.wrapping_sub(1).wrapping_mul(4))),
                );
            }
            let head = this.wrapping_add(FORM_SLOTS);
            wrf(head, px);
            wrf(head.wrapping_add(4), py);
            wrf(head.wrapping_add(8), pz);
            wr32(head.wrapping_add(12), ZERO.to_bits());
            wr32(this.wrapping_add(FORM_EXTRA), ONE.to_bits());
            let dx2 = sub(px, rdf(this.wrapping_add(FORM_LAST)));
            let dy2 = sub(py, rdf(this.wrapping_add(FORM_LAST + 4)));
            let d2 = add(mul(dx2, dx2), mul(dy2, dy2));
            if d2 > FIFTY_MILLI {
                let a: f32 = lf_checker_rt::callee_cdecl!(
                    ID_ANGLE, f32,
                    dx2.to_bits(), dy2.to_bits(), 0, 0
                );
                wrf(this.wrapping_add(FORM_HEADS), a);
            } else {
                wrf(this.wrapping_add(FORM_HEADS), heading);
            }
            wrf(this.wrapping_add(FORM_LAST + 8), pz);
            wr32(this.wrapping_add(FORM_LAST + 12), ZERO.to_bits());
            wrf(this.wrapping_add(FORM_LAST), px);
            wrf(this.wrapping_add(FORM_LAST + 4), py);
            sv_dx = ZERO;
            sv_dy = ZERO;
            span = MILLE;
        }
        let vtable = rd32(group);
        let target = rd32(vtable.wrapping_add(VTABLE_SLOT));
        let mut framebuf = [0u32; 4];
        let hook: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let answer = hook(group, framebuf.as_mut_ptr() as u32);
        let f0 = rdf(answer);
        let f1 = rdf(answer.wrapping_add(4));
        let f2 = rdf(answer.wrapping_add(8));
        let sum_sq = add(add(mul(f0, f0), mul(f1, f1)), mul(f2, f2));
        let root_s = core::hint::black_box(sum_sq).sqrt();
        // ucomiss + lahf + test + jp: the block runs unless the span word is
        // exactly zero (a NaN span also runs it); on zero the tag decides.
        let run_block = if span != ZERO {
            true
        } else {
            count != rd32(this.wrapping_add(FORM_TAG))
        };
        if run_block {
            wrf(this.wrapping_add(FORM_DIR), px);
            wrf(this.wrapping_add(FORM_DIR + 4), py);
            wrf(this.wrapping_add(FORM_DIR + 8), pz);
            wrf(this.wrapping_add(FORM_DIR + 12), heading);
            let qy = mul(sv_dy, sv_dy);
            let qx = mul(sv_dx, sv_dx);
            wr32(this.wrapping_add(FORM_DIR + 16), ONE.to_bits());
            let r = core::hint::black_box(add(qy, qx)).sqrt();
            let capped = if r > form8 { form8 } else { r };
            let factor = div(capped, form8);
            let mut e_cur = this.wrapping_add(FORM_EXTRA).wrapping_add(4);
            let mut z_ptr = this.wrapping_add(FORM_SLOTS).wrapping_add(0x18);
            let neg_off = NEG_OFF.wrapping_sub(this);
            let mut index: u32 = 0;
            let mut out = this.wrapping_add(FORM_OUT);
            for k in 0..7u32 {
                let poll: u32 = lf_checker_rt::callee_thiscall!(ID_POLL, u32, group_ref, k);
                if poll != 0 {
                    let base = index.wrapping_add(this);
                    let d3 = sub(rdf(base.wrapping_add(0xf8)), rdf(z_ptr));
                    let d7 = sub(rdf(base.wrapping_add(0xf0)), rdf(z_ptr.wrapping_sub(8)));
                    let d5 = sub(rdf(base.wrapping_add(0xf4)), rdf(z_ptr.wrapping_sub(4)));
                    let v5 = add(mul(d5, factor), rdf(z_ptr.wrapping_sub(4)));
                    let v4 = add(mul(d7, factor), rdf(z_ptr.wrapping_sub(8)));
                    let v6 = add(mul(d3, factor), rdf(z_ptr));
                    let (mut w20, mut w1c, mut w10) = (v4, v5, v6);
                    if root_s > TENTH {
                        w10 = add(mul(d3, HALF), w10);
                        w20 = add(mul(d7, HALF), w20);
                        w1c = add(mul(d5, HALF), w1c);
                    }
                    let ang: f32 = lf_checker_rt::callee_cdecl!(
                        ID_ANGLE, f32,
                        d7.to_bits(), d5.to_bits(), 0, 0
                    );
                    wrf(out.wrapping_sub(8), w20);
                    wrf(out.wrapping_sub(4), w1c);
                    wrf(out, w10);
                    wrf(out.wrapping_add(4), ang);
                    wr32(out.wrapping_add(8), rd32(e_cur));
                    index = neg_off.wrapping_add(z_ptr);
                    z_ptr = z_ptr.wrapping_add(0x10);
                    e_cur = e_cur.wrapping_add(4);
                }
                out = out.wrapping_add(OUT_STRIDE);
            }
        }
        wr32(this.wrapping_add(FORM_TAG), count);
        1
    }
});
