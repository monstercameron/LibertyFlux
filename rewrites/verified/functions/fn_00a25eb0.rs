// original: 0x00a25eb0 ped_task_speed_update (proposed)

/// Refresh a ped's task-driven speed value and return whether it is active.
///
/// `a1` is the ped object (gated on the word at `+0xd68`, task list at
/// `+0x224`), `a2` points to the speed float, `a3` to a factor float.
/// The routine queries a helper for two scratch values, runs the speed and
/// the first scratch value through a shaping callee, picks a blend constant
/// (one of two equal 0.1 values, depending on whether the factor is zero),
/// finds the ped's task of type 0x41e, and then either clamps the task's
/// limit field, runs a final three-argument shaping call whose result
/// replaces the speed, or does nothing, based on the task's kind field, two
/// more queries, and the magnitude of the factor.
///
/// Only the low byte of the return value is meaningful (1 active, 0 idle);
/// the upper bytes are leftover register contents. As a side effect the
/// original also clobbers the high byte of its own incoming first-argument
/// slot with a 0/1 flag; the rewrite cannot address that slot, so the
/// contract runs with the stack comparison off (see `narrowed`).
///
/// Original: 0x00a25eb0 (stdcall, three stack arguments).
lf_checker_rt::export!(stdcall, rw_00a25eb0(a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const GATE: u32 = 0xd68;
        const TASK_LIST: u32 = 0x224;
        const TASK_TYPE: u32 = 0x41e;
        const KIND: u32 = 0x38;
        const LIMIT: u32 = 0x4c;
        const KIND_DIRECT: u32 = 0x11;
        const KIND_QUERY: u32 = 0x10;
        const C_ZERO: u32 = 0x00fe8628;
        const C_PICK_A: u32 = 0x0103c770;
        const C_PICK_B: u32 = 0x0103c76c;
        const C_LIMIT: u32 = 0x0103c778;
        const C_SMALL: u32 = 0x0103c774;
        const C_MAG: u32 = 0x00fe879c;
        const C_SIGNMASK: u32 = 0x00fe8fa0;

        #[inline(always)]
        unsafe fn rd32(base: u32, off: u32) -> u32 {
            unsafe { ((base + off) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(base: u32, off: u32, v: u32) {
            unsafe { ((base + off) as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read_unaligned() }
        }

        if rd32(a1, GATE) == 0 {
            return 0;
        }
        // Scratch pair plus a copy of the first argument, as the original
        // passes pointers to its own frame slots (compared by skip + effect).
        let mut slot0: u32 = 0;
        let mut slot1: u32 = 0;
        let mut arg_copy: u32 = a1;
        let ok = lf_checker_rt::callee_stdcall!(
            1, u32, a1,
            &mut slot0 as *mut u32 as u32,
            &mut slot1 as *mut u32 as u32,
            &mut arg_copy as *mut u32 as u32
        );
        if (ok & 0xff) == 0 {
            return 0;
        }
        let _ = slot1;
        let _ = arg_copy;
        // Shape the speed and the helper's first scratch value.
        let r1: f32 = lf_checker_rt::callee_cdecl!(2, f32, rd32(a2, 0));
        let r1bits = r1.to_bits();
        wr32(a2, 0, r1bits);
        let r2: f32 = lf_checker_rt::callee_cdecl!(3, f32, slot0);
        let r2bits = r2.to_bits();
        // Blend pick: the zero-comparison constant selects between the two.
        let factor = f32::from_bits(rd32(a3, 0));
        let is_zero = factor == f32::from_bits(g32(C_ZERO));
        let mut picked = if is_zero { g32(C_PICK_B) } else { g32(C_PICK_A) };
        // Find the task; a miss keeps the shaped speed and returns active.
        let found = lf_checker_rt::callee_thiscall!(
            4, u32, rd32(a1, TASK_LIST).wrapping_add(0x44), TASK_TYPE
        );
        if found == 0 {
            return 1;
        }
        let mut flag: u8 = 0;
        let limit = f32::from_bits(rd32(found, LIMIT));
        let cap = f32::from_bits(g32(C_LIMIT));
        // Ordered `cap > limit` continues; unordered (NaN) skips like `jbe`.
        if cap > limit {
            let usable = lf_checker_rt::callee_thiscall!(5, u32, found, a1);
            if (usable & 0xff) != 0 {
                let kind = rd32(found, KIND);
                if kind == 3 || kind == 5 || kind == 0x0f {
                    let x = f32::from_bits(rd32(a3, 0));
                    flag = 1;
                    // Absolute value through the sign mask (taken only for
                    // negative ordered values, matching the jbe structure).
                    let ax = if 0.0 > x {
                        f32::from_bits(x.to_bits() ^ g32(C_SIGNMASK))
                    } else {
                        x
                    };
                    let mag = f32::from_bits(g32(C_MAG));
                    if ax > mag {
                        wr32(found, LIMIT, g32(C_LIMIT));
                        flag = 0;
                    } else {
                        picked = g32(C_SMALL);
                    }
                }
            }
        }
        let kind2 = rd32(found, KIND);
        let run_final = if kind2 == KIND_DIRECT {
            true
        } else if kind2 == KIND_QUERY {
            // The original jumps to the final block when the query answers
            // zero and consults the flag otherwise.
            let q = lf_checker_rt::callee_cdecl!(6, u32,);
            (q & 0xff) == 0 || flag != 0
        } else {
            flag != 0
        };
        if run_final {
            let r6: f32 = lf_checker_rt::callee_cdecl!(7, f32, r1bits, r2bits, picked);
            wr32(a2, 0, r6.to_bits());
        }
        1
    }
});
