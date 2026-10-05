// original: 0x00cffb40 combat_range_gate (proposed)

/// Gate a combat-task update on three range tests, then ask two helpers.
///
/// `this` is the task object, `slot` a 3-float position record, `target` a
/// handle whose word at `+0x20` points at the target's position record
/// (floats at `+0x30/+0x34/+0x38`). Returns an 8-bit status in `al`, nonzero
/// on two early exits; the upper 24 bits of `eax` are whatever the last
/// executed step left there (a kept callee answer or a kept pointer), and
/// are reproduced exactly.
///
/// Behaviour in order:
/// 1. Increment the re-entry counter at `this+0x94` and call helper 0 with
///    (counter, `target`, `slot`). A zero `al` answer resets the counter
///    to 8 and returns.
/// 2. If the mode byte at `this+0xa4` is nonzero, call helper 1 on it with
///    one out-word, and return whether that word equals 1 (a zero `al`
///    answer decrements the counter and returns zero instead).
/// 3. Otherwise compare squared distances: |slot - anchor|^2 (anchor is the
///    three floats at `this+0x20/0x24/0x28`) against |target - anchor|^2;
///    return when the slot is not the nearer.
/// 4. Compare |slot - cover|^2 (cover is the record behind `this+0xac`)
///    against 16.0 and then against |target - cover|^2; call helper 2 with
///    (`target`, cover, `slot`, 1.0, 0, 0, 0) when the slot loses either
///    test. A nonzero `al` answer returns.
/// 5. Spill `this+0x80/0x84/0x88` to a scratch quad; when cover exists and
///    its word at `+0xd68` is nonzero, call helper 3 with that word, the
///    cover pointer, a 3-float direction, a scratch buffer and an out-quad,
///    and take the out-quad as the scratch quad on a nonzero `al` answer.
/// 6. Return 1 when the flag bit at `this+0xbc` is set; otherwise call
///    helper 4 on the cover pointer (0 when there is none) and then
///    helper 5 with (`slot`, scratch area, 2, helper-4 answer, 0x8e, 0).
///    A nonzero `al` answer decrements the counter and returns zero.
///
/// Two null checks on the cover pointer sit after its first dereference,
/// so they never fire; they are kept as written. All float arithmetic is
/// single-precision in the original's operand order; `comiss` + `jbe`
/// is "not above" (true for NaN) and `comiss` + `ja` is "above".
///
/// Original: 0x00cffb40 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00cffb40(this: u32, slot: u32, target: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x94;
        const MODE: u32 = 0xa4;
        const COVER: u32 = 0xac;
        const FLAG: u32 = 0xbc;
        const ANCHOR_X: u32 = 0x20;
        const ANCHOR_Y: u32 = 0x24;
        const ANCHOR_Z: u32 = 0x28;
        const SPILL: u32 = 0x80;
        const HANDLE_POS: u32 = 0x20;
        const POS_X: u32 = 0x30;
        const POS_Y: u32 = 0x34;
        const POS_Z: u32 = 0x38;
        const COVER_KEY: u32 = 0xd68;
        const ONE_BITS: u32 = 0x3f80_0000;
        const HELPER5_TAG: u32 = 0x8e;

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
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
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
        fn above(a: f32, b: f32) -> bool {
            a > b // comiss + ja: false for NaN
        }
        #[inline(always)]
        fn not_above(a: f32, b: f32) -> bool {
            !(a > b) // comiss + jbe: true for NaN
        }

        // Step 1: count and helper 0.
        let count = rd32(this.wrapping_add(COUNT)).wrapping_add(1);
        wr32(this.wrapping_add(COUNT), count);
        let r0: u32 = lf_checker_rt::callee_thiscall!(0, u32, this, count, target, slot);
        if r0 & 0xff == 0 {
            wr32(this.wrapping_add(COUNT), 8);
            return r0 & 0xffff_ff00;
        }
        // Step 2: mode byte path with helper 1.
        if ((this.wrapping_add(MODE)) as *const u8).read() != 0 {
            let mut out: u32 = 0;
            let r1: u32 = lf_checker_rt::callee_thiscall!(1, u32, this.wrapping_add(MODE), &mut out as *mut u32 as u32);
            if r1 & 0xff == 0 {
                wr32(this.wrapping_add(COUNT), count.wrapping_sub(1));
                return r1 & 0xffff_ff00;
            }
            return (r1 & 0xffff_ff00) | ((out == 1) as u32);
        }
        // Step 3: slot-nearer-than-target test around the anchor.
        let d0 = rdf(slot);
        let d1 = rdf(slot.wrapping_add(4));
        let d2 = rdf(slot.wrapping_add(8));
        let tpos = rd32(target.wrapping_add(HANDLE_POS));
        let mut near_slot = sub(rdf(tpos.wrapping_add(POS_Y)), rdf(this.wrapping_add(ANCHOR_Y)));
        let mut near_tgt = sub(d0, rdf(this.wrapping_add(ANCHOR_X)));
        let mut far_slot = sub(rdf(tpos.wrapping_add(POS_X)), rdf(this.wrapping_add(ANCHOR_X)));
        let mut far_tgt = sub(d2, rdf(this.wrapping_add(ANCHOR_Z)));
        let mut up_tgt = sub(rdf(tpos.wrapping_add(POS_Z)), rdf(this.wrapping_add(ANCHOR_Z)));
        let mut up_slot = sub(d1, rdf(this.wrapping_add(ANCHOR_Y)));
        near_slot = mul(near_slot, near_slot);
        near_tgt = mul(near_tgt, near_tgt);
        up_slot = mul(up_slot, up_slot);
        far_slot = mul(far_slot, far_slot);
        up_slot = add(up_slot, near_tgt);
        far_tgt = mul(far_tgt, far_tgt);
        near_slot = add(near_slot, far_slot);
        up_tgt = mul(up_tgt, up_tgt);
        up_slot = add(up_slot, far_tgt);
        near_slot = add(near_slot, up_tgt);
        if not_above(up_slot, near_slot) {
            return tpos & 0xffff_ff00;
        }
        // Step 4: cover tests, then helper 2.
        let cover = rd32(this.wrapping_add(COVER));
        let cpos = rd32(cover.wrapping_add(HANDLE_POS));
        let mut s7 = sub(d1, rdf(cpos.wrapping_add(POS_Y)));
        let mut s0 = sub(d0, rdf(cpos.wrapping_add(POS_X)));
        let mut s1 = sub(d2, rdf(cpos.wrapping_add(POS_Z)));
        s7 = mul(s7, s7);
        s0 = mul(s0, s0);
        s1 = mul(s1, s1);
        s7 = add(s7, s0);
        s7 = add(s7, s1);
        if cover == 0 {
            return cpos & 0xffff_ff00;
        }
        let dist_max = rdf(lf_checker_rt::relocated(0x00fe8b28));
        if !above(s7, dist_max) {
            let tpos2 = rd32(target.wrapping_add(HANDLE_POS));
            let mut u1 = sub(rdf(tpos2.wrapping_add(POS_X)), rdf(cpos.wrapping_add(POS_X)));
            let mut u2 = sub(rdf(tpos2.wrapping_add(POS_Y)), rdf(cpos.wrapping_add(POS_Y)));
            let mut u0 = sub(rdf(tpos2.wrapping_add(POS_Z)), rdf(cpos.wrapping_add(POS_Z)));
            u2 = mul(u2, u2);
            u1 = mul(u1, u1);
            u0 = mul(u0, u0);
            u2 = add(u2, u1);
            u2 = add(u2, u0);
            if not_above(s7, u2) {
                return cpos & 0xffff_ff00;
            }
        }
        let r2: u32 = lf_checker_rt::callee_cdecl!(2, u32, target, cover, slot, ONE_BITS, 0, 0, 0);
        if r2 & 0xff != 0 {
            return r2 & 0xffff_ff00;
        }
        // Step 5: spill and helper 3.
        let mut quad = [
            rd32(this.wrapping_add(SPILL)),
            rd32(this.wrapping_add(SPILL).wrapping_add(4)),
            rd32(this.wrapping_add(SPILL).wrapping_add(8)),
            0u32,
        ];
        let cover2 = rd32(this.wrapping_add(COVER));
        let mut last = r2;
        let mut p1w0 = 0u32;
        if cover2 != 0 && rd32(cover2.wrapping_add(COVER_KEY)) != 0 {
            let key = rd32(cover2.wrapping_add(COVER_KEY));
            let tp = rd32(target.wrapping_add(HANDLE_POS));
            let cp = rd32(cover2.wrapping_add(HANDLE_POS));
            let dir = [
                sub(rdf(tp.wrapping_add(POS_X)), rdf(cp.wrapping_add(POS_X))).to_bits(),
                sub(rdf(tp.wrapping_add(POS_Y)), rdf(cp.wrapping_add(POS_Y))).to_bits(),
                sub(rdf(tp.wrapping_add(POS_Z)), rdf(cp.wrapping_add(POS_Z))).to_bits(),
            ];
            let mut scratch = [0u32; 4];
            let mut outq = [0u32; 4];
            let r3: u32 = lf_checker_rt::callee_cdecl!(
                3, u32, key, cover2,
                dir.as_ptr() as u32, scratch.as_mut_ptr() as u32, outq.as_mut_ptr() as u32
            );
            last = r3;
            p1w0 = outq[0];
            if r3 & 0xff != 0 {
                quad = outq;
            }
        }
        // Step 6: flag exit, helpers 4 and 5.
        if ((this.wrapping_add(FLAG)) as *const u8).read() & 1 != 0 {
            return (last & 0xffff_ff00) | 1;
        }
        let cover3 = rd32(this.wrapping_add(COVER));
        let r4: u32 = if cover3 != 0 {
            lf_checker_rt::callee_thiscall!(4, u32, cover3)
        } else {
            0
        };
        let mut area = [0u32; 8];
        area[3] = quad[0];
        area[4] = quad[1];
        area[5] = quad[2];
        area[6] = quad[3];
        area[7] = p1w0;
        let r5: u32 = lf_checker_rt::callee_thiscall!(
            5, u32, this.wrapping_add(MODE), slot, area.as_mut_ptr() as u32, 2, r4, HELPER5_TAG, 0
        );
        if r5 & 0xff == 0 {
            return r5 & 0xffff_ff00;
        }
        wr32(this.wrapping_add(COUNT), count.wrapping_sub(1));
        r5 & 0xffff_ff00
    }
});
