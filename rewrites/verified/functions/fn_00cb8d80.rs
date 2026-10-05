// original: 0x00CB8D80 peds_task_guard_8d80 (proposed)

/// Guarded task advance: pick one of several follow-ups by owner and task state.
///
/// `this` is the task (`+0x08` its owner), `a0`/`a1` two vector records.
/// When the owner's flag word reads exactly `4` in its low three bits the
/// function runs a self-test call and polls the owner's id slot: answer
/// `0x3ae` enters the short path (a second id check, a possible notify plus
/// `0x38b` follow-up, else a vector-slot call whose point is gated and an
/// owner-slot call, finished by stamping the owner state and returning the
/// owner), answer `0x384` enters the long path (a vector-slot call gated at
/// `0.2` with a notify plus owner call, else a two-vector measurement
/// steering into a notify plus `0x3ae` follow-up or a second owner-slot
/// call), and any other answer returns the owner at once. A null self-test
/// skips the follow-ups and lands in the shared tails. Original: thiscall
/// with two stack words.
lf_checker_rt::export!(thiscall, rw_00cb8d80(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const THIS_OWNER: u32 = 0x08;
        const THIS_SEQ: u32 = 0x78;
        const THIS_MODE: u32 = 0xB4;
        const OWNER_FLAGS: u32 = 0x0C;
        const OWNER_VEC: u32 = 0x14;
        const OWNER_ST0: u32 = 0x60;
        const OWNER_ST1: u32 = 0x64;
        const OWNER_BITS: u32 = 0xD8;
        const VT_ID: u32 = 0x0C;
        const VT_POINT: u32 = 0x1C;
        const VT_SLOT: u32 = 0x24;
        const VT_CB: u32 = 0x04;
        const VT_OWNER_CALL: u32 = 0x48;
        const ID_SHORT: u32 = 0x3AE;
        const ID_LONG: u32 = 0x384;
        const ID_CHILD_OK: u32 = 2;
        const FOLLOW_SHORT: u32 = 0x38B;
        const FOLLOW_LONG: u32 = 0x3AE;
        const BIT_ADVANCED: u32 = 0x100000;
        const BIT_EXTRA: u32 = 0x200000;
        const ST1_NEG: u32 = 0xC47A0000;
        const GATE_ONE: f32 = 1.0;
        const GATE_SIX: f32 = 6.0;
        const GATE_FOUR: f32 = 4.0;
        const GATE_SHORT: f32 = 0.2;
        const GATE_FAR: f32 = 8.0;
        const ABS_MASK: u32 = 0x7FFF_FFFF;
        const CALLEE_SELFTEST: u32 = 1;
        const CALLEE_CHILD: u32 = 3;
        const CALLEE_NOTIFY: u32 = 4;
        const CALLEE_FOLLOW: u32 = 5;
        const CALLEE_FLOAT: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        #[inline(always)]
        unsafe fn owner_id(owner: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(owner) + VT_ID) as usize);
                f(owner)
            }
        }

        let owner = rd32(this + THIS_OWNER);
        // Entry: low three flag bits must read exactly 0b100.
        let flags = rd32(owner + OWNER_FLAGS);
        if ((flags >> 2) & 1) == 0 || (flags & 1) != 0 || ((flags >> 1) & 1) != 0 {
            return owner;
        }
        let probe = lf_checker_rt::callee_thiscall!(CALLEE_SELFTEST, u32, this);
        let id = owner_id(owner);
        if id == ID_LONG {
            // Long path: vector-slot call gated at 0.2, else the shared tail.
            if probe != 0 && rd32(this + THIS_SEQ) == 1 {
                let anchor = rd32(probe + 0x20);
                let slot_obj = owner + OWNER_VEC;
                let point: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(slot_obj) + VT_POINT) as usize);
                let mut scratch = [0u32; 4];
                let rp = point(slot_obj, scratch.as_mut_ptr() as u32);
                let dx = sub(rdf(anchor + 0x30), rdf(rp));
                let dy = sub(rdf(anchor + 0x34), rdf(rp + 4));
                let dz = sub(rdf(anchor + 0x38), rdf(rp + 8));
                let sumsq = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                if GATE_SHORT > sumsq {
                    let ok = lf_checker_rt::callee_thiscall!(
                        CALLEE_NOTIFY, u32, owner, a0, 1u32, 0u32,
                    ) & 0xFF;
                    if ok != 0 {
                        let ocall: extern "thiscall" fn(u32, u32) -> u32 =
                            core::mem::transmute(rd32(rd32(this) + VT_OWNER_CALL) as usize);
                        return ocall(this, a0);
                    }
                }
            }
            // Shared tail: measure the two vectors and steer.
            let vec = rd32(a0 + 0x20);
            let dx = sub(rdf(a1 + 4), rdf(vec + 0x34));
            let dy = sub(rdf(a1), rdf(vec + 0x30));
            let dz = sub(rdf(a1 + 8), rdf(vec + 0x38));
            let flat = add(mul(dx, dx), mul(dy, dy));
            if flat >= GATE_FAR || absf(dz) > GATE_FOUR {
                let ok = lf_checker_rt::callee_thiscall!(
                    CALLEE_NOTIFY, u32, owner, a0, 1u32, 0u32,
                ) & 0xFF;
                if ok != 0 {
                    return lf_checker_rt::callee_thiscall!(
                        CALLEE_FOLLOW, u32, this, FOLLOW_LONG, a0,
                    );
                }
            }
            let slot_obj = owner + OWNER_VEC;
            let slot: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(slot_obj) + VT_SLOT) as usize);
            slot(slot_obj, a0, a1, 0u32);
            let fv: f32 =
                lf_checker_rt::callee_thiscall!(CALLEE_FLOAT, f32, this, a0, a1, 0u32);
            let cb: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(slot_obj) + VT_CB) as usize);
            cb(slot_obj, fv.to_bits());
            return owner;
        }
        if id != ID_SHORT {
            return owner;
        }
        // Short path.
        if lf_checker_rt::callee_thiscall!(CALLEE_CHILD, u32, owner) == ID_CHILD_OK && probe != 0
        {
            let ok =
                lf_checker_rt::callee_thiscall!(CALLEE_NOTIFY, u32, owner, a0, 1u32, 0u32) & 0xFF;
            if ok != 0 {
                return lf_checker_rt::callee_thiscall!(CALLEE_FOLLOW, u32, this, FOLLOW_SHORT, a0);
            }
        }
        // Short measurement: the w14 word, the point call, the gates.
        let w14 = if probe != 0 {
            rd32(rd32(probe + 0x224) + 0x2E8) & 7
        } else {
            1
        };
        let mode = rd32(this + THIS_MODE) as u8;
        let slot_obj = owner + OWNER_VEC;
        let point: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(slot_obj) + VT_POINT) as usize);
        let mut scratch = [0u32; 4];
        let rp = point(slot_obj, scratch.as_mut_ptr() as u32);
        let dy = sub(rdf(rp + 4), rdf(a1 + 4));
        let dx = sub(rdf(rp), rdf(a1));
        let dz = sub(rdf(rp + 8), rdf(a1 + 8));
        let flat = add(mul(dy, dy), mul(dx, dx));
        let limit = if (mode & 8) != 0 { GATE_ONE } else { GATE_SIX };
        let mut run_slot = true;
        if (mode & 4) == 0 && !(flat >= limit) && !(absf(dz) > GATE_FOUR) {
            run_slot = false;
        }
        if run_slot {
            let slot: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(slot_obj) + VT_SLOT) as usize);
            slot(slot_obj, a0, a1, 0u32);
        }
        // Owner stamp.
        let fv: f32 = lf_checker_rt::callee_thiscall!(CALLEE_FLOAT, f32, this, a0, a1, 1u32);
        let cb: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(slot_obj) + VT_CB) as usize);
        cb(owner, fv.to_bits());
        wr32(owner + OWNER_BITS, rd32(owner + OWNER_BITS) | BIT_ADVANCED);
        wr32(owner + OWNER_ST0, GATE_FOUR.to_bits());
        wr32(owner + OWNER_ST1, ST1_NEG);
        let mut bits = rd32(owner + OWNER_BITS);
        if probe != 0 && w14 >= 2 {
            bits |= BIT_EXTRA;
            wr32(owner + OWNER_BITS, bits);
            return owner;
        }
        bits &= !BIT_EXTRA;
        wr32(owner + OWNER_BITS, bits);
        owner
    }
});
