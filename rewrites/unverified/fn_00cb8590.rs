// original: 0x00CB8590 CTaskComplexMoveFollowPointRoute::vf5

/// Route-follow task poll: advance one route point and report completion.
///
/// `this` is the complex task (`+0x08` its owner object, `+0x34` a pointer to
/// the step counter, `+0x38` the current step), `a0`/`a1` are opaque words
/// passed through to the owner calls (`a1` null clears the step counter),
/// and `a2` is the current sub-task or null. When a sub-task is present and
/// the next step is still below the counter, the function asks the sub-task
/// for its move target (answer `1` selects slot `+0x14`, `0x38` slot `+0x18`,
/// anything else abandons the poll), checks the owner gates, and calls the
/// target's vector slot with a scratch word; the returned point must lie
/// within squared length `0.015625`. It then runs the owner's float and int
/// slots, feeds both answers plus the scratch address twice into a direct
/// helper, and on success runs a second direct call. The tail always runs:
/// it clears the counter when `a1` is null, returns 1 when the owner's flag
/// bit 0 is set, and otherwise asks the owner slot `+0x14`, setting flag
/// bit 1 and returning 1 on success or returning 0. Only the low byte of the
/// result is defined. Original: thiscall with three stack words.
lf_checker_rt::export!(thiscall, rw_00cb8590(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const THIS_OWNER: u32 = 0x08;
        const THIS_COUNT_PTR: u32 = 0x34;
        const THIS_STEP: u32 = 0x38;
        const SUB_VEC_A: u32 = 0x14;
        const SUB_VEC_B: u32 = 0x18;
        const OWNER_FLAGS: u32 = 0x0C;
        const VT_SUB_QUERY: u32 = 0x04;
        const VT_OWNER_GATE: u32 = 0x28;
        const VT_OWNER_NEXT: u32 = 0x30;
        const VT_VEC_POINT: u32 = 0xEC;
        const VT_NEXT_FLOAT: u32 = 0x20;
        const VT_NEXT_INT: u32 = 0x1C;
        const VT_OWNER_POLL: u32 = 0x14;
        const QUERY_A: u32 = 1;
        const QUERY_B: u32 = 0x38;
        const FLAG_DONE: u32 = 1;
        const FLAG_ADVANCED: u32 = 2;
        const MAX_LEN_SQ: f32 = 0.015625;
        const CALLEE_HELPER: u32 = 7;
        const CALLEE_FINISH: u32 = 8;

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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        if a2 != 0 {
            let step = rd32(this + THIS_STEP).wrapping_add(1);
            if (step as i32) < (rd32(rd32(this + THIS_COUNT_PTR)) as i32) {
                let q: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(a2) + VT_SUB_QUERY) as usize);
                let r = q(a2);
                let slot = if r == QUERY_A {
                    rd32(a2 + SUB_VEC_A)
                } else if r == QUERY_B {
                    rd32(a2 + SUB_VEC_B)
                } else {
                    0
                };
                let owner = rd32(this + THIS_OWNER);
                let gate: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(owner) + VT_OWNER_GATE) as usize);
                if (gate(owner) & 0xFF) != 0 {
                    let next: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(rd32(rd32(owner) + VT_OWNER_NEXT) as usize);
                    let slotb = next(owner);
                    if slotb != 0 && slot != 0 {
                        let point: extern "thiscall" fn(u32, u32) -> u32 =
                            core::mem::transmute(rd32(rd32(slot) + VT_VEC_POINT) as usize);
                        let mut scratch = [0u32; 4];
                        let sp = scratch.as_mut_ptr() as u32;
                        let rp = point(slot, sp);
                        let x = rdf(rp);
                        let y = rdf(rp + 4);
                        let z = rdf(rp + 8);
                        let lensq = add(add(mul(x, x), mul(y, y)), mul(z, z));
                        if MAX_LEN_SQ > lensq {
                            let flt: extern "thiscall" fn(u32) -> f32 =
                                core::mem::transmute(rd32(rd32(slotb) + VT_NEXT_FLOAT) as usize);
                            let fv = flt(slotb);
                            let int: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                                rd32(rd32(slotb) + VT_NEXT_INT) as usize,
                            );
                            let iv = int(slotb, sp);
                            let ok = lf_checker_rt::callee_cdecl!(
                                CALLEE_HELPER,
                                u32,
                                sp,
                                iv,
                                sp,
                                fv.to_bits(),
                            ) & 0xFF;
                            if ok != 0 {
                                lf_checker_rt::callee_thiscall!(
                                    CALLEE_FINISH, u32, owner, a0, 0u32, a2,
                                );
                            }
                        }
                    }
                }
            }
        }
        if a1 == 0 {
            wr32(rd32(this + THIS_COUNT_PTR), 0);
        }
        let owner = rd32(this + THIS_OWNER);
        if (rd8(owner + OWNER_FLAGS) & 1) == 0 {
            let poll: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(owner) + VT_OWNER_POLL) as usize);
            if (poll(owner, a0, a1, a2) & 0xFF) != 0 {
                wr32(owner + OWNER_FLAGS, rd32(owner + OWNER_FLAGS) | FLAG_ADVANCED);
                return 1;
            }
            return 0;
        }
        1
    }
});
