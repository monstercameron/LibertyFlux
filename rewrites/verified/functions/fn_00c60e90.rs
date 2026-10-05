// original: 0x00C60E90 vehicle_grenade_throw_tick (proposed)

/// Advance one tick of the "throw grenade from vehicle" task for a ped.
///
/// `this` is the task object: flag bits at `+0x0c` (bit 0 started, bit 1
/// armed, bit 2 accepted), the throw target at `+0x10`, a timer at `+0x20`,
/// the pin-pulled latch at `+0x24`, a state word at `+0x28` (0 pick the
/// grenade, 1 aim and throw, anything else idle), the item and its extra at
/// `+0x2c`/`+0x30`, the ready latch at `+0x34` and a second latch at `+0x35`.
/// `ped` is the throwing ped.
///
/// Behaviour: unless the ped is already resisting in a vehicle with the
/// task armed, offer the task to the ped's current routine first (through
/// the task's own function table); a refusal, or an unstarted task, reports
/// done. Otherwise mark the ped tasked, raise the global grenade-active
/// flag and run the state machine:
/// - state 0 takes the grenade item, makes sure the ped's weapon slot is
///   valid and hands the throw to the ped.
/// - state 1 first runs the target animation, then, while the ped sits
///   properly, reads the input device the same way the arrest task does
///   (two button-code pairs, each accepted at or below `0x7f`); when the
///   game mode allows grenades and the ped holds one, it blends the aim:
///   the weapon's throw distance (or a default), mirrored for left-hand
///   throws, rotated by the throw angle's sine and cosine, scattered by two
///   calibrated direction probes, scaled by the ped's throw power and
///   finally offset by the vehicle's position and the aim direction. The
///   resulting arc is checked against the world; on a hit the weapon is
///   hidden, ammo is made sure of and the task reports done, otherwise it
///   keeps aiming.
/// Returns 1 while the task still has work for the next tick, 0 when the
/// ped is handed off or the throw can no longer happen.
///
/// Original: 0x00C60E90 (thiscall, one stack word: the ped pointer; callee
/// pops it). Two stack slots the original reads without writing feed only
/// dead stores below the stack pointer, so the rewrite omits them (see the
/// lane report); the frame pointers handed to intercepted callees are
/// skipped in the contract, with a snapshot for the one whose contents the
/// throw check reads. Float comparisons keep the original's unordered
/// (NaN) behaviour, written as `!(a > b)`.
lf_checker_rt::export!(thiscall, rw_00c60e90(this: u32, ped: u32) -> u8 {
    unsafe {
        const TASK_FLAGS: u32 = 0x0c;
        const TASK_TARGET: u32 = 0x10;
        const TASK_TIMER: u32 = 0x20;
        const TASK_PINPULLED: u32 = 0x24;
        const TASK_STATE: u32 = 0x28;
        const TASK_ITEM: u32 = 0x2c;
        const TASK_ITEM2: u32 = 0x30;
        const TASK_READY: u32 = 0x34;
        const TASK_ARMED: u32 = 0x35;
        const FLAG_STARTED: u8 = 0x01;
        const FLAG_ARMED: u32 = 0x02;
        const FLAG_ACCEPTED: u32 = 0x02;
        const PED_SLOT: u32 = 0x20;
        const PED_CHILD: u32 = 0x6c;
        const PED_SEAT1: u32 = 0x218;
        const PED_SEAT2: u32 = 0x219;
        const PED_RESIST: u32 = 0x26c;
        const PED_TASKMARK: u32 = 0x2a0;
        const PED_WEAPONS: u32 = 0x2b0;
        const PED_VOICE2: u32 = 0x3c0;
        const PED_VEH: u32 = 0xb30;
        const RESIST_BIT: u8 = 0x04;
        const TASKMARK_BIT: u32 = 0x10;
        const TICK_ADD: u32 = 0x11735bc;
        const W_THROW_DIST: u32 = 0xfe8ad8; // 5.0
        const ARC_BIAS: u32 = 0xfe8960; // 1.5
        const ARC_STEP: u32 = 0xfe876c; // 0.05
        const DOT_FLOOR: u32 = 0xfe8628; // 0.0
        const ARC_UNIT: u32 = 0xfe88e8; // 1.0
        const VEH_ORG_X: u32 = 0x128e320;
        const VEH_ORG_Y: u32 = 0x128e324;
        const VEH_ORG_Z: u32 = 0x128e328;
        const VEH_OFF_X: u32 = 0x128e330;
        const VEH_OFF_Y: u32 = 0x128e334;
        const VEH_OFF_Z: u32 = 0x128e338;
        const VEH_TABLE: u32 = 0x1295cd8;
        const GAME_MODE: u32 = 0x11d6fd4;
        const GRENADE_ACTIVE: u32 = 0x11db239;
        const SAY_PIN: u32 = 0xecb1bc;
        const BUTTON_LIMIT: u8 = 0x7f;
        const THROW_SLOT: u32 = 4;
        const ARMED_KIND: u32 = 0x24;
        const FINISH_BLEND: f32 = -8.0;

        const C_BEGIN: u32 = 1;
        const C_INPUT: u32 = 2;
        const C_INPUTMODE: u32 = 3;
        const C_ARMEDKIND: u32 = 4;
        const C_ANIM: u32 = 5;
        const C_SETCLIP: u32 = 6;
        const C_WEAPONMGR: u32 = 7;
        const C_THROWABLE: u32 = 8;
        const C_HASH: u32 = 9;
        const C_SAY: u32 = 10;
        const C_CANTHROW: u32 = 11;
        const C_SEED: u32 = 12;
        const C_AIMPOS: u32 = 13;
        const C_AIMDIR: u32 = 14;
        const C_WINFO: u32 = 15;
        const C_THROWMODE: u32 = 16;
        const C_ARCPARAM: u32 = 17;
        const C_SIN: u32 = 18;
        const C_COS: u32 = 19;
        const C_SCATTER: u32 = 21;
        const C_POWER: u32 = 22;
        const C_TRAJECTORY: u32 = 23;
        const C_HIDEWEAPON: u32 = 24;
        const C_AMMO: u32 = 25;
        const C_GIVEWEAPON: u32 = 26;
        const C_FINISH: u32 = 27;
        const C_SUBTASK: u32 = 28;
        const C_FORCEEQUIP: u32 = 29;
        const C_FINDITEM: u32 = 30;
        const C_SETUPTHROW: u32 = 31;
        const VT_SLOT_TASK: u32 = 0x14;
        const VT_SLOT_VEH: u32 = 0xec;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16sx(a: u32) -> i32 {
            unsafe { (a as *const i16).read_unaligned() as i32 }
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
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn neg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ 0x8000_0000)
        }
        #[inline(always)]
        unsafe fn g32(file_va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(file_va)) }
        }
        #[inline(always)]
        unsafe fn gf(file_va: u32) -> f32 {
            unsafe { f32::from_bits(g32(file_va)) }
        }

        // Shared tail: run the sub-task step, report done.
        let epilogue1 = || -> u8 {
            unsafe {
                lf_checker_rt::callee_thiscall!(
                    C_SUBTASK, u32, this.wrapping_add(0x14), 0xffff_ffffu32
                );
            }
            1
        };
        // Shared step: clear the stale bit on the target link.
        let bit6clear = || {
            unsafe {
                let o = rd32(this.wrapping_add(TASK_TARGET));
                let w = rd32(o.wrapping_add(4));
                if (w >> 6) & 1 != 0 {
                    wr32(o.wrapping_add(4), w & !0x40);
                }
            }
        };
        // Shared tail: finish the throw when the ped can no longer aim.
        let path_14dc = |dl: u8| -> u8 {
            unsafe {
                if dl == 0 {
                    return 0;
                }
                lf_checker_rt::callee_thiscall!(C_FINISH, u32, this, FINISH_BLEND.to_bits());
            }
            epilogue1()
        };
        // Shared body: aim the throw from the weapon slot onward.
        let path_i21 = || -> u8 {
            unsafe {
                let b6 = lf_checker_rt::callee_thiscall!(
                    C_WEAPONMGR, u32, ped.wrapping_add(PED_WEAPONS)
                );
                let timer = rdf(this.wrapping_add(TASK_TIMER));
                let dl =
                    lf_checker_rt::callee_cdecl!(C_THROWABLE, u32, ped, timer.to_bits()) as u8;
                let o10 = rd32(this.wrapping_add(TASK_TARGET));
                if rd8(o10.wrapping_add(0x74)) & 1 != 0 && rd8(this.wrapping_add(TASK_PINPULLED)) == 0
                {
                    wr8(this.wrapping_add(TASK_PINPULLED), 1);
                    if b6 != 0 && rd32(b6.wrapping_add(0x18)) == THROW_SLOT {
                        let h = lf_checker_rt::callee_cdecl!(
                            C_HASH, u32, lf_checker_rt::relocated(SAY_PIN), 0
                        );
                        lf_checker_rt::callee_thiscall!(C_SAY, u32, ped.wrapping_add(PED_VOICE2), h);
                    }
                }
                let g = rd32(ped.wrapping_add(PED_CHILD));
                if g != 0 && rd8(g.wrapping_add(0xe)) != 0 {
                    let o10c = rd32(this.wrapping_add(TASK_TARGET));
                    if rd32(o10c.wrapping_add(0x74)) & 0x200 == 0 {
                        return 0;
                    }
                    wr8(this.wrapping_add(TASK_ARMED), 1);
                    return 0;
                }
                let o10d = rd32(this.wrapping_add(TASK_TARGET));
                if rd32(o10d.wrapping_add(0x74)) & 0x200 == 0 && dl == 0 {
                    return 0;
                }
                if b6 == 0 {
                    return path_14dc(dl);
                }
                let t = lf_checker_rt::callee_thiscall!(C_CANTHROW, u32, b6) as u8;
                if t == 0 {
                    return path_14dc(dl);
                }
                let r12 = lf_checker_rt::callee_cdecl!(C_SEED, u32, 0x4d0);
                let r13 = lf_checker_rt::callee_cdecl!(C_AIMPOS, u32, r12);
                let m4d = lf_checker_rt::callee_thiscall!(C_AIMDIR, u32, ped, r13);
                let wslot = rd32(b6.wrapping_add(0x18));
                let af1 = lf_checker_rt::callee_cdecl!(C_WINFO, u32, wslot);
                let base = if af1 == 0 {
                    gf(W_THROW_DIST)
                } else {
                    let af2 = lf_checker_rt::callee_cdecl!(
                        C_WINFO, u32, rd32(b6.wrapping_add(0x18))
                    );
                    rdf(af2.wrapping_add(0xd8))
                };
                let b30 = rd32(ped.wrapping_add(PED_VEH));
                let mode = lf_checker_rt::callee_thiscall!(C_THROWMODE, u32, b30, ped);
                let (mut f10, mut f20): (f32, f32);
                if mode == 1 {
                    let idx = rd16sx(b30.wrapping_add(0x2e));
                    let entry = rd32(
                        lf_checker_rt::relocated(VEH_TABLE)
                            .wrapping_add((idx as u32).wrapping_mul(4)),
                    );
                    if (rd32(entry.wrapping_add(0x94)) >> 5) & 1 == 0 {
                        f20 = neg(base);
                        f10 = 0.0;
                    } else {
                        f10 = neg(base);
                        f20 = 0.0;
                    }
                } else if mode == 0 {
                    f20 = neg(base);
                    f10 = 0.0;
                } else {
                    f20 = base;
                    f10 = 0.0;
                }
                let mut f18 = 0.0f32;
                // The original copies one uninitialized stack word through
                // two dead stores here; it is never observed (below the
                // stack pointer on both sides), so the rewrite omits it.
                let s = f32::from_bits(lf_checker_rt::callee_thiscall!(C_ARCPARAM, u32, b30));
                let sin = f32::from_bits(lf_checker_rt::callee_cdecl!(C_SIN, u32, s.to_bits()));
                let cos = f32::from_bits(lf_checker_rt::callee_cdecl!(C_COS, u32, s.to_bits()));
                let t1 = mul(f10, sin);
                let t4 = mul(f20, sin);
                let t5 = mul(f20, cos);
                let t3 = mul(f10, cos);
                f20 = sub(t5, t1);
                f10 = add(t3, t4);
                let scatter =
                    (g32(GAME_MODE) as i32) >= 2 && rd32(b6.wrapping_add(0x18)) == ARMED_KIND;
                let (mut x, mut y, mut z);
                if scatter {
                    let b30b = rd32(ped.wrapping_add(PED_VEH));
                    let mut f34 = gf(ARC_BIAS);
                    if rd32(b30b.wrapping_add(0x1304)) == 2 {
                        let k = gf(ARC_STEP);
                        let v3 = add(mul(gf(VEH_OFF_X), k), gf(VEH_ORG_X));
                        let v2 = add(mul(gf(VEH_OFF_Y), k), gf(VEH_ORG_Y));
                        let v1 = add(mul(gf(VEH_OFF_Z), k), gf(VEH_ORG_Z));
                        let vt: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                            rd32(rd32(b30b).wrapping_add(VT_SLOT_VEH)) as usize,
                        );
                        vt(b30b, 0);
                        // The three blended offsets are stored to dead stack
                        // slots (overwritten by the probes below); keep the
                        // computation for faithfulness, drop the stores.
                        let _ = (v3, v2, v1);
                        let mut o1 = [0u32; 4];
                        let mut o2 = [0u32; 4];
                        lf_checker_rt::callee_thiscall!(C_SCATTER, u32, o1.as_mut_ptr() as u32);
                        lf_checker_rt::callee_thiscall!(C_SCATTER, u32, o2.as_mut_ptr() as u32);
                        let a = [
                            f32::from_bits(o1[0]),
                            f32::from_bits(o1[1]),
                            f32::from_bits(o1[2]),
                        ];
                        let b = [
                            f32::from_bits(o2[0]),
                            f32::from_bits(o2[1]),
                            f32::from_bits(o2[2]),
                        ];
                        let d0 = mul(b[1], a[1]);
                        let d1 = mul(b[0], a[0]);
                        let d2 = mul(b[2], a[2]);
                        let mut dot = add(d0, d1);
                        dot = add(dot, d2);
                        f10 = a[1];
                        f20 = a[0];
                        f18 = a[2];
                        f34 = if !(dot > gf(DOT_FLOOR)) {
                            gf(ARC_UNIT)
                        } else {
                            add(mul(dot, gf(ARC_BIAS)), gf(ARC_UNIT))
                        };
                        let p = f32::from_bits(lf_checker_rt::callee_cdecl!(C_POWER, u32, ped));
                        f20 = mul(f20, p);
                        f10 = mul(f10, p);
                        f18 = mul(f18, p);
                    }
                    let b30c = rd32(ped.wrapping_add(PED_VEH));
                    let vt2: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                        rd32(rd32(b30c).wrapping_add(VT_SLOT_VEH)) as usize,
                    );
                    let r = vt2(b30c, 0);
                    let (mut xx, mut yy, mut zz) = (
                        rdf(r),
                        rdf(r.wrapping_add(4)),
                        rdf(r.wrapping_add(8)),
                    );
                    zz = mul(zz, f34);
                    xx = mul(xx, f34);
                    zz = add(zz, f18);
                    yy = mul(yy, f34);
                    x = xx;
                    y = yy;
                    z = zz;
                } else {
                    let b30d = rd32(ped.wrapping_add(PED_VEH));
                    let vt3: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                        rd32(rd32(b30d).wrapping_add(VT_SLOT_VEH)) as usize,
                    );
                    let r = vt3(b30d, 0);
                    x = rdf(r);
                    y = rdf(r.wrapping_add(4));
                    z = rdf(r.wrapping_add(8));
                }
                y = add(y, f10);
                x = add(x, f20);
                let q1 = rdf(m4d.wrapping_add(0x34));
                let q0 = rdf(m4d.wrapping_add(0x38));
                x = add(x, rdf(m4d.wrapping_add(0x30)));
                let q1 = add(q1, y);
                let q0 = add(q0, z);
                let frame = [x.to_bits(), q1.to_bits(), q0.to_bits()];
                let br = lf_checker_rt::callee_thiscall!(
                    C_TRAJECTORY, u32, b6, ped, m4d, m4d.wrapping_add(0x30),
                    frame.as_ptr() as u32, 1, 0, 0, 0, (-1.0f32).to_bits()
                ) as u8;
                if br != 0 {
                    lf_checker_rt::callee_thiscall!(
                        C_HIDEWEAPON, u32, ped.wrapping_add(PED_WEAPONS), ped, 1, 0
                    );
                }
                let w0 = rd32(ped.wrapping_add(PED_WEAPONS));
                let ammo = lf_checker_rt::callee_thiscall!(
                    C_AMMO, u32, ped.wrapping_add(PED_WEAPONS), w0
                );
                if ammo == 0 {
                    lf_checker_rt::callee_thiscall!(C_GIVEWEAPON, u32, ped);
                }
                wr8(this.wrapping_add(TASK_READY), 1);
                path_14dc(dl)
            }
        };

        // Entry gate: a resisting ped in a vehicle with the task armed goes
        // straight to the state machine; otherwise offer the task onward.
        let mut main = false;
        if rd8(ped.wrapping_add(PED_RESIST)) & RESIST_BIT != 0
            && rd32(ped.wrapping_add(PED_VEH)) != 0
            && (rd32(this.wrapping_add(TASK_FLAGS)) >> 1) & 1 == 0
        {
            main = true;
        }
        if !main {
            if rd8(this.wrapping_add(TASK_FLAGS)) & FLAG_STARTED != 0 {
                return 1;
            }
            let offer: extern "thiscall" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(
                rd32(rd32(this).wrapping_add(VT_SLOT_TASK)) as usize,
            );
            if offer(this, ped, 2, 0) as u8 != 0 {
                wr32(
                    this.wrapping_add(TASK_FLAGS),
                    rd32(this.wrapping_add(TASK_FLAGS)) | FLAG_ACCEPTED,
                );
                return 1;
            }
        }
        wr32(
            ped.wrapping_add(PED_TASKMARK),
            rd32(ped.wrapping_add(PED_TASKMARK)) | TASKMARK_BIT,
        );
        wr8(lf_checker_rt::relocated(GRENADE_ACTIVE), 1);
        wr8(this.wrapping_add(TASK_ARMED), 0);
        let state = rd32(this.wrapping_add(TASK_STATE));
        if state == 0 {
            let d = lf_checker_rt::callee_thiscall!(
                C_SUBTASK, u32, this.wrapping_add(0x14), rd32(this.wrapping_add(TASK_ITEM))
            ) as u8;
            if d == 0 {
                return 0;
            }
            wr32(this.wrapping_add(TASK_STATE), 1);
            let b6b = lf_checker_rt::callee_thiscall!(
                C_WEAPONMGR, u32, ped.wrapping_add(PED_WEAPONS)
            );
            if b6b == 0 {
                lf_checker_rt::callee_thiscall!(
                    C_FORCEEQUIP, u32, ped.wrapping_add(PED_WEAPONS), ped, 0xffff_ffffu32, 0
                );
            }
            let sc = rd32(this.wrapping_add(TASK_ITEM));
            if sc != 0xffff_ffffu32 {
                let c = lf_checker_rt::callee_cdecl!(
                    C_FINDITEM, u32, sc, rd32(this.wrapping_add(TASK_ITEM2))
                );
                if c != 0 {
                    lf_checker_rt::callee_thiscall!(
                        C_SETUPTHROW, u32, this, ped, sc,
                        rd32(this.wrapping_add(TASK_ITEM2)), 4.0f32.to_bits(), 1
                    );
                }
            }
            if rd32(this.wrapping_add(TASK_TARGET)) == 0 {
                return epilogue1();
            }
            return 0;
        }
        if state != 1 {
            return 0;
        }
        if rd32(this.wrapping_add(TASK_TARGET)) == 0 {
            return epilogue1();
        }
        lf_checker_rt::callee_thiscall!(C_BEGIN, u32, this, ped);
        let timer = add(rdf(this.wrapping_add(TASK_TIMER)), gf(TICK_ADD));
        wrf(this.wrapping_add(TASK_TIMER), timer);
        let seated = rd8(ped.wrapping_add(PED_SEAT1)) == 0 && rd8(ped.wrapping_add(PED_SEAT2)) != 0;
        if !seated {
            let g = rd32(ped.wrapping_add(PED_CHILD));
            if g == 0 {
                bit6clear();
                return path_i21();
            }
            if rd8(g.wrapping_add(0xe)) == 0 {
                bit6clear();
                return path_i21();
            }
            if rd8(this.wrapping_add(TASK_READY)) == 0 {
                return path_i21();
            }
            bit6clear();
            return path_i21();
        }
        let a2 = lf_checker_rt::callee_thiscall!(C_INPUT, u32, ped);
        let b30 = rd32(ped.wrapping_add(PED_VEH));
        let a4 = lf_checker_rt::callee_thiscall!(C_INPUTMODE, u32, b30, ped) as u8;
        if a2 != 0 {
            let ok = if a4 != 0 {
                (rd8(a2.wrapping_add(0x28fe)) ^ rd8(a2.wrapping_add(0x28fc))) <= BUTTON_LIMIT
            } else {
                (rd8(a2.wrapping_add(0x26de)) ^ rd8(a2.wrapping_add(0x26dc))) <= BUTTON_LIMIT
                    && (rd8(a2.wrapping_add(0x28fe)) ^ rd8(a2.wrapping_add(0x28fc)))
                        <= BUTTON_LIMIT
            };
            if ok {
                if (g32(GAME_MODE) as i32) >= 2 {
                    let armed = lf_checker_rt::callee_thiscall!(
                        C_ARMEDKIND, u32, ped.wrapping_add(PED_WEAPONS)
                    );
                    if armed == ARMED_KIND {
                        bit6clear();
                        let esi2 = rd32(this.wrapping_add(TASK_TARGET));
                        let f4c = rdf(esi2.wrapping_add(0x4c));
                        lf_checker_rt::callee_thiscall!(
                            C_ANIM, u32, esi2, rd32(esi2.wrapping_add(0x40))
                        );
                        if rdf(esi2.wrapping_add(0x68)) > f4c {
                            lf_checker_rt::callee_thiscall!(
                                C_ANIM, u32, esi2, rd32(esi2.wrapping_add(0x40))
                            );
                            let f68b = rdf(esi2.wrapping_add(0x68));
                            lf_checker_rt::callee_thiscall!(
                                C_SETCLIP, u32, rd32(this.wrapping_add(TASK_TARGET)),
                                f68b.to_bits()
                            );
                        }
                        return path_i21();
                    }
                }
                bit6clear();
                return path_i21();
            }
        }
        path_i21()
    }
});
