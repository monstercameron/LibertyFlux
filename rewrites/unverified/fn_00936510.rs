// original: 0x00936510 net_state_poll (proposed)

/// Poll the two network state slots and fold their contribution into a pair
/// of running totals.
///
/// `arg0`'s low byte is an override flag. Callee 1 (no arguments) reports
/// whether slot data is available; its low byte set means both slots are
/// live. The running totals start at zero unless the first state word is 2
/// or 3, the second state word is 2 or 3, the override or callee 1 allows
/// slot 0, and the flag byte allows slot 1, in which case they start at
/// (-1, -1) with weights (2, 2) taken from the read-only constants.
///
/// The loop then visits the three state words at `STATE0 + k*STEP`
/// (`k = 0, 1, 2`, signed bound). A word of 2 or 3 whose slot byte is
/// non-zero (slot 0 uses the override-derived byte, slot 1 the flag-derived
/// byte, slot 2 the stack-fill byte, always zero) asks callee 2 for a
/// triple of words into a stack buffer and forwards two of them, with the
/// current first total, to callee 3 (object at 0xD64 below the state word).
/// A word of 5 calls callee 4 with the two current lane values instead.
/// Either call folds the weights into the totals (`t0 += w0`, `t1 += w1`,
/// in the original's operand order); any other word value skips the fold.
/// Returns whatever is left in eax: the last state word read, or the last
/// callee's answer when the final iteration calls.
///
/// Original: 0x00936510 (cdecl, one stack word, only its low byte read).
/// The original realigns its stack frame (`and esp, -16`); the rewrite uses
/// a normal frame, which the stack-pointer check accepts because both are
/// net zero. Two reads hit stack the original never wrote (a padding byte
/// beside the saved registers, forwarded as callee 3's first argument, and
/// slot 2's byte): both are the contract's zero stack fill, hard-coded here.
lf_checker_rt::export!(cdecl, rw_00936510(arg0: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x011A_2EA2;
        const STATE0: u32 = 0x011A_4024;
        const STATE1: u32 = 0x011A_4DD4;
        const STATE_END: u32 = 0x011A_5B84;
        const STEP: u32 = 0xDB0;
        const OBJ_BACK: u32 = 0xD64;
        const CAL_READY: u32 = 1;
        const CAL_TRIPLE: u32 = 2;
        const CAL_FOLD2: u32 = 3;
        const CAL_FOLD5: u32 = 4;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let base = lf_checker_rt::relocated(STATE0);
        let end = lf_checker_rt::relocated(STATE_END);

        // Callee 1 answers in its low byte; the original's fallback bytes
        // beside it are explicitly zeroed, so a zero answer means (0, 0).
        let ready: u32 = lf_checker_rt::callee_cdecl!(CAL_READY, u32,);
        let live: u32 = if (ready as u8) != 0 { 1 } else { 0 };
        let slot0: u32 = if (arg0 as u8) != 0 { 1 } else { live };
        let slot1: u32 = if rd8(lf_checker_rt::relocated(FLAG)) == 0 {
            0
        } else {
            live
        };

        let neg_one = rdf(lf_checker_rt::relocated(0x00FE_8D94));
        let two = rdf(lf_checker_rt::relocated(0x00FE_8A24));
        let mut tot0 = 0.0f32;
        let mut tot1 = 0.0f32;
        let (mut w0, mut w1) = (0.0f32, 0.0f32);
        let mut lane1 = 0.0f32;
        let head = rd32(base);
        if (head == 3 || head == 2) && slot0 != 0 {
            let other = rd32(lf_checker_rt::relocated(STATE1));
            if (other == 3 || other == 2) && slot1 != 0 {
                tot0 = neg_one;
                tot1 = neg_one;
                w0 = two;
                w1 = two;
                lane1 = neg_one;
            }
        }
        let mut lane0 = tot0;

        // Triple buffer for callee 2, zeroed exactly once like the
        // original's (its frame holds the zero fill at the first call and
        // the previous scripted words afterwards).
        let mut triple = [0u32; 3];
        let mut ptr = base;
        let mut lane = 0u32;
        // The original returns whatever is left in eax: each iteration
        // loads the state word, and every callee answer overwrites it, so
        // when the last iteration calls, the answer is the return value.
        let mut eax = slot0;
        while (ptr as i32) < (end as i32) {
            let v = rd32(ptr);
            eax = v;
            let mut fold = false;
            if v == 3 || v == 2 {
                let b = if lane == 0 {
                    slot0
                } else if lane == 1 {
                    slot1
                } else {
                    0
                };
                if b != 0 {
                    let r2: u32 = lf_checker_rt::callee_cdecl!(
                        CAL_TRIPLE,
                        u32,
                        triple.as_mut_ptr() as u32
                    );
                    eax = r2;
                    let obj = ptr.wrapping_sub(OBJ_BACK);
                    let r3: u32 = lf_checker_rt::callee_thiscall!(
                        CAL_FOLD2,
                        u32,
                        obj,
                        0u32,
                        tot0.to_bits(),
                        1u32,
                        triple[0],
                        triple[1]
                    );
                    eax = r3;
                    fold = true;
                }
            } else if v == 5 {
                let obj = ptr.wrapping_sub(OBJ_BACK);
                let r4: u32 = lf_checker_rt::callee_thiscall!(
                    CAL_FOLD5,
                    u32,
                    obj,
                    lane1.to_bits(),
                    lane0.to_bits()
                );
                eax = r4;
                fold = true;
            }
            if fold {
                tot1 = add(w0, tot1);
                tot0 = add(w1, tot0);
                lane1 = tot1;
                lane0 = tot0;
            }
            ptr = ptr.wrapping_add(STEP);
            lane += 1;
        }
        eax
    }
});
