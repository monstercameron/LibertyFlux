// original: 0x00da5d00 CTaskComplexShockingEventHurryAway::vf19

/// React to a shocking event for a hurrying-away ped: either keep the current
/// behaviour (return 0) or steer toward a target point and pick a follow-up.
///
/// `this` is the task object, `ped` the reacting ped. The task reads a mode
/// flag at `+0x46`, a secondary state at `+0x48`, a behaviour id at `+0x20`
/// and a position at `+0x30` (three floats). The ped's target block is reached
/// through `ped+0x20`, with its own position at `+0x30` there.
///
/// Gate: when the flag is set but the secondary state is clear, or the flag
/// is clear but the state is set, or both are clear and the squared length of
/// the task position does not exceed `LEN2_LIMIT`, the function returns 0 and
/// changes nothing. (The length test is an unordered-aware `jbe`: NaN takes
/// the early return.)
///
/// Otherwise it asks callee 1 (with `this+0x20` and a four-float scratch
/// buffer) for a goal point, stores the ped-target-minus-goal delta into
/// `this+0x70` (three floats plus the scratch's fourth word), zeroes
/// `this+0x78` again, and calls callee 2. When the behaviour id is one of
/// `FLEE_IDS` the answer comes from callee 5 (`this`, `0x3ae`, `ped`).
/// For any other id the function reads the manager pointer from the global
/// `MANAGER`, asks callee 3 for a child object and hands it to callee 4 with
/// `this+0x20`; the slot at the child's `+0x70` is then set to `CHILD_HINT
/// and the child's address is returned. When callee 3 answers null the slot
/// store runs against address 0 and faults, exactly like the original.
///
/// Original: 0x00da5d00 (thiscall, one stack word; returns the child address,
/// callee 5's answer, or 0).
lf_checker_rt::export!(thiscall, rw_00da5d00(this: u32, ped: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x46;
        const STATE: u32 = 0x48;
        const KIND: u32 = 0x20;
        const POS: u32 = 0x30;
        const DELTA: u32 = 0x70;
        const CHILD_SLOT: u32 = 0x70;
        const CHILD_HINT: u32 = 0x5dc;
        const FLEE_ARG: u32 = 0x3ae;
        const LEN2_LIMIT: f32 = f32::from_bits(0x3d4c_cccd); // 0.05
        const FLEE_IDS: [u32; 5] = [0x1a, 0x0c, 0x17, 0x16, 0x12];
        const MANAGER: u32 = 0x0167_e2a0;
        const GOAL_CALLEE: u32 = 1;
        const AFTER_CALLEE: u32 = 2;
        const CHILD_CALLEE: u32 = 3;
        const SETUP_CALLEE: u32 = 4;
        const FLEE_CALLEE: u32 = 5;

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

        let flag = rd8(this + FLAG);
        let state = rd32(this + STATE);
        if flag != 0 {
            if state == 0 {
                return 0;
            }
        } else {
            if state != 0 {
                return 0;
            }
            // Squared length in the original's order: (x*x + y*y) + z*z.
            let x = rdf(this + POS);
            let y = rdf(this + POS + 4);
            let z = rdf(this + POS + 8);
            let len2 = add(add(mul(x, x), mul(y, y)), mul(z, z));
            // `comiss; jbe`: taken when below, equal, or unordered (NaN).
            if !(len2 > LEN2_LIMIT) {
                return 0;
            }
        }

        let base = this + KIND;
        let mut goal = [0u32; 4];
        let _: u32 =
            lf_checker_rt::callee_thiscall!(GOAL_CALLEE, u32, base, goal.as_mut_ptr() as u32);
        let target = rd32(ped + KIND);
        let dx = sub(rdf(target + POS), f32::from_bits(goal[0]));
        let dy = sub(rdf(target + POS + 4), f32::from_bits(goal[1]));
        let dz = sub(rdf(target + POS + 8), f32::from_bits(goal[2]));
        wrf(this + DELTA + 8, dz);
        wrf(this + DELTA, dx);
        wrf(this + DELTA + 4, dy);
        wr32(this + DELTA + 0x0c, goal[3]);
        wr32(this + DELTA + 8, 0);
        let _: u32 = lf_checker_rt::callee_cdecl!(AFTER_CALLEE, u32,);

        let kind = rd32(this + KIND);
        if FLEE_IDS.contains(&kind) {
            return lf_checker_rt::callee_thiscall!(FLEE_CALLEE, u32, this, FLEE_ARG, ped);
        }
        let mgr = lf_checker_rt::global::<u32>(MANAGER).read();
        let child: u32 = lf_checker_rt::callee_thiscall!(CHILD_CALLEE, u32, mgr);
        if child == 0 {
            // The original stores through the null answer and faults; the
            // volatile store keeps the compiler from turning it into a trap.
            core::ptr::write_volatile(CHILD_SLOT as *mut u32, CHILD_HINT);
            return 0;
        }
        let r: u32 = lf_checker_rt::callee_thiscall!(SETUP_CALLEE, u32, child, base);
        wr32(r + CHILD_SLOT, CHILD_HINT);
        r
    }
});
