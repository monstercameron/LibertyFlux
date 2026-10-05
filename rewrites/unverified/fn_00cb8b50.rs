// original: 0x00CB8B50 peds_task_guard_8b50 (proposed)

/// Guarded task step: run one of three follow-up calls by owner state.
///
/// `this` is the task (`+0x08` its owner), `a0` an object with a vector
/// record at `+0x20`, `a1` a second vector record. The function first polls
/// the owner's id slot up to three times, continuing when any answer is
/// `0x386`, `0x11a` or `0x3b7` and returning 0 otherwise. A fourth poll for
/// `0x3b7` plus a flag byte and a child id check select the fast path: a
/// range check of the `a0` vector against the task's anchor (squared length
/// above the task's own squared limit) followed by an owner notification and
/// the `0x3ae` follow-up, whose answer is returned. Every other route lands
/// in the measurement block, which diffs the two vector records, stores the
/// `a1` record into the task, and steers by three float gates: below `6.0`
/// it takes the low branch (three more gates against `0.0625`, the task's
/// squared limit and `4.0`, then a notification and the `0x384` follow-up),
/// at or above it takes the high branch (past the squared limit, or past an
/// absolute `4.0` bound, into an owner poll that sets flag bit 1 and the
/// `0x3ae` follow-up). Any gate failure returns 0. Original: thiscall with
/// two stack words, no frame pointer.
lf_checker_rt::export!(thiscall, rw_00cb8b50(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const THIS_OWNER: u32 = 0x08;
        const OWNER_ID_SLOT: u32 = 0x0C;
        const OWNER_POLL_SLOT: u32 = 0x14;
        const OWNER_GATE_BYTE: u32 = 0x5A;
        const OWNER_CHILD: u32 = 0x08;
        const OWNER_FLAGS: u32 = 0x0C;
        const ID_A: u32 = 0x386;
        const ID_B: u32 = 0x11A;
        const ID_C: u32 = 0x3B7;
        const ID_CHILD_SKIP: u32 = 0x3AE;
        const FOLLOW_A: u32 = 0x3AE;
        const FOLLOW_B: u32 = 0x384;
        const FLAG_ADVANCED: u32 = 2;
        const VEC_PTR: u32 = 0x20;
        const TASK_V0: u32 = 0x40;
        const TASK_V1: u32 = 0x44;
        const TASK_V2: u32 = 0x48;
        const TASK_V3: u32 = 0x4C;
        const TASK_A0: u32 = 0x50;
        const TASK_A1: u32 = 0x54;
        const TASK_A2: u32 = 0x58;
        const TASK_LIM: u32 = 0x64;
        const TASK_THRESH: u32 = 0x68;
        const C_FAR: f32 = 6.0;
        const C_NEAR: f32 = 0.0625;
        const C_FOUR: f32 = 4.0;
        const C_TINY: f32 = 0.0009765625;
        const ABS_MASK: u32 = 0x7FFF_FFFF;
        const CALLEE_SELFTEST: u32 = 3;
        const CALLEE_NOTIFY: u32 = 4;
        const CALLEE_FOLLOW: u32 = 5;

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
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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
        fn absf(a: f32) -> f32 {
            f32::from_bits(a.to_bits() & ABS_MASK)
        }
        // Id-slot poll through the object's own vtable (lands on whichever
        // stub the contract planted there).
        #[inline(always)]
        unsafe fn owner_id(owner: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(owner) + OWNER_ID_SLOT) as usize);
                f(owner)
            }
        }

        let owner = rd32(this + THIS_OWNER);
        if owner_id(owner) != ID_A {
            if owner_id(owner) != ID_B {
                if owner_id(owner) != ID_C {
                    return 0;
                }
            }
        }
        // Fast-path entry gates, in order, each firing at most once.
        let mut fast = false;
        if owner_id(owner) == ID_C && rd8(owner + OWNER_GATE_BYTE) != 0 {
            let child = rd32(owner + OWNER_CHILD);
            if child == 0 || owner_id(child) != ID_CHILD_SKIP {
                if lf_checker_rt::callee_thiscall!(CALLEE_SELFTEST, u32, this) != 0 {
                    fast = true;
                }
            }
        }
        if fast {
            let vec = rd32(a0 + VEC_PTR);
            let thr = rdf(this + TASK_THRESH);
            let dy = sub(rdf(vec + 0x30), rdf(this + TASK_A0));
            let dz = sub(rdf(vec + 0x34), rdf(this + TASK_A1));
            let dx = sub(rdf(vec + 0x38), rdf(this + TASK_A2));
            let sumsq = add(add(mul(dz, dz), mul(dy, dy)), mul(dx, dx));
            if sumsq > mul(thr, thr) {
                let ok =
                    lf_checker_rt::callee_thiscall!(CALLEE_NOTIFY, u32, owner, a0, 1u32, 0u32)
                        & 0xFF;
                if ok != 0 {
                    return lf_checker_rt::callee_thiscall!(
                        CALLEE_FOLLOW, u32, this, FOLLOW_A, a0,
                    );
                }
            }
            return 0;
        }
        // Measurement block.
        let vec = rd32(a0 + VEC_PTR);
        let b0 = rdf(a1);
        let b1 = rdf(a1 + 4);
        let b2 = rdf(a1 + 8);
        let d0 = sub(rdf(vec + 0x30), b0);
        let d1 = sub(rdf(vec + 0x34), b1);
        let d2 = sub(b0, rdf(this + TASK_V0));
        let s1 = add(mul(d1, d1), mul(d0, d0));
        let e0 = sub(b1, rdf(this + TASK_V1));
        let d3 = sub(rdf(vec + 0x38), b2);
        let s0 = add(mul(e0, e0), mul(d2, d2));
        if !(s0 > C_TINY) {
            return 0;
        }
        wrf(this + TASK_V0, b0);
        wrf(this + TASK_V1, b1);
        wrf(this + TASK_V2, b2);
        wr32(this + TASK_V3, rd32(a1 + 0x0C));
        let q = rdf(this + TASK_LIM);
        if s1 >= C_FAR {
            // High branch: past the squared limit or the absolute bound the
            // owner poll runs; otherwise this falls into the low gates below.
            if s1 > mul(q, q) || absf(d3) > C_FOUR {
                if (rd8(owner + OWNER_FLAGS) & 1) == 0 {
                    let poll: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                        core::mem::transmute(rd32(rd32(owner) + OWNER_POLL_SLOT) as usize);
                    if (poll(owner, a0, 1u32, 0u32) & 0xFF) == 0 {
                        return 0;
                    }
                    wr32(owner + OWNER_FLAGS, rd32(owner + OWNER_FLAGS) | FLAG_ADVANCED);
                }
                return lf_checker_rt::callee_thiscall!(CALLEE_FOLLOW, u32, this, FOLLOW_A, a0);
            }
        }
        // Low gates (shared by the low branch and high-branch fallthrough).
        if !(s1 > C_NEAR) {
            return 0;
        }
        if !(s1 > mul(q, q)) {
            return 0;
        }
        if !(C_FOUR > absf(d3)) {
            return 0;
        }
        let ok = lf_checker_rt::callee_thiscall!(CALLEE_NOTIFY, u32, owner, a0, 1u32, 0u32) & 0xFF;
        if ok == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(CALLEE_FOLLOW, u32, this, FOLLOW_B, a0)
    }
});
