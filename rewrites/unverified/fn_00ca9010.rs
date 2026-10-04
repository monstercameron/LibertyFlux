// original: 0x00ca9010 CEventHandler::vf19
/// Answer a general event by kind: ten arms sharing one pool allocator,
/// most of them building a specialised task, with a virtual fallback for
/// unknown kinds.
///
/// `handler` points to the event handler (`+0x04` holds its ped, `+0x08`
/// and `+0x0c` receive tasks on different paths). `event` is the event
/// record (`+0x10` is the kind, `+0x18` the target, always non-null in
/// live trials). The second and third stack arguments are not read.
///
/// Kinds: 0x19f builds a timed task unless a ducking check vetoes it;
/// 0x1ab builds one unconditionally; 0x2c4 builds a scream task from a
/// ped word; 0x398 builds a face-entity task from two constants; 0x38f
/// splits on a target mode mask (set: combat thread plus flag bit, clear:
/// smart-flee thread plus tag byte) and both sides join a shared react
/// tail; 0x3fe runs a long validation (mode mask, ped flag, weapon info,
/// range check against the target distance) and builds a driveby task;
/// 0x2d6 and 0x76c share the combat path (optional weapon survey with a
/// three-way mode pick, then the same react tail). Anything else calls
/// the handler's virtual slot `+0x13c` with the kind, target and event
/// and returns its value with nothing stored. Kind 0xc8 stores zero and
/// returns the caller's leftover return register, so live trials never
/// send it. Three paths fault on null (two flag writes, one power read):
/// both sides fault identically there.
///
/// Original: 0x00ca9010 (thiscall, three stack words; the second and third
/// are not read).
lf_checker_rt::export!(thiscall, rw_00ca9010(handler: u32, event: u32, _a2: u32, _a3: u32) -> u32 {
    unsafe {
        const HANDLER_PED: u32 = 0x04;
        const HANDLER_ALT: u32 = 0x08;
        const HANDLER_TASK: u32 = 0x0c;
        const EVT_KIND: u32 = 0x10;
        const EVT_TARGET: u32 = 0x18;
        const KIND_C8: u32 = 0xc8;
        const KIND_19F: u32 = 0x19f;
        const KIND_1AB: u32 = 0x1ab;
        const KIND_2C4: u32 = 0x2c4;
        const KIND_2D6: u32 = 0x2d6;
        const KIND_38F: u32 = 0x38f;
        const KIND_398: u32 = 0x398;
        const KIND_3FE: u32 = 0x3fe;
        const KIND_76C: u32 = 0x76c;
        const VT_FALLBACK: u32 = 0x13c;
        const TGT_POS: u32 = 0x20;
        const POS_ARG: u32 = 0x30;
        const POS_X: u32 = 0x30;
        const POS_Y: u32 = 0x34;
        const POS_Z: u32 = 0x38;
        const TGT_MODE: u32 = 0x28;
        const MODE_MASK: u32 = 0x3c0;
        const MODE_WANT: u32 = 0xc0;
        const TGT_FLAG: u32 = 0x219;
        const TGT_POWER: u32 = 0x228;
        const POWER_OFF: u32 = 0x70;
        const POWER_LEVEL: u32 = 0x1c;
        const INFO_SUB: u32 = 0x08;
        const INFO_MODE: u32 = 0x0c;
        const INFO_RANGE: u32 = 0x14;
        const INFO_WANT: u32 = 4;
        const PED_SCOPE: u32 = 0x224;
        const PED_DUCK: u32 = 0x26c;
        const DUCK_BIT: u32 = 4;
        const PED_B30: u32 = 0xb30;
        const B30_LINK: u32 = 0xf50;
        const PED_WQB: u32 = 0x21c;
        const WQB_STATE: u32 = 0x12c;
        const WQB_WANT: u32 = 2;
        const PED_WTABLE: u32 = 0x2b0;
        const PED_TAG: u32 = 0x370;
        const SCOPE_OFF: u32 = 0x44;
        const FIND_TYPE: u32 = 0x2c5;
        const POOL_GLOBAL: u32 = 0x0167e2a0;
        const SINGLETON: u32 = 0x0128aa90;
        const RESULT_OFF: u32 = 0x60;
        const RESULT_BIT38: u32 = 8;
        const TAG_OFF: u32 = 0x39;
        const FACE_C2: u32 = 0x00ed7e68;
        const FACE_C1: u32 = 0x00ed7e70;
        const FLEE_F0: u32 = 0x00eef940;
        const FLEE_C1: u32 = 0x00eef944;
        const FLEE_C2: u32 = 0x00eef948;
        const FLEE_F1: u32 = 0x00eef94c;
        const RANGE_PAD: u32 = 0x00fe8ad8;
        const V_FALLBACK: u32 = 1;
        const DUCKING: u32 = 2;
        const ALLOC_ARM: u32 = 3;
        const TIMED: u32 = 4;
        const SCREAM: u32 = 5;
        const FACE: u32 = 6;
        const COMBAT: u32 = 7;
        const REGISTER: u32 = 8;
        const SMARTFLEE: u32 = 9;
        const PROXIMITY: u32 = 10;
        const ALLOC_TAIL: u32 = 11;
        const BUILD_A: u32 = 12;
        const BUILD_B: u32 = 13;
        const FIND: u32 = 14;
        const WEAPON_INFO: u32 = 15;
        const VALIDATE: u32 = 16;
        const RANGE_CALL: u32 = 17;
        const DRIVEBY: u32 = 18;
        const POWER_LOOKUP: u32 = 19;
        const POWER_USE: u32 = 20;
        const SPOTTER: u32 = 21;

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
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        let own = rd32(handler + HANDLER_PED);
        let target = rd32(event + EVT_TARGET);
        if target == 0 {
            // Unreachable in live trials (see vf27): the exit value is the
            // caller's leftover return register. Contract pins non-null.
            return 0;
        }
        let kind = rd32(event + EVT_KIND);
        let k = kind as i32;
        // Fallback for unknown kinds.
        macro_rules! fallback {
            () => {{
                let fb: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(handler) + VT_FALLBACK) as usize);
                return fb(handler, kind, target, event);
            }};
        }
        if k > KIND_2D6 as i32 {
            if k > KIND_3FE as i32 {
                if kind != KIND_76C {
                    fallback!();
                }
            } else if kind == KIND_3FE {
                // Range-validated driveby arm.
                let masked = rd32(target + TGT_MODE) & MODE_MASK;
                if masked != MODE_WANT {
                    return masked;
                }
                if rd32(own + PED_DUCK) & DUCK_BIT == 0 {
                    return masked;
                }
                let b30 = rd32(own + PED_B30);
                if b30 == 0 {
                    return 0;
                }
                if own == rd32(b30 + B30_LINK) {
                    return b30;
                }
                let scope = rd32(own + PED_SCOPE);
                let found: u32 =
                    lf_checker_rt::callee_thiscall!(FIND, u32, scope + SCOPE_OFF, FIND_TYPE);
                if found == 0 {
                    return 0;
                }
                let ix = rd32(own + PED_WTABLE).wrapping_add(3).wrapping_mul(3);
                let w0 = rd32(own + ix.wrapping_mul(4) + PED_WTABLE);
                let info: u32 = lf_checker_rt::callee_cdecl!(WEAPON_INFO, u32, w0);
                if rd32(info + INFO_SUB) != 1 {
                    return info;
                }
                let ok: u32 = lf_checker_rt::callee_thiscall!(VALIDATE, u32, scope, target);
                if (ok as u8) != 0 {
                    return ok;
                }
                let info2: u32 = lf_checker_rt::callee_cdecl!(WEAPON_INFO, u32, w0);
                let aff = rdf(info2 + INFO_RANGE);
                let st: f32 = lf_checker_rt::callee_thiscall!(RANGE_CALL, f32, scope);
                let st5 = add(
                    st,
                    f32::from_bits(rd32(lf_checker_rt::relocated(RANGE_PAD))),
                );
                let range = if core::hint::black_box(aff) > core::hint::black_box(st5) {
                    aff
                } else {
                    st5
                };
                let posa = rd32(target + TGT_POS);
                let posb = rd32(own + TGT_POS);
                let dx = sub(rdf(posa + POS_X), rdf(posb + POS_X));
                let dy = sub(rdf(posa + POS_Y), rdf(posb + POS_Y));
                let dz = sub(rdf(posa + POS_Z), rdf(posb + POS_Z));
                let dist = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz)).sqrt();
                if !(core::hint::black_box(range) > core::hint::black_box(dist)) {
                    wr32(handler + HANDLER_TASK, 0);
                    return posa;
                }
                let tag = rd8(own + PED_TAG);
                let pool = rd32(lf_checker_rt::relocated(POOL_GLOBAL));
                let slot: u32 = lf_checker_rt::callee_thiscall!(ALLOC_ARM, u32, pool);
                if slot == 0 {
                    wr32(handler + HANDLER_TASK, 0);
                    return 0;
                }
                let task: u32 = lf_checker_rt::callee_thiscall!(
                    DRIVEBY,
                    u32,
                    slot,
                    target,
                    0,
                    range.to_bits(),
                    tag as u32,
                    8,
                    0
                );
                wr32(handler + HANDLER_TASK, task);
                return task;
            } else if kind.wrapping_sub(KIND_38F) == 0 {
                // Combat / smart-flee split arm.
                let pool = rd32(lf_checker_rt::relocated(POOL_GLOBAL));
                let join_eax: u32;
                if rd32(target + TGT_MODE) & MODE_MASK == MODE_WANT {
                    let slot: u32 = lf_checker_rt::callee_thiscall!(ALLOC_ARM, u32, pool);
                    if slot != 0 {
                        let fu: u32 =
                            lf_checker_rt::callee_thiscall!(COMBAT, u32, slot, target, 0);
                        wr32(handler + HANDLER_TASK, fu);
                        wr32(fu + RESULT_OFF, rd32(fu + RESULT_OFF) | RESULT_BIT38);
                    } else {
                        wr32(handler + HANDLER_TASK, 0);
                        wr32(RESULT_OFF, rd32(RESULT_OFF) | RESULT_BIT38);
                    }
                    join_eax =
                        lf_checker_rt::callee_thiscall!(REGISTER, u32, rd32(own + PED_SCOPE), target, 1);
                } else {
                    let slot: u32 = lf_checker_rt::callee_thiscall!(ALLOC_ARM, u32, pool);
                    if slot != 0 {
                        let f0 = rd32(lf_checker_rt::relocated(FLEE_F0));
                        let c1 = rd32(lf_checker_rt::relocated(FLEE_C1));
                        let c2 = rd32(lf_checker_rt::relocated(FLEE_C2));
                        let f1 = rd32(lf_checker_rt::relocated(FLEE_F1));
                        let task: u32 = lf_checker_rt::callee_thiscall!(
                            SMARTFLEE, u32, slot, target, 1, f0, c1, c2, f1, 0
                        );
                        wr32(handler + HANDLER_TASK, task);
                        ((task + TAG_OFF) as *mut u8).write(1);
                        join_eax = task;
                    } else {
                        wr32(handler + HANDLER_TASK, 0);
                        // Null slot: the tag write faults on address 0x39.
                        ((TAG_OFF) as *mut u8).write(1);
                        join_eax = 0;
                    }
                }
                if rd32(handler + HANDLER_ALT) != 0 {
                    return join_eax;
                }
                let prox: u32 = lf_checker_rt::callee_cdecl!(PROXIMITY, u32, own, event);
                if (prox as u8) == 0 {
                    return prox;
                }
                let slot: u32 = lf_checker_rt::callee_thiscall!(ALLOC_TAIL, u32, pool);
                if slot == 0 {
                    wr32(handler + HANDLER_ALT, 0);
                    return 0;
                }
                let part: u32 =
                    lf_checker_rt::callee_stdcall!(BUILD_A, u32, own, target, 0, 0);
                let task: u32 = lf_checker_rt::callee_thiscall!(
                    BUILD_B, u32, slot, 1, 1, part, target, 0, 0
                );
                wr32(handler + HANDLER_ALT, task);
                return task;
            } else if kind.wrapping_sub(KIND_38F).wrapping_sub(KIND_398 - KIND_38F) == 0 {
                // Face-entity arm.
                let pool = rd32(lf_checker_rt::relocated(POOL_GLOBAL));
                let slot: u32 = lf_checker_rt::callee_thiscall!(ALLOC_ARM, u32, pool);
                if slot == 0 {
                    wr32(handler + HANDLER_TASK, 0);
                    return 0;
                }
                let c2 = rd32(lf_checker_rt::relocated(FACE_C2));
                let c1 = rd32(lf_checker_rt::relocated(FACE_C1));
                let task: u32 =
                    lf_checker_rt::callee_thiscall!(FACE, u32, slot, target, c2, c1);
                wr32(handler + HANDLER_TASK, task);
                return task;
            } else {
                fallback!();
            }
        } else if kind == KIND_2D6 {
            // Shared with 0x76c below.
        } else if k > KIND_1AB as i32 {
            if kind != KIND_2C4 {
                fallback!();
            } else {
                let pool = rd32(lf_checker_rt::relocated(POOL_GLOBAL));
                let slot: u32 = lf_checker_rt::callee_thiscall!(ALLOC_ARM, u32, pool);
                if slot == 0 {
                    wr32(handler + HANDLER_TASK, 0);
                    return 0;
                }
                let task: u32 =
                    lf_checker_rt::callee_thiscall!(SCREAM, u32, slot, rd32(own + PED_B30), 0);
                wr32(handler + HANDLER_TASK, task);
                return task;
            }
        } else if kind == KIND_1AB {
            let pool = rd32(lf_checker_rt::relocated(POOL_GLOBAL));
            let slot: u32 = lf_checker_rt::callee_thiscall!(ALLOC_ARM, u32, pool);
            if slot == 0 {
                wr32(handler + HANDLER_TASK, 0);
                return 0;
            }
            let task: u32 =
                lf_checker_rt::callee_thiscall!(TIMED, u32, slot, 0, 0x98967f, 0xffff_ffff);
            wr32(handler + HANDLER_TASK, task);
            return task;
        } else if kind == KIND_C8 {
            // Unreachable in live trials: the exit value is the caller's
            // leftover return register. Contract never sends 0xc8.
            wr32(handler + HANDLER_TASK, 0);
            return 0;
        } else if kind == KIND_19F {
            let duck: u32 = lf_checker_rt::callee_thiscall!(DUCKING, u32, own);
            if (duck as u8) != 0 {
                return duck;
            }
            let pool = rd32(lf_checker_rt::relocated(POOL_GLOBAL));
            let slot: u32 = lf_checker_rt::callee_thiscall!(ALLOC_ARM, u32, pool);
            if slot == 0 {
                wr32(handler + HANDLER_TASK, 0);
                return 0;
            }
            let task: u32 =
                lf_checker_rt::callee_thiscall!(TIMED, u32, slot, 0, 0xbb8, 0xffff_ffff);
            wr32(handler + HANDLER_TASK, task);
            return task;
        } else {
            fallback!();
        }
        // Combat path (kinds 0x2d6 and 0x76c).
        let masked = rd32(target + TGT_MODE) & MODE_MASK;
        if masked != MODE_WANT {
            return masked;
        }
        if rd32(rd32(own + PED_WQB) + WQB_STATE) == WQB_WANT && rd8(target + TGT_FLAG) != 0
        {
            let block = rd32(target + TGT_POWER);
            let arg = if block == 0 { 0 } else { block + POWER_OFF };
            let power: u32 = lf_checker_rt::callee_thiscall!(POWER_LOOKUP, u32, arg);
            if power == 0 {
                let block2 = rd32(target + TGT_POWER);
                let p = if block2 == 0 { 0 } else { block2 + POWER_OFF };
                let level = rdf(p + POWER_LEVEL);
                if core::hint::black_box(level) > core::hint::black_box(0.0f32) {
                    let _: u32 =
                        lf_checker_rt::callee_thiscall!(POWER_USE, u32, target, 1, 0x3e8);
                    let ix = rd32(target + PED_WTABLE).wrapping_add(3).wrapping_mul(3);
                    let w0 = rd32(target + ix.wrapping_mul(4) + PED_WTABLE);
                    let info: u32 = lf_checker_rt::callee_cdecl!(WEAPON_INFO, u32, w0);
                    let sub = rd32(info + INFO_SUB);
                    let mode = rd32(info + INFO_MODE);
                    let (arg1, arg2) = if mode == INFO_WANT {
                        (0x1e, 0x3e8)
                    } else if sub == 3 && mode == 3 {
                        (0x16, 0x1770)
                    } else {
                        (0x1c, 0x3e8)
                    };
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        SPOTTER,
                        u32,
                        lf_checker_rt::relocated(SINGLETON),
                        rd32(target + TGT_POS) + POS_ARG,
                        arg1,
                        arg2
                    );
                }
            }
        }
        let pool = rd32(lf_checker_rt::relocated(POOL_GLOBAL));
        let slot: u32 = lf_checker_rt::callee_thiscall!(ALLOC_ARM, u32, pool);
        if slot != 0 {
            let fu: u32 = lf_checker_rt::callee_thiscall!(COMBAT, u32, slot, target, 0);
            wr32(handler + HANDLER_TASK, fu);
        } else {
            wr32(handler + HANDLER_TASK, 0);
        }
        let scope = rd32(own + PED_SCOPE);
        let regged: u32 = lf_checker_rt::callee_thiscall!(REGISTER, u32, scope, target, 1);
        if rd32(handler + HANDLER_ALT) != 0 {
            return regged;
        }
        let prox: u32 = lf_checker_rt::callee_cdecl!(PROXIMITY, u32, own, event);
        if (prox as u8) == 0 {
            return prox;
        }
        let slot2: u32 = lf_checker_rt::callee_thiscall!(ALLOC_TAIL, u32, pool);
        if slot2 == 0 {
            wr32(handler + HANDLER_ALT, 0);
            return 0;
        }
        let part: u32 = lf_checker_rt::callee_stdcall!(BUILD_A, u32, own, target, 0, 0);
        // This tail pushes (part, 1, 0); the 0x38f join pushes
        // (part, 1, 1): same shape, different first word.
        let task: u32 =
            lf_checker_rt::callee_thiscall!(BUILD_B, u32, slot2, 0, 1, part, target, 0, 0);
        wr32(handler + HANDLER_ALT, task);
        task
    }
});
