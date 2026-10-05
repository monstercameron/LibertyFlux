// original: 0x00a2a6c0 pedtask_range_gate (proposed)
//
// DRAFT (lane r-b409): not verified. The original reads a game-global mode
// word through an unrelocated absolute address (see report.md), which the
// checker cannot serve, so no verdict exists for this rewrite. Kept as a
// starting point for a future lane once the checker can shadow writable data.

/// Decide whether a task's target is in range and suitably gated.
///
/// `this` is the task object, `target` the candidate entity, `task` a
/// parameter block holding the radius (`+0x14`), a flag byte (`+0x20`) and a
/// mode word (`+0xc`). Returns 1 to reject early, 0 to accept.
///
/// Behaviour: the radius is `scale_in = [task+0x14]`, multiplied by the
/// factor at `[[this+0x228]+0x5b4]` (as `factor * scale_in`) when the global
/// mode word equals 2, the gate callee answers non-zero and `[this+0x228]`
/// is non-null. The transform callee then runs with `(target, this, &local)`
/// and may overwrite the radius slot. The squared distance between the
/// target's anchor (`[target+0x20]+0x30`, or `target+0x10` when null) and
/// `[this+0x20]+0x30/34/38` is formed as `(dy*dy + dx*dx) + dz*dz`; a
/// distance strictly above the squared radius rejects. Bit 0 of
/// `[task+0x20]` must be set. When the mode word is 2 and
/// `([target+0x28] & 0x3c0) == 0xc0` with `[task+4] != 8`, the probe callee
/// must accept the target (unless `[target+0x211]` is already 0) and
/// `[target+0x210]` must be 0. Otherwise a position triple is read from the
/// target's anchor (replaced by the fetch callee's `+0x30..+0x3c` block when
/// the kind bits equal 0xc0); a task mode of 1 accepts, anything else runs
/// the final callee with `(this, &pos, task, 0)` and accepts unless it
/// answers 0.
///
/// NaN distances take the accept path (`jbe`); the rewrite uses `>`-form
/// comparisons.
///
/// Original: 0x00a2a6c0 (thiscall, two stack words; returns al, upper bytes
/// are stale).
lf_checker_rt::export!(thiscall, rw_00a2a6c0(this: u32, target: u32, task: u32) -> u32 {
    unsafe {
        const TASK_RADIUS: u32 = 0x14;
        const TASK_MODE: u32 = 0x0c;
        const TASK_KIND: u32 = 0x04;
        const TASK_FLAGS: u32 = 0x20;
        const ENT_ANCHOR: u32 = 0x20;
        const ANCHOR_POS: u32 = 0x30;
        const ENT_KIND: u32 = 0x28;
        const KIND_MASK: u32 = 0x3c0;
        const KIND_SUB: u32 = 0x0c0;
        const GATE_CALLEE: u32 = 1;
        const TRANSFORM_CALLEE: u32 = 2;
        const PROBE_CALLEE: u32 = 3;
        const FETCH_CALLEE: u32 = 4;
        const FINAL_CALLEE: u32 = 5;
        const G_MODE: u32 = 0x11d6fd4;
        const MODE_SCALED: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        unsafe fn anchor_pos(ent: u32) -> u32 {
            unsafe {
                let a = rd32(ent.wrapping_add(ENT_ANCHOR));
                if a != 0 {
                    a.wrapping_add(ANCHOR_POS)
                } else {
                    ent.wrapping_add(0x10)
                }
            }
        }

        let gmode: u32 = lf_checker_rt::global::<u32>(G_MODE).read();
        let mut radius = rdf(task.wrapping_add(TASK_RADIUS));
        if gmode == MODE_SCALED {
            let gate: u32 =
                lf_checker_rt::callee_thiscall!(GATE_CALLEE, u32, this);
            if gate != 0 {
                let m = rd32(this.wrapping_add(0x228));
                if m != 0 {
                    // Operand order is the original's: factor * radius.
                    radius = mul(rdf(m.wrapping_add(0x5b4)), radius);
                }
            }
        }

        // The transform callee may overwrite the radius slot; its write
        // width is unknown (at least one word), so the contract declares it.
        let mut radius_slot = radius;
        let out_ptr = core::ptr::addr_of_mut!(radius_slot) as u32;
        let _: u32 =
            lf_checker_rt::callee_cdecl!(TRANSFORM_CALLEE, u32, target, this, out_ptr);
        radius = radius_slot;

        let p = anchor_pos(target);
        let base = rd32(this.wrapping_add(ENT_ANCHOR));
        let dx = sub(rdf(p), rdf(base.wrapping_add(0x30)));
        let dy = sub(rdf(p.wrapping_add(4)), rdf(base.wrapping_add(0x34)));
        let dz = sub(rdf(p.wrapping_add(8)), rdf(base.wrapping_add(0x38)));
        // Operand order is the original's: (dy*dy + dx*dx) + dz*dz.
        let dist2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
        let r2 = mul(radius, radius);
        if dist2 > r2 {
            return 1;
        }
        if rd8(task.wrapping_add(TASK_FLAGS)) & 1 == 0 {
            return 1;
        }
        if gmode == MODE_SCALED {
            let is_eight = rd32(task.wrapping_add(TASK_KIND)) == 8;
            if rd32(target.wrapping_add(ENT_KIND)) & KIND_MASK == KIND_SUB
                && !is_eight
            {
                if rd8(target.wrapping_add(0x211)) != 0 {
                    let ok: u32 =
                        lf_checker_rt::callee_thiscall!(PROBE_CALLEE, u32, target);
                    if ok == 0 {
                        return 1;
                    }
                }
                if rd8(target.wrapping_add(0x210)) != 0 {
                    return 1;
                }
            }
        }

        let p2 = anchor_pos(target);
        // Fourth word: only written on the fetch path below; the original
        // leaves stack garbage there otherwise, so the future contract must
        // either define the stack fill or snapshot only three words.
        let mut pos = [0u32; 4];
        pos[0] = rd32(p2);
        pos[1] = rd32(p2.wrapping_add(4));
        pos[2] = rd32(p2.wrapping_add(8));
        if rd32(target.wrapping_add(ENT_KIND)) & KIND_MASK == KIND_SUB {
            let q: u32 =
                lf_checker_rt::callee_thiscall!(FETCH_CALLEE, u32, target, 0u32);
            pos[0] = rd32(q.wrapping_add(0x30));
            pos[1] = rd32(q.wrapping_add(0x34));
            pos[2] = rd32(q.wrapping_add(0x38));
            pos[3] = rd32(q.wrapping_add(0x3c));
        }
        if rd32(task.wrapping_add(TASK_MODE)) == 1 {
            return 0;
        }
        let pos_ptr = pos.as_mut_ptr() as u32;
        let done: u32 = lf_checker_rt::callee_thiscall!(
            FINAL_CALLEE, u32, this, pos_ptr, task, 0u32);
        if done == 0 {
            1
        } else {
            0
        }
    }
});
