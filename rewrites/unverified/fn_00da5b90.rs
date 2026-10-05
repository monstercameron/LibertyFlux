// original: 0x00da5b90 CTaskComplexShockingEventGoto::vf19

/// Pick a goto target for a ped reacting to a shocking event: seed a random
/// range from two hashed values, derive a wait and a goal, then build the
/// child tasks that walk there.
///
/// `this` is the task object, `ped` the reacting ped (only `ped+0x570` is
/// passed on, to callee 1). Callee 1 is asked first with ten constant stack
/// words. Callees 2 and 3 hash `this+0x20` (each answer is multiplied by
/// `MAGIC` and the high word shifted right by 6); callee 4 answers a random
/// word of which the low 16 bits scale `FRAC` and the hash difference,
/// truncated to an integer with `cvttss2si` semantics, added to the first
/// hash and multiplied by 1000 into `this+0x70`. Callee 5 fills nothing the
/// function reads. The globals `STAMP` and `MANAGER` land in `this+0x74` and
/// the child-builder calls, `this+0x78` repeats the wait, and byte
/// `this+0x7c` is set to 1.
///
/// The manager is then asked three times (callees 6, 8, 10): a null answer
/// yields a null child or, for the third ask, an immediate 0 return.
/// Otherwise callee 7 builds the first child from `this+0x80` and the global
/// float `RATE` (handed at a negative offset from the passed pointer),
/// callee 9 builds the second with constant words, and callee 11 combines
/// both children into the returned task.
///
/// Original: 0x00da5b90 (thiscall, one stack word; returns callee 11's answer
/// or 0).
lf_checker_rt::export!(thiscall, rw_00da5b90(this: u32, ped: u32) -> u32 {
    unsafe {
        use core::arch::x86::{_mm_cvtt_ss2si, _mm_set_ss};

        const KIND: u32 = 0x20;
        const WAIT: u32 = 0x70;
        const STAMP_SLOT: u32 = 0x74;
        const WAIT_COPY: u32 = 0x78;
        const READY: u32 = 0x7c;
        const SPEED: u32 = 0x80;
        const MAGIC: u64 = 0x1062_4dd3;
        const FRAC: f32 = f32::from_bits(0x3800_0000); // ~3.05e-5
        const PER_MILLE: u32 = 1000;
        const STAMP: u32 = 0x0117_35b4;
        const RATE: u32 = 0x0105_0e84;
        const MANAGER: u32 = 0x0167_e2a0;
        const SEED_CALLEE: u32 = 1;
        const HASH1_CALLEE: u32 = 2;
        const HASH2_CALLEE: u32 = 3;
        const RAND_CALLEE: u32 = 4;
        const GOAL_CALLEE: u32 = 5;
        const MGR1_CALLEE: u32 = 6;
        const CHILD1_CALLEE: u32 = 7;
        const MGR2_CALLEE: u32 = 8;
        const CHILD2_CALLEE: u32 = 9;
        const MGR3_CALLEE: u32 = 10;
        const COMBINE_CALLEE: u32 = 11;

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
        /// High word of `MAGIC * v`, shifted right by 6 (`mul` + `shr`).
        #[inline(always)]
        fn hash6(v: u32) -> u32 {
            (((MAGIC * v as u64) >> 32) as u32) >> 6
        }

        let _: u32 = lf_checker_rt::callee_thiscall!(
            SEED_CALLEE,
            u32,
            ped.wrapping_add(0x570),
            0x00ee_f58c,
            0,
            0,
            0,
            0xffff_ffff,
            0,
            0,
            0x3f80_0000,
            0,
            0
        );
        let kind = rd32(this + KIND);
        let h1: u32 = lf_checker_rt::callee_cdecl!(HASH1_CALLEE, u32, kind);
        let first = hash6(h1);
        let h2: u32 = lf_checker_rt::callee_cdecl!(HASH2_CALLEE, u32, kind);
        let second = hash6(h2);
        let rnd: u32 = lf_checker_rt::callee_cdecl!(RAND_CALLEE, u32,);
        // (rand16 * FRAC) * (second - first), all in the original's order.
        let scaled = mul((rnd & 0xffff) as f32, FRAC);
        let diff = second.wrapping_sub(first) as i32 as f32;
        let picked = mul(scaled, diff);
        let wait = _mm_cvtt_ss2si(_mm_set_ss(picked)) as u32;
        let wait = wait.wrapping_add(first).wrapping_mul(PER_MILLE);
        wr32(this + WAIT, wait);

        let mut goal = [0u32; 4];
        let _: u32 =
            lf_checker_rt::callee_thiscall!(GOAL_CALLEE, u32, this + KIND, goal.as_mut_ptr() as u32);
        wr32(this + STAMP_SLOT, lf_checker_rt::global::<u32>(STAMP).read());
        wr32(this + WAIT_COPY, wait);
        ((this + READY) as *mut u8).write(1);

        let mgr = lf_checker_rt::global::<u32>(MANAGER).read();
        let obj1: u32 = lf_checker_rt::callee_thiscall!(MGR1_CALLEE, u32, mgr);
        let child1 = if obj1 == 0 {
            0
        } else {
            // The original passes a pointer 24 bytes past these two words.
            let mut buf = [0u32; 8];
            buf[0] = rdf(this + SPEED).to_bits();
            buf[1] = lf_checker_rt::global::<u32>(RATE).read();
            let arg = buf.as_mut_ptr() as u32 + 24;
            lf_checker_rt::callee_thiscall!(CHILD1_CALLEE, u32, obj1, arg)
        };
        let obj2: u32 = lf_checker_rt::callee_thiscall!(MGR2_CALLEE, u32, mgr);
        let child2 = if obj2 == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(CHILD2_CALLEE, u32, obj2, 0, 0x3ea8_f5c3, 0, 0, 0, 0x14)
        };
        let obj3: u32 = lf_checker_rt::callee_thiscall!(MGR3_CALLEE, u32, mgr);
        if obj3 == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(COMBINE_CALLEE, u32, obj3, child1, child2, 0, 0)
    }
});
