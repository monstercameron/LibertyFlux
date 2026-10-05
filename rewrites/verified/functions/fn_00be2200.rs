// original: 0x00be2200 CTaskComplexUseMobilePhone::vf20

/// Mobile-phone task update: advance the call staging and animate the user.
///
/// `this` is the complex task, `ped` the pedestrian. The task keeps its
/// running subtask at `+0x08`, a phone handle at `+0x04`, flags at `+0x0C`
/// and `+0x28`..`+0x2B`, and an outcome slot that starts as the subtask and
/// is returned at the end.
///
/// The update runs in stages. First the subtask gate: when the staging flag
/// (bit `0x01` of `+0x0C` past the low bit) is set, the subtask must accept
/// the pedestrian or the update falls through; on acceptance a shared reset
/// runs and the update returns 0. Otherwise the subtask's type decides
/// whether the phone pose (four constants) is applied to the pedestrian.
/// A nearby-props marker and the water flag then decide whether a dial
/// target is attached to the phone handle through the task pool.
///
/// The middle either stages a call (index `0x0A`/`0x0B` by the hand flag at
/// `+0x2B`, scored through two lookup calls and a float threshold with
/// `comiss` unordered semantics) or, when unstaged, attaches a dial tone
/// through a cdecl table lookup and a pool task built from eight stack
/// words (the callee takes them with caller cleanup while the follow-up
/// consumes the residue, popping all eight). An uninitialised frame word is
/// pushed on one path; the contract runs with a zero stack fill and the
/// rewrite pushes 0 there.
///
/// The tail handles the in-call pedestrian (type `0x643` gains a flag at
/// `+0x270`) and, for a pedestrian in a vehicle, measures the phone-to-ear
/// vector through an out-pointer call: when its squared length is below a
/// quarter, or below four while the gauge reads exactly zero (the
/// `ucomiss`+`lahf` parity test exits on nonzero and NaN alike), a path
/// query runs with a frame
/// pointer argument and the result is blended in. Any failure lands in a
/// shared cleanup that touches `+0x10` and releases the staging. All float
/// arithmetic keeps the original's operand order.
///
/// Original: 0x00be2200 (thiscall, one stack word, returns the outcome slot
/// or 0).
lf_checker_rt::export!(thiscall, rw_00be2200(this: u32, ped: u32) -> u32 {
    unsafe {
        const SUBTASK: u32 = 0x08;
        const PHONE: u32 = 0x04;
        const SUB_STATUS: u32 = 0x0c;
        const SUB_STARTED: u32 = 0x02;
        const SUB_ALIVE: u8 = 0x01;
        const VT_START: u32 = 0x14;
        const VT_TYPE: u32 = 0x0c;
        const TASK_POOL: u32 = 0x167e2a0;
        const TYPE_A: u32 = 0x642;
        const TYPE_B: u32 = 0x643;
        const TYPE_C: u32 = 0x641;
        const TYPE_DIAL: u32 = 0x11d;
        const POSE_X: u32 = 0x1050d30;
        const POSE_Y: u32 = 0x1050d34;
        const POSE_Z: u32 = 0x1050d38;
        const POSE_W: u32 = 0x1050d3c;
        const PED_POSE: u32 = 0xbd0;
        const THRESH_HI: u32 = 0x167f5a0;
        const THRESH_LO: u32 = 0x10475bc;
        const THRESH_MID: u32 = 0x10475c0;
        const DIAL_TABLE: u32 = 0x1295cd8;
        const DIAL_VALUE: u32 = 0x12b4138;
        const PED_COUNT: u32 = 0x11d6fd4;
        const VT_MEASURE: u32 = 0x08;
        const VT_VECTOR: u32 = 0xec;
        const VT_POSE: u32 = 0xd0;
        const LEN_LO: u32 = 0xfe87e4;
        const LEN_HI: u32 = 0xfe8ab8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
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
        /// Virtual start hook on a subtask (mode selects the second word).
        unsafe fn start_sub(sub: u32, ped: u32, mode: u32) -> u8 {
            unsafe {
                let slot = rd32(rd32(sub) + VT_START);
                let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                (f(sub, ped, mode, 0) & 0xff) as u8
            }
        }
        /// Virtual type query on a task object.
        unsafe fn task_type(obj: u32) -> u32 {
            unsafe {
                let slot = rd32(rd32(obj) + VT_TYPE);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(obj)
            }
        }
        /// Fresh task from the pool, or 0 when empty.
        unsafe fn pool_task() -> u32 {
            unsafe {
                let pool = rd32(lf_checker_rt::relocated(TASK_POOL));
                lf_checker_rt::callee_thiscall!(1, u32, pool)
            }
        }
        /// Build a pool task from eight stack words (site selects the blend
        /// factor and row), then consume the residue. Returns the
        /// follow-up's result. `task` is the pool result, passed on in ecx;
        /// site 2 re-pushes the staged index as its second word.
        unsafe fn build_call(site2: bool, ped: u32, task: u32, staged: u32) -> u32 {
            unsafe {
                let (blend, row, vtable): (u32, u32, u32) = if site2 {
                    (0x4080_0000, 0x642, lf_checker_rt::relocated(0xeb8a98))
                } else {
                    (0x447a_0000, 0x643, lf_checker_rt::relocated(0xeb8a8c))
                };
                let extra = if site2 { staged } else { 0x0cu32 };
                let r = lf_checker_rt::callee_cdecl!(
                    10, u32, ped, extra, blend, row, vtable, 0, 0x3f80_0000, 0
                );
                lf_checker_rt::callee_thiscall!(
                    11, u32, task, r, extra, blend, row, vtable, 0, 0x3f80_0000, 0
                )
            }
        }

        let sub8 = rd32(this + SUBTASK);
        let mut slot10 = sub8;
        let mut slot18 = 0u32;
        let mut slot1c = 0u32;
        // Entry gate.
        if (rd32(this + 0x0c) >> 1) & 1 != 0 {
            slot18 = sub8;
            let mut accepted = true;
            if rd8(sub8 + SUB_STATUS) & SUB_ALIVE == 0 {
                if start_sub(sub8, ped, 1) == 0 {
                    accepted = false;
                } else {
                    wr32(sub8 + SUB_STATUS, rd32(sub8 + SUB_STATUS) | SUB_STARTED);
                }
            }
            if accepted {
                lf_checker_rt::callee_stdcall!(2, u32, ped);
                return 0;
            }
        }
        // Subtask type dispatch.
        if sub8 != 0 {
            let mut matched = false;
            if task_type(sub8) == TYPE_A {
                matched = true;
            } else if task_type(sub8) == TYPE_B {
                matched = true;
            } else if task_type(sub8) == TYPE_C {
                matched = true;
            }
            if matched {
                let slot = rd32(rd32(ped) + VT_POSE);
                let pose: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(slot as usize);
                pose(ped);
                wr32(ped + PED_POSE, rd32(lf_checker_rt::relocated(POSE_X)));
                wr32(ped + PED_POSE + 4, rd32(lf_checker_rt::relocated(POSE_Y)));
                wr32(ped + PED_POSE + 8, rd32(lf_checker_rt::relocated(POSE_Z)));
                wr32(ped + PED_POSE + 12, rd32(lf_checker_rt::relocated(POSE_W)));
            }
        }
        // Nearby-props marker.
        let prop = rd32(ped + 0x228);
        if prop != 0 {
            let mark = prop.wrapping_add(0x70);
            if mark != 0 {
                wr8(mark.wrapping_add(0x415), 1);
            }
        }
        // Dial target attach.
        if rd8(ped + 0x219) != 0 {
            let handle = rd32(this + PHONE);
            if handle != 0 && task_type(handle) == TYPE_DIAL {
                let inner = rd32(handle + 0x14);
                if task_type(inner) != 2 && task_type(inner) != 0x3ae {
                    let task = pool_task();
                    let av = if task == 0 {
                        0
                    } else {
                        lf_checker_rt::callee_thiscall!(3, u32, task)
                    };
                    lf_checker_rt::callee_thiscall!(4, u32, handle, av);
                }
            }
        }
        wr32(ped + 0x29c, rd32(ped + 0x29c) | 0x80);
        // Middle: staged call, or dial tone when unstaged.
        let staged = if rd8(this + 0x2b) != 0 { 0x0bu32 } else { 0x0au32 };
        slot18 = staged;
        if rd8(this + 0x2a) == 0 {
            if rd8(this + 0x28) == 0 {
                let dial = rd32(lf_checker_rt::relocated(DIAL_VALUE));
                let idx = rd32(this + 0x24);
                let ok =
                    lf_checker_rt::callee_cdecl!(12, u32, idx, dial) & 0xff != 0;
                if ok {
                    let tab = lf_checker_rt::relocated(DIAL_TABLE);
                    let ent = rd32(tab.wrapping_add(idx.wrapping_mul(4)));
                    lf_checker_rt::callee_thiscall!(13, u32, ent);
                    wr8(this + 0x2a, 1);
                    slot1c = sub8;
                    let mut started = true;
                    if rd8(sub8 + SUB_STATUS) & SUB_ALIVE == 0 {
                        if start_sub(sub8, ped, 1) == 0 {
                            started = false;
                        } else {
                            wr32(sub8 + SUB_STATUS, rd32(sub8 + SUB_STATUS) | SUB_STARTED);
                        }
                    }
                    if started {
                        let task = pool_task();
                        slot1c = task;
                        slot10 = if task == 0 { 0 } else { build_call(true, ped, task, staged) };
                    }
                } else {
                    let dial = rd32(lf_checker_rt::relocated(DIAL_VALUE));
                    lf_checker_rt::callee_cdecl!(14, u32, rd32(this + 0x24), dial, 0x18);
                }
            }
        } else {
            let m1 = lf_checker_rt::callee_stdcall!(5, u32, staged);
            slot18 = m1;
            let m2 = lf_checker_rt::callee_stdcall!(28, u32, 0x0c);
            if m1 == 0 {
                if m2 != 0 {
                    let x = f32::from_bits(rd32(m2 + 0x4c));
                    slot1c = x.to_bits();
                    let cc = f32::from_bits(rd32(lf_checker_rt::relocated(THRESH_MID)));
                    if x >= cc {
                        let slot = rd32(rd32(m2) + VT_MEASURE);
                        let measure: extern "thiscall" fn(u32) -> f32 =
                            core::mem::transmute(slot as usize);
                        let m = measure(m2);
                        slot18 = m.to_bits();
                        if cc > sub(x, m) {
                            lf_checker_rt::callee_stdcall!(2, u32, ped);
                        }
                    }
                } else {
                    let al = lf_checker_rt::callee_thiscall!(9, u32, this + 0x18) & 0xff;
                    if al != 0 || rd8(this + 0x29) != 0 {
                        slot1c = sub8;
                        let mut started = true;
                        if rd8(sub8 + SUB_STATUS) & SUB_ALIVE == 0 {
                            if start_sub(sub8, ped, 2) == 0 {
                                started = false;
                            } else {
                                wr32(sub8 + SUB_STATUS, rd32(sub8 + SUB_STATUS) | SUB_STARTED);
                            }
                        }
                        if started {
                            let task = pool_task();
                            slot1c = task;
                            slot10 = if task == 0 { 0 } else { build_call(false, ped, task, staged) };
                        }
                    }
                }
            } else {
                let thr = if rd8(this + 0x2b) != 0 {
                    rd32(lf_checker_rt::relocated(THRESH_HI))
                } else {
                    rd32(lf_checker_rt::relocated(THRESH_LO))
                };
                slot18 = thr;
                let f: f32 = lf_checker_rt::callee_thiscall!(6, f32, m1);
                slot1c = f.to_bits();
                let t = f32::from_bits(thr);
                // jae continues; below-or-unordered consults the hand flag.
                if f >= t || rd8(this + 0x2b) != 0 {
                    if rd32(ped + 0x2c4) == 0 {
                        lf_checker_rt::callee_thiscall!(7, u32, ped + 0x2b0, 0x2e, 1);
                        let idx = rd32(this + 0x24);
                        let ok = lf_checker_rt::callee_thiscall!(8, u32, ped + 0x2b0, ped, idx, 0);
                        if ok != 0 {
                            let p = rd32(ped + 0x2c4);
                            wr32(p + 0x210, rd32(p + 0x210) | 0x400_0000);
                        }
                    }
                }
            }
        }
        // In-call flagging.
        if rd8(ped + 0x218) == 0 && rd8(ped + 0x219) != 0 && sub8 != 0 {
            if task_type(sub8) == TYPE_B {
                let ac = lf_checker_rt::callee_thiscall!(15, u32, ped);
                if ac != 0 {
                    let ok = lf_checker_rt::callee_thiscall!(16, u32, ac.wrapping_add(0x26c8))
                        & 0xff;
                    if ok != 0 {
                        wr32(ped + 0x270, rd32(ped + 0x270) | 0x8000_0000);
                    }
                }
            }
        }
        // Vehicle branch.
        if rd8(ped + 0x218) == 0
            && rd8(ped + 0x219) != 0
            && (rd32(lf_checker_rt::relocated(PED_COUNT)) as i32) >= 1
        {
            slot18 = 0x180;
            let mut ok = rd8(ped + 0x26c) & 4 != 0;
            let veh = rd32(ped + 0xb30);
            if ok {
                ok = veh != 0 && rd32(veh + 0x1304) == 1;
            }
            if ok {
                ok = lf_checker_rt::callee_stdcall!(17, u32, ped) & 0xff != 0;
            }
            if ok {
                let mut out = 0u32;
                let slot = rd32(rd32(veh) + VT_VECTOR);
                let vec_of: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                let q = vec_of(veh, &mut out as *mut u32 as u32);
                let x = f32::from_bits(rd32(q));
                let y = f32::from_bits(rd32(q + 4));
                let z = f32::from_bits(rd32(q + 8));
                let x2 = mul(x, x);
                let y2 = mul(y, y);
                let z2 = mul(z, z);
                let mut len = add(x2, y2);
                len = add(len, z2);
                let lo = f32::from_bits(rd32(lf_checker_rt::relocated(LEN_LO)));
                if lo > len {
                    ok = true;
                } else {
                    // ucomiss+lahf+test+jp exits unless the gauge reads
                    // exactly zero: the parity test passes for ordered
                    // nonzero and unordered alike, and fails only on equal.
                    let w = f32::from_bits(rd32(veh + 0x1078));
                    let hi = f32::from_bits(rd32(lf_checker_rt::relocated(LEN_HI)));
                    ok = w == 0.0 && hi > len;
                }
            }
            if ok {
                let idx = rd16(veh + 0x2e) as i16 as i32 as u32;
                let tab = lf_checker_rt::relocated(DIAL_TABLE);
                let ent = rd32(tab.wrapping_add(idx.wrapping_mul(4)));
                let frame = [slot18, slot1c];
                let r = lf_checker_rt::callee_cdecl!(
                    18,
                    u32,
                    rd32(ent + 0xc4),
                    frame.as_ptr() as u32,
                    ped,
                    veh,
                    0,
                    0,
                    0,
                    1
                );
                if r as i32 != -1 {
                    lf_checker_rt::callee_thiscall!(
                        19, u32, this, ped, r, slot18, 0x4100_0000, 2
                    );
                    return slot10;
                }
            }
            return release_call(this, slot10);
        }
        slot10
    }
});

/// Shared cleanup of the vehicle branch: mark `+0x10` and release staging.
#[inline(never)]
unsafe fn release_call(this: u32, slot10: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let o = rd32(this + 0x10);
        if o != 0 {
            wr32(o + 4, rd32(o + 4) | 0x40_0000);
        }
        lf_checker_rt::callee_thiscall!(20, u32, this, 0xc100_0000);
        slot10
    }
}
