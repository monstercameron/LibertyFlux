// original: 0x00cf8050 CTaskComplexClimbLadder::vf20 (stage A, see results.json)

/// Advance one climb-ladder task tick (stage A: entry, dispatch, ladder
/// event, direction solve and tail; the ladder-angle computation is out
/// of scope, see below).
///
/// `task` is the complex task (`+0x98` accumulated time, `+0xC` state
/// flags, `+0x8` current sub-task, `+0x14` direction mode, `+0x40` the
/// ladder frame, `+0x70` facing) and `ped` the climbing ped.
/// Every tick adds the frame delta to the task timer and dispatches on
/// the sub-task's type: an odd tick flag runs the finish query and
/// returns 0; type `0x11D` with no child returns the child; `0x386`
/// marks the ped and returns the child once the climb counter reaches 2;
/// `0x120` remaps the child's stage from the task's counters; `0x387`
/// and the fall-throughs raise the ladder event; `0x3AE` validates the
/// mount (object query, ped checks, 9.0 distance gate) and either raises
/// the event or starts climb task `0x387`. The event path posts a ladder
/// event unless the event count is below 30, in which case the direction
/// solve runs: sine/cosine of the facing pick a side vector, the ped
/// matrix yields goal offsets, two flag tests (height band 2.5 when the
/// mode is 4; unit goal circle when the mode is 1 and the mount is valid)
/// set a proceed flag, and the goal is handed to the mount query. Skipped
/// mounts, missing mount targets and mode 3 (which runs a byte maze over
/// the target instead) all rejoin the tail: a clear flag returns the
/// child, otherwise a final virtual check (or the tick flag) decides
/// between returning the child and running the finish query for 0.
///
/// The ladder-angle computation (stick read, double arctangent, angle
/// wrap and flag test) is NOT implemented here: it calls a helper taking
/// two doubles in vector registers and returning a double the same way,
/// which this checker's callee scripts cannot express (4-byte vector
/// transports only, 32-bit float answers only). Stage-A inputs never
/// reach it; the branch is an explicit trap stating the gap so a later
/// lane continues from a passing stage instead of starting again.
///
/// Original: 0x00CF8050 (thiscall; stack arg is the ped).
lf_checker_rt::export!(thiscall, rw_00cf8050(task: u32, ped: u32) -> u32 {
    unsafe {
        const FRAME_DT: u32 = 0x11735BC;
        const TICK: u32 = 0x11735B4;
        const CLIMB_COUNT: u32 = 0x11D6FD4;
        const TYPE_TABLE: u32 = 0x1295CD8;
        const EVENT_LIMIT: u32 = 0xE9D7FC;
        const DIST_GATE: u32 = 0xFE8B00;
        const SIGN_MASK: u32 = 0xFE8FA0;
        const ABS_MASK: u32 = 0xFE8F80;
        const HEIGHT_BAND: u32 = 0xFE8A60;
        const Z_GATE: u32 = 0xFE8830;
        const UNIT_ONE: u32 = 0xFE88E8;
        const VF_TYPE: u32 = 0x0C;
        const VF_FINAL: u32 = 0x14;
        const CLIMB_TASK: u32 = 0x387;
        const CAL_FINISH: u32 = 1;
        const CAL_MOUNT_Q: u32 = 2;
        const CAL_MOUNT_OK: u32 = 3;
        const CAL_PED_Q: u32 = 4;
        const CAL_START: u32 = 5;
        const CAL_EV_NEW: u32 = 6;
        const CAL_EV_POST: u32 = 7;
        const CAL_EV_FREE: u32 = 8;
        const CAL_PED9: u32 = 9;
        const CAL_SIN: u32 = 10;
        const CAL_COS: u32 = 11;
        const CAL_AB0Q: u32 = 12;
        const CAL_GOAL: u32 = 13;
        const CAL_TARGET: u32 = 14;

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
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
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
        /// Task-type query through the virtual slot (planted in the check).
        #[inline(always)]
        unsafe fn v_type(obj: u32) -> u32 {
            unsafe {
                let vtbl = (obj as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(vtbl.wrapping_add(VF_TYPE)) as usize);
                f(obj)
            }
        }
        /// The ladder event (original's 0xCF822D block): post the event
        /// unless the count is below the limit, then solve the direction.
        #[inline(always)]
        unsafe fn ladder_event(task: u32, ped: u32) -> u32 {
            unsafe {
                let e1 = rd32(ped.wrapping_add(0x224));
                let e2 = rd32(e1.wrapping_add(0x264));
                if (e2 as i32) < rd32(lf_checker_rt::relocated(EVENT_LIMIT)) as i32 {
                    return direction_solve(task, ped);
                }
                let type3 = v_type(task);
                let mut ev = [0u32; 8];
                let evp = ev.as_mut_ptr() as u32;
                let _: u32 = lf_checker_rt::callee_thiscall!(CAL_EV_NEW, u32, evp, type3);
                let e3 = rd32(ped.wrapping_add(0x224)).wrapping_add(0x84);
                let _: u32 = lf_checker_rt::callee_thiscall!(CAL_EV_POST, u32, e3, evp, 0, 1);
                let child2 = rd32(task.wrapping_add(0x08));
                let _: u32 = lf_checker_rt::callee_thiscall!(CAL_EV_FREE, u32, evp);
                child2
            }
        }
        /// The direction solve and tail (see the export's doc comment).
        #[inline(always)]
        unsafe fn direction_solve(task: u32, ped: u32) -> u32 {
            unsafe {
                let child = rd32(task.wrapping_add(0x08));
                let a80 = rd32(ped.wrapping_add(0xA80));
                wr32(a80.wrapping_add(0x50), rd32(a80.wrapping_add(0x50)) | 0x100);
                let _: u32 = lf_checker_rt::callee_thiscall!(CAL_PED9, u32, ped, 1);
                let ang = rdf(task.wrapping_add(0x70));
                let sin: f32 = f32::from_bits(lf_checker_rt::callee_cdecl!(
                    CAL_SIN,
                    u32,
                    ang.to_bits()
                ));
                let sign_lo = rd32(lf_checker_rt::relocated(SIGN_MASK));
                let negsin = f32::from_bits(sin.to_bits() ^ sign_lo);
                let cos: f32 = f32::from_bits(lf_checker_rt::callee_cdecl!(
                    CAL_COS,
                    u32,
                    ang.to_bits()
                ));
                let mode = rd32(task.wrapping_add(0x14));
                let v40 = rdf(task.wrapping_add(0x40));
                let v44 = rdf(task.wrapping_add(0x44));
                let v48 = rdf(task.wrapping_add(0x48));
                let (f40, f44) = if mode == 1 {
                    (add(v40, negsin), add(v44, cos))
                } else {
                    (sub(v40, negsin), sub(v44, cos))
                };
                let mut flag: u32 = 0;
                let mat20 = rd32(ped.wrapping_add(0x20));
                if mode == 4 {
                    let t = f32::from_bits(
                        sub(v48, rdf(mat20.wrapping_add(0x38))).to_bits()
                            & rd32(lf_checker_rt::relocated(ABS_MASK)),
                    );
                    if t > rdf(lf_checker_rt::relocated(HEIGHT_BAND)) {
                        flag = 1;
                    }
                }
                let g10 = sub(rdf(mat20.wrapping_add(0x30)), f40);
                let g20 = sub(rdf(mat20.wrapping_add(0x34)), f44);
                let g30 = sub(rdf(mat20.wrapping_add(0x38)), v48);
                if mode == 1 {
                    let ab0 = rd32(ped.wrapping_add(0xAB0));
                    if ab0 != 0 {
                        let r4b: u32 = lf_checker_rt::callee_thiscall!(CAL_AB0Q, u32, ab0);
                        if r4b & 0xFF != 0 {
                            let idx = ((ab0.wrapping_add(0x2E)) as *const u16).read_unaligned();
                            if (idx as i16) >= 0 {
                                let entry = (lf_checker_rt::relocated(TYPE_TABLE)
                                    .wrapping_add((idx as u32).wrapping_mul(4))
                                    as *const u32)
                                    .read_unaligned();
                                if rd32(entry.wrapping_add(0x40)) & 0x80000 == 0 {
                                    if g30 > rdf(lf_checker_rt::relocated(Z_GATE)) {
                                        let s = add(mul(g20, g20), mul(g10, g10));
                                        if rdf(lf_checker_rt::relocated(UNIT_ONE)) > s {
                                            flag = 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                let fr = [f40, f44, v48];
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    CAL_GOAL,
                    u32,
                    ped,
                    fr.as_ptr() as u32
                );
                if rd8(ped.wrapping_add(0x218)) != 0 {
                    return tail(task, ped, child, flag);
                }
                if rd8(ped.wrapping_add(0x219)) == 0 {
                    return tail(task, ped, child, flag);
                }
                let a2: u32 = lf_checker_rt::callee_thiscall!(CAL_TARGET, u32, ped);
                if a2 == 0 {
                    return tail(task, ped, child, flag);
                }
                if a2 == lf_checker_rt::relocated(0x1185C08) {
                    return tail(task, ped, child, flag);
                }
                if rd32(task.wrapping_add(0x14)) == 3 {
                    if byte_maze(a2) {
                        if flag & 0xFF == 0 {
                            return child;
                        }
                        return tail_check(task, ped, child);
                    }
                    return tail_check(task, ped, child);
                }
                // STAGE-A BOUNDARY (checker_gap): the ladder-angle
                // computation calls a helper taking two doubles in xmm0/1
                // and returning a double in xmm0. The checker's vector
                // transports move 4 bytes and its float answers mirror 32
                // bits, so neither the call nor its result is scriptable.
                // Stage-A inputs never reach here.
                unreachable!("checker_gap: xmm-double callee (angle path)");
            }
        }
        /// Byte maze over the mount target: true = check the flag first.
        #[inline(always)]
        unsafe fn byte_maze(a2: u32) -> bool {
            unsafe {
                let dl = rd8(a2.wrapping_add(0x26CC));
                if rd8(a2.wrapping_add(0x26CE)) ^ dl > 0x7F {
                    if rd8(a2.wrapping_add(0x26CF)) ^ dl <= 0x7F {
                        return false;
                    }
                }
                let dl2 = rd8(a2.wrapping_add(0x26BC));
                if rd8(a2.wrapping_add(0x26BE)) ^ dl2 > 0x7F {
                    if rd8(a2.wrapping_add(0x26BF)) ^ dl2 <= 0x7F {
                        return false;
                    }
                }
                true
            }
        }
        /// Tail shared by the flag-check and maze exits.
        #[inline(always)]
        unsafe fn tail(task: u32, ped: u32, child: u32, flag: u32) -> u32 {
            unsafe {
                if flag & 0xFF == 0 {
                    return child;
                }
                tail_check(task, ped, child)
            }
        }
        /// Final virtual check / finish query.
        #[inline(always)]
        unsafe fn tail_check(task: u32, ped: u32, child: u32) -> u32 {
            unsafe {
                if rd8(task.wrapping_add(0x0C)) & 1 != 0 {
                    let _: u32 = lf_checker_rt::callee_stdcall!(CAL_FINISH, u32, ped);
                    return 0;
                }
                let vtbl = (task as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(vtbl.wrapping_add(VF_FINAL)) as usize);
                let r = f(task, ped, 1, 0);
                if r & 0xFF == 0 {
                    return child;
                }
                wr32(task.wrapping_add(0x0C), rd32(task.wrapping_add(0x0C)) | 2);
                let _: u32 = lf_checker_rt::callee_stdcall!(CAL_FINISH, u32, ped);
                0
            }
        }

        // Timer accumulate (global first, as the addss issues it).
        let tick_t = add(
            rdf(lf_checker_rt::relocated(FRAME_DT)),
            rdf(task.wrapping_add(0x98)),
        );
        wrf(task.wrapping_add(0x98), tick_t);
        // Odd tick flag: finish query, return 0.
        if (rd32(task.wrapping_add(0x0C)) >> 1) & 1 != 0 {
            let _: u32 = lf_checker_rt::callee_stdcall!(CAL_FINISH, u32, ped);
            return 0;
        }
        let ped224 = rd32(ped.wrapping_add(0x224));
        wr32(
            ped224.wrapping_add(0x28C),
            rd32(lf_checker_rt::relocated(TICK)),
        );
        let child = rd32(task.wrapping_add(0x08));
        if v_type(child) == 0x11D {
            if rd32(child.wrapping_add(0x14)) == 0 {
                return child;
            }
        }
        let type2 = v_type(child);
        if type2 == 0x386 {
            if (rd32(lf_checker_rt::relocated(CLIMB_COUNT)) as i32) < 2 {
                return child;
            }
            let a80 = rd32(ped.wrapping_add(0xA80));
            wr32(a80.wrapping_add(0x50), rd32(a80.wrapping_add(0x50)) | 0x100);
            return child;
        }
        if type2 == 0x120 {
            let c = rd32(task.wrapping_add(0x7C));
            if c == 0 {
                return child;
            }
            if rd8(task.wrapping_add(0x76)) == 0 {
                if c == 2 {
                    wr32(child.wrapping_add(0x20), 4);
                    return child;
                }
                wr32(child.wrapping_add(0x20), if c == 3 { 5 } else { 2 });
                return child;
            }
            wr32(
                child.wrapping_add(0x20),
                if (c as i32) >= 2 { 3 } else { 1 },
            );
            return child;
        }
        if type2 == CLIMB_TASK {
            return ladder_event(task, ped);
        }
        if type2 == 0x3AE {
            let be = lf_checker_rt::callee_thiscall!(CAL_MOUNT_Q, u32, child, ped);
            if be == 0 {
                return ladder_event(task, ped);
            }
            let d2: u32 = lf_checker_rt::callee_thiscall!(CAL_MOUNT_OK, u32, be);
            if d2 == 2 {
                let r49: u32 = lf_checker_rt::callee_thiscall!(CAL_PED_Q, u32, child, ped, 1, 0);
                if r49 & 0xFF != 0 {
                    let ans: u32 =
                        lf_checker_rt::callee_thiscall!(CAL_START, u32, task, CLIMB_TASK, ped);
                    return ans;
                }
            }
            if rd8(task.wrapping_add(0x77)) != 0 {
                return ladder_event(task, ped);
            }
            let bec = be;
            if (rd32(bec.wrapping_add(0xD8)) >> 9) & 1 == 0 {
                return ladder_event(task, ped);
            }
            let cnt = rd32(bec.wrapping_add(0x70));
            if cnt == 0 {
                return ladder_event(task, ped);
            }
            if (rd32(cnt) as i32) <= 0 {
                return ladder_event(task, ped);
            }
            let dx = sub(rdf(cnt.wrapping_add(0x10)), rdf(task.wrapping_add(0x40)));
            let dy = sub(rdf(cnt.wrapping_add(0x14)), rdf(task.wrapping_add(0x44)));
            let dz = sub(rdf(cnt.wrapping_add(0x18)), rdf(task.wrapping_add(0x48)));
            let dist2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
            if !(dist2 > rdf(lf_checker_rt::relocated(DIST_GATE))) {
                return ladder_event(task, ped);
            }
            let r49b: u32 = lf_checker_rt::callee_thiscall!(CAL_PED_Q, u32, child, ped, 1, 0);
            if r49b & 0xFF == 0 {
                return ladder_event(task, ped);
            }
            wr8(task.wrapping_add(0x77), 1);
            let ans: u32 =
                lf_checker_rt::callee_thiscall!(CAL_START, u32, task, CLIMB_TASK, ped);
            return ans;
        }
        child
    }
});
