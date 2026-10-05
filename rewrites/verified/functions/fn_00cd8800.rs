// original: 0x00cd8800 CTaskComplexSeekEntityAiming::vf20

/// Range-gated aim request for a seek-entity-aiming task (thiscall, one stack word).
///
/// `this` is the task object: `+0x08` holds the gate object, `+0x14` the
/// aim target record, `+0x1c` the engagement-range float. `p0` is the event
/// record whose `+0x20` points at the subject's position block (x at `+0x30`,
/// y at `+0x34`).
///
/// The anchor position comes from the target record: when its `+0x20` link is
/// set the anchor is that block's `+0x30`/`+0x34`, otherwise the record's own
/// `+0x10`/`+0x14`. The planar offset (subject minus anchor) is formed with
/// two scalar-float subtracts.
///
/// Gates, in order: the target record and the event must be non-null; the
/// gate object must be non-null and its virtual slot `+0x0c` must answer
/// `0x11d`. Then the squared range is compared against the squared offset
/// length (`dx*dx + dy*dy`, formed exactly so): a range strictly greater
/// takes the near path, anything else (including an unordered NaN compare)
/// the far path. Both paths re-check the sub-object at gate `+0x08`
/// (non-null, slot `+0x0c` answering `0x119` near / `0x3fc` far), fetch a
/// builder through the manager getter, and commit one value to the gate
/// object: near builds with (2, target, 0, 60.0, 0, 1, 1, 1.0) and commits
/// the result, far calls the two-word filler with (0, 0) and commits that,
/// and a null builder commits 0. Any failed gate skips straight to the exit.
///
/// The return value is always the gate object (`[this+0x08]`), whatever path
/// ran. No heap or global memory is written; only frame scratch is used.
///
/// Original: 0x00cd8800 (thiscall, one stack word, always returns `[this+8]`).
lf_checker_rt::export!(thiscall, rw_00cd8800(this: u32, p0: u32) -> u32 {
    unsafe {
        const GATE_OFF: u32 = 0x08;
        const TARGET_OFF: u32 = 0x14;
        const RANGE_OFF: u32 = 0x1c;
        const POS_LINK: u32 = 0x20;
        const POS_X: u32 = 0x30;
        const POS_Y: u32 = 0x34;
        const ANCHOR_X: u32 = 0x10;
        const ANCHOR_Y: u32 = 0x14;
        const SUB_OFF: u32 = 0x08;
        const VSLOT_GATE: u32 = 0x0c;
        const GATE_OK: u32 = 0x11d;
        const NEAR_OK: u32 = 0x119;
        const FAR_OK: u32 = 0x3fc;
        const MANAGER: u32 = 0x0167e2a0;
        const ONE: u32 = 0x3f80_0000;
        const SIXTY: u32 = 0x4270_0000;
        const CALLEE_GETTER: u32 = 1;
        const CALLEE_BUILD: u32 = 2;
        const CALLEE_FILL: u32 = 3;
        const CALLEE_COMMIT: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        /// Call virtual slot `VSLOT_GATE` on `obj` (object pointer in ecx).
        #[inline(always)]
        unsafe fn vgate(obj: u32) -> u32 {
            unsafe {
                let slot: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + VSLOT_GATE) as usize);
                slot(obj)
            }
        }

        let gate = rd32(this + GATE_OFF);
        let target = rd32(this + TARGET_OFF);
        if target == 0 || p0 == 0 {
            return gate;
        }
        let mid = rd32(target + POS_LINK);
        let (ax, ay) = if mid != 0 {
            (rd32(mid + POS_X), rd32(mid + POS_Y))
        } else {
            (rd32(target + ANCHOR_X), rd32(target + ANCHOR_Y))
        };
        let subj = rd32(p0 + POS_LINK);
        let dx = sub(
            f32::from_bits(rd32(subj + POS_X)),
            f32::from_bits(ax),
        );
        let dy = sub(
            f32::from_bits(rd32(subj + POS_Y)),
            f32::from_bits(ay),
        );
        if gate == 0 || vgate(gate) != GATE_OK {
            return gate;
        }
        let r = f32::from_bits(rd32(this + RANGE_OFF));
        let dist2 = add(mul(dx, dx), mul(dy, dy));
        let r2 = mul(r, r);
        // comiss+ja semantics: near only when strictly greater (NaN goes far).
        let want = if r2 > dist2 { NEAR_OK } else { FAR_OK };
        let sub_obj = rd32(gate + SUB_OFF);
        if sub_obj == 0 || vgate(sub_obj) != want {
            return gate;
        }
        let mgr = lf_checker_rt::global::<u32>(MANAGER).read();
        let builder: u32 = lf_checker_rt::callee_thiscall!(CALLEE_GETTER, u32, mgr);
        let value = if builder == 0 {
            0
        } else if want == NEAR_OK {
            lf_checker_rt::callee_thiscall!(
                CALLEE_BUILD, u32, builder, 2u32, target, 0u32, SIXTY, 0u32, 1u32, 1u32, ONE
            )
        } else {
            lf_checker_rt::callee_thiscall!(CALLEE_FILL, u32, builder, 0u32, 0u32)
        };
        lf_checker_rt::callee_thiscall!(CALLEE_COMMIT, u32, gate, value);
        gate
    }
});
