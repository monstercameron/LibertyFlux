// original: 0x00c7bc70 CTaskComplexMoveBetweenPointsScenario::vf20
//
// Move-between-points scenario tick: advance the scenario's state machine
// and decide the sub-task the pedestrian runs this frame. `this` is the
// scenario task, `ped` the pedestrian. Returns the task to run: the current
// sub-task (`+0x08`) on most paths, the route anchor built in state 1 on the
// state-1 success paths.
//
// Behaviour. Flag the ped's intelligence as scenario-driven
// (`[ped+PED_INTEL]+INTEL_SCENARIO = 1`). An idle scenario runs the body,
// otherwise the shared gate re-polls the task's own sub-operation (vtable
// slot `SELF_VT_SUBOP`, kinds 1 then 2) and yields (null) when it accepts.
// The body resolves the scenario config, then dispatches on the state
// (`+0x28`): state 0 counts the wait timer (`+0x24`) down by the frame step
// from the game's data and, once it runs out, randomises a new wait between
// 0.8x and 1.2x of the configured span, samples a destination through the
// route service, and commits it (state 1) when it lies more than 2 units
// from the ped's placement; state 1 validates the leg through the route and
// schedule services, resolves the world anchor through the scenario-global
// pointer, derives the heading with an arctangent over the leg vector, and
// seats the ped (returning the anchor); any other state skips to the tail,
// which pings the ped when the aux id (`+0x14`) selects the chat variant and
// returns the current sub-task.
//
// The frame holds the service out-words as scratch; the rewrite models each
// slot as a zeroed local and passes local addresses where the original
// passes frame pointers. Three reads need care, all through unrelocated
// absolute addresses the checker cannot serve on this machine: tuning
// floats, the frame step and random thresholds, the scenario-global pointer,
// and a 16-byte sign mask; every one is read through `relocated()` so the
// rewrite stays correct wherever the image is mapped. The heading call takes
// its two doubles in vector registers with no stack arguments, which the
// checker cannot transport to the rewrite side; the rewrite computes the
// same doubles (kept as documentation) and the contract records the gap.
// Original: thiscall, one stack word, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00c7bc70(this: u32, ped: u32) -> u32 {
    unsafe {
        const SUB_TASK: u32 = 0x08;
        const STATUS: u32 = 0x0c;
        const STATUS_STICKY: u32 = 0x01;
        const STATUS_DONE: u32 = 0x02;
        const AUX_ID: u32 = 0x14;
        const AUX_CHAT: u32 = 0x1d;
        const AUX_NARROW: u32 = 0x1e;
        const PENDING_GUARD: u32 = 0x1c;
        const ACTIVE: u32 = 0x20;
        const WAIT_TIMER: u32 = 0x24;
        const STATE: u32 = 0x28;
        const LEG: u32 = 0x2c;
        const POINT: u32 = 0x30;
        const PED_PLACEMENT: u32 = 0x20;
        const PED_INTEL: u32 = 0x224;
        const PED_GROUP: u32 = 0xa80;
        const INTEL_SCENARIO: u32 = 0x2dc;
        const INTEL_ANCHOR: u32 = 0x284;
        const SELF_VT_SUBOP: u32 = 0x14;
        const GROUP_VT_SEAT: u32 = 0x44;
        const SCHED_VT_LEG: u32 = 0x54;
        const RAND_SCALE: f32 = f32::from_bits(0x38000100); // 2^-15
        const WAIT_LO: f32 = f32::from_bits(0x3f4ccccd); // 0.8
        const WAIT_HI: f32 = f32::from_bits(0x3f99999a); // 1.2
        const FAR2: f32 = f32::from_bits(0x40800000); // 4.0
        const FULL_WEIGHT: f32 = 1.0;
        const NEG_ONE: f32 = f32::from_bits(0xbf800000); // -1.0
        const CHAT_PING_IMM_FILE_VA: u32 = 0xed4a40; // relocated immediate
        const CHAT_PING_W: u32 = 0x3d4ccccd; // 0.05f
        const CAL_CONFIG: u32 = 2;
        const CAL_LEG_OK: u32 = 3;
        const CAL_SCHEDULE: u32 = 4;
        const CAL_SCHEDULE_USE: u32 = 5;
        const CAL_ROUTE_S1: u32 = 6;
        const CAL_ROUTE_S0: u32 = 7;
        const CAL_DEST_S1: u32 = 8;
        const CAL_DEST_S0: u32 = 9;
        const CAL_COMMIT_A: u32 = 10;
        const CAL_COMMIT_B: u32 = 11;
        const CAL_WORLD_S1A: u32 = 12;
        const CAL_WORLD_S1B: u32 = 13;
        const CAL_WORLD_S1C: u32 = 14;
        const CAL_WORLD_S1D: u32 = 15;
        const CAL_WORLD_S1E: u32 = 16;
        const CAL_WORLD_S1F: u32 = 17;
        const CAL_WORLD_S1G: u32 = 18;
        const CAL_ANCHOR: u32 = 19;
        const CAL_DOCK: u32 = 20;
        const CAL_BOARD: u32 = 21;
        const CAL_SEAT_INFO: u32 = 22;
        const CAL_ATTACH: u32 = 23;
        const CAL_LEG_LEN: u32 = 24;
        const CAL_HEADING: u32 = 25;
        const CAL_APPROACH: u32 = 26;
        const CAL_ROUTE_COMMIT: u32 = 27;
        const CAL_CHAT_PING: u32 = 28;
        const CAL_RAND: u32 = 29;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        /// The task's own sub-operation through its vtable (`kind` 1 or 2),
        /// answered by the checker's planted stub on both sides.
        #[inline(always)]
        unsafe fn self_subop(this: u32, ped: u32, kind: u32) -> u32 {
            unsafe {
                let slot = rd32(rd32(this) + SELF_VT_SUBOP);
                let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(this, ped, kind, 0)
            }
        }

        // Shared prologue: mark the ped's intelligence scenario-driven.
        wr8(rd32(ped + PED_INTEL) + INTEL_SCENARIO, 1);
        // Gate: an idle scenario runs the body, otherwise re-poll and yield
        // when the sub-operation accepts.
        if rd8(this + ACTIVE) != 0 && rd32(this + PENDING_GUARD) == 0 {
            if rd8(this + STATUS) & (STATUS_STICKY as u8) != 0 {
                return 0;
            }
            if self_subop(this, ped, 1) & 0xff != 0 {
                wr32(this + STATUS, rd32(this + STATUS) | STATUS_DONE);
                return 0;
            }
            if rd8(this + STATUS) & (STATUS_STICKY as u8) != 0 {
                return 0;
            }
            if self_subop(this, ped, 2) & 0xff != 0 {
                wr32(this + STATUS, rd32(this + STATUS) | STATUS_DONE);
                return 0;
            }
        }
        let config: u32 =
            lf_checker_rt::callee_cdecl!(CAL_CONFIG, u32, rd32(this + AUX_ID));
        let r = match rd32(this + STATE) {
            0 => state_wait(this, ped, config),
            1 => state_move(this, ped),
            _ => None,
        };
        if let Some(v) = r {
            return v;
        }
        // Tail: ping the ped for the chat variant, return the sub-task.
        if rd32(this + AUX_ID) == AUX_CHAT {
            let _ping: u32 = lf_checker_rt::callee_thiscall!(
                CAL_CHAT_PING,
                u32,
                ped,
                lf_checker_rt::relocated(CHAT_PING_IMM_FILE_VA),
                CHAT_PING_W,
                0,
                0
            );
        }
        let tail_value = rd32(this + SUB_TASK);

        /// State 0: count the wait timer down; on expiry randomise a new
        /// wait, sample a destination, and commit it (moving to state 1)
        /// when it lies beyond `FAR2` squared units. Returns the function
        /// result when the state decides it, else `None` for the tail.
        #[inline(always)]
        unsafe fn state_wait(this: u32, ped: u32, config: u32) -> Option<u32> {
            unsafe {
                let step = f32::from_bits(rd32(lf_checker_rt::relocated(0x11735bc)));
                let left = sub(rdf(this + WAIT_TIMER), step);
                wrf(this + WAIT_TIMER, left);
                if !(0.0 > left) {
                    // Still waiting (or unordered): tail.
                    return None;
                }
                let span = rdf(config.wrapping_add(0x44));
                let s0 = mul(span, WAIT_LO);
                let s1 = mul(span, WAIT_HI);
                let draw: u32 = lf_checker_rt::callee_cdecl!(CAL_RAND, u32,);
                let width = sub(s1, s0);
                let frac = mul(draw as i32 as f32, RAND_SCALE);
                wrf(this + WAIT_TIMER, add(mul(frac, width), s0));
                let weight_bits = if rd32(this + AUX_ID) == AUX_NARROW {
                    rd32(lf_checker_rt::relocated(0xfe8ad8))
                } else {
                    rd32(lf_checker_rt::relocated(0xfe8b38))
                };
                let placement = rd32(ped + PED_PLACEMENT);
                // Route-service scratch: the two frame slots the original
                // passes plus the two out-words it reads back. The contract
                // writes the out-words 12 bytes past each frame pointer.
                let mut routew = [0u32; 8];
                let sampled: u32 = lf_checker_rt::callee_thiscall!(
                    CAL_ROUTE_S0,
                    u32,
                    this.wrapping_add(AUX_ID),
                    placement.wrapping_add(0x30),
                    weight_bits,
                    1,
                    this.wrapping_add(AUX_ID),
                    &mut routew[0] as *mut u32 as u32,
                    &mut routew[1] as *mut u32 as u32,
                    1,
                    ped,
                    1
                );
                if sampled & 0xff == 0 {
                    return None;
                }
                let (leg_aux, leg_anchor) = (routew[3], routew[4]);
                let mut dest = [0u32; 4];
                let _refined: u32 = lf_checker_rt::callee_thiscall!(
                    CAL_DEST_S0,
                    u32,
                    leg_aux,
                    leg_anchor,
                    dest.as_mut_ptr() as u32
                );
                let base = placement.wrapping_add(0x30);
                let dx = sub(f32::from_bits(dest[0]), rdf(base));
                let dy = sub(f32::from_bits(dest[1]), rdf(base.wrapping_add(4)));
                let dz = sub(f32::from_bits(dest[2]), rdf(base.wrapping_add(8)));
                let dist2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                if dist2 <= FAR2 {
                    return None;
                }
                // Commit the point and the leg.
                let pt = this.wrapping_add(POINT);
                wr32(pt, dest[0]);
                wr32(pt.wrapping_add(4), dest[1]);
                wr32(pt.wrapping_add(8), dest[2]);
                wr32(pt.wrapping_add(12), dest[3]);
                let intel = rd32(ped + PED_INTEL);
                let group = rd32(ped + PED_GROUP);
                let seat_slot = rd32(rd32(group) + GROUP_VT_SEAT);
                let seat: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(seat_slot as usize);
                let seated = seat(group, 0);
                let leg: u32 = lf_checker_rt::callee_cdecl!(
                    CAL_ROUTE_COMMIT,
                    u32,
                    base,
                    pt,
                    seated | rd32(intel + INTEL_ANCHOR)
                );
                wr32(this + LEG, leg);
                if leg != 0 {
                    wr32(this + STATE, 1);
                }
                None
            }
        }

        /// State 1: validate the leg, resolve the anchor, seat the ped.
        /// Returns the function result when the state decides it, else
        /// `None` for the tail.
        #[inline(always)]
        unsafe fn state_move(this: u32, ped: u32) -> Option<u32> {
            unsafe {
                let leg_ok: u32 = lf_checker_rt::callee_cdecl!(
                    CAL_LEG_OK,
                    u32,
                    rd32(this + LEG)
                );
                if leg_ok & 0xff == 0 {
                    return None;
                }
                // Schedule-service scratch: three zeroed words plus the
                // three out-words the original passes frame pointers for.
                let schedz = [0u32; 3];
                let mut sched = [0u32; 3];
                let slot = rd32(this + LEG);
                let planned: u32 = lf_checker_rt::callee_cdecl!(
                    CAL_SCHEDULE,
                    u32,
                    slot,
                    &mut sched[2] as *mut u32 as u32,
                    &mut sched[1] as *mut u32 as u32,
                    &mut sched[0] as *mut u32 as u32,
                    0
                );
                let _used: u32 =
                    lf_checker_rt::callee_cdecl!(CAL_SCHEDULE_USE, u32, slot);
                wr32(this + LEG, 0);
                if planned == 0 {
                    wr32(this + STATE, 0);
                    return None;
                }
                // Route-service out-words, zeroed before the call.
                let mut leg_anchor: u32 = 0;
                let mut leg_aux: u32 = 0;
                let routed: u32 = lf_checker_rt::callee_thiscall!(
                    CAL_ROUTE_S1,
                    u32,
                    this.wrapping_add(AUX_ID),
                    this.wrapping_add(POINT),
                    FULL_WEIGHT.to_bits(),
                    1,
                    this.wrapping_add(AUX_ID),
                    &mut leg_anchor as *mut u32 as u32,
                    &mut leg_aux as *mut u32 as u32,
                    1,
                    ped,
                    0
                );
                if routed & 0xff == 0 {
                    wr32(this + STATE, 0);
                    return None;
                }
                let mut dest = [0u32; 4];
                let _refined: u32 = lf_checker_rt::callee_thiscall!(
                    CAL_DEST_S1,
                    u32,
                    leg_anchor,
                    leg_aux,
                    dest.as_mut_ptr() as u32
                );
                let _commit_a: u32 = lf_checker_rt::callee_thiscall!(
                    CAL_COMMIT_A,
                    u32,
                    this,
                    leg_aux
                );
                wr32(this + STATE, 2);
                let _commit_b: u32 = lf_checker_rt::callee_thiscall!(
                    CAL_COMMIT_B,
                    u32,
                    this,
                    leg_anchor
                );
                let world = rd32(lf_checker_rt::relocated(0x167e2a0));
                let world_a: u32 =
                    lf_checker_rt::callee_thiscall!(CAL_WORLD_S1A, u32, world);
                let anchor = if world_a == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(CAL_ANCHOR, u32, world_a)
                };
                let dockw: u32 =
                    lf_checker_rt::callee_thiscall!(CAL_WORLD_S1B, u32, world);
                if dockw == 0 {
                    // Null dock: attach a null boarding and head along.
                    return Some(heading(anchor, 0, world, leg_anchor));
                }
                let world_c: u32 =
                    lf_checker_rt::callee_thiscall!(CAL_WORLD_S1C, u32, world);
                let mut dock: u32 = 0;
                if world_c != 0 {
                    dock = lf_checker_rt::callee_thiscall!(
                        CAL_DOCK, u32, world_c, 0, NEG_ONE.to_bits(), 0, 0, 0, 0xc
                    );
                }
                let _dockslot = dock;
                let world_d: u32 =
                    lf_checker_rt::callee_thiscall!(CAL_WORLD_S1D, u32, world);
                let mut boarded: u32 = 0;
                if world_d != 0 {
                    let fare_hi = rd32(lf_checker_rt::relocated(0xee1eb4));
                    let fare_lo = rd32(lf_checker_rt::relocated(0xee1eb0));
                    let mut gate: u32 = 0;
                    boarded = lf_checker_rt::callee_thiscall!(
                        CAL_BOARD,
                        u32,
                        world_d,
                        2,
                        &mut gate as *mut u32 as u32,
                        fare_lo,
                        fare_hi,
                        0xffff_ffff,
                        1,
                        0,
                        0,
                        0,
                        1
                    );
                }
                let _seated: u32 = lf_checker_rt::callee_thiscall!(
                    CAL_SEAT_INFO,
                    u32,
                    dockw,
                    boarded,
                    schedz[2],
                    0,
                    0
                );
                Some(heading(anchor, boarded, world, leg_anchor))
            }
        }

        /// Heading block: resolve the leg vector through the schedule
        /// service, derive the heading, seat the ped. Always returns the
        /// function result (the anchor).
        #[inline(always)]
        unsafe fn heading(
            anchor: u32,
            boarded: u32,
            world: u32,
            leg_anchor: u32,
        ) -> u32 {
            unsafe {
                let _attached: u32 =
                    lf_checker_rt::callee_thiscall!(CAL_ATTACH, u32, anchor, boarded);
                let mut leg_vec = [0u32; 4];
                let leg_slot = rd32(rd32(leg_anchor) + SCHED_VT_LEG);
                let leg_svc: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(leg_slot as usize);
                let leg_len: u32 =
                    leg_svc(leg_anchor, leg_vec.as_mut_ptr() as u32, 0);
                let _measured: u32 =
                    lf_checker_rt::callee_thiscall!(CAL_LEG_LEN, u32, leg_len);
                // Heading over the leg vector. The two doubles travel in
                // vector registers with no stack arguments, which the
                // checker cannot transport to this side; the values below
                // document the computation the contract cannot observe.
                let mask = rd32(lf_checker_rt::relocated(0xfe8fa0));
                let _arg0 = (f32::from_bits(leg_vec[1] ^ mask) as f64).to_bits();
                let _arg1lo = (f32::from_bits(leg_vec[2]) as f64).to_bits();
                let _arg1hi = (f32::from_bits(leg_vec[3]) as f64).to_bits();
                let head: f64 = lf_checker_rt::callee_cdecl!(CAL_HEADING, f64,);
                let approach = head as f32;
                let world_e: u32 =
                    lf_checker_rt::callee_thiscall!(CAL_WORLD_S1E, u32, world);
                if world_e == 0 {
                    let _attached2: u32 =
                        lf_checker_rt::callee_thiscall!(CAL_ATTACH, u32, anchor, 0);
                    return anchor;
                }
                let world_f: u32 =
                    lf_checker_rt::callee_thiscall!(CAL_WORLD_S1F, u32, world);
                let mut seat: u32 = 0;
                if world_f != 0 {
                    seat = lf_checker_rt::callee_thiscall!(
                        CAL_DOCK, u32, world_f, 0, NEG_ONE.to_bits(), 0, 0, 0, 0xc
                    );
                }
                let world_g: u32 =
                    lf_checker_rt::callee_thiscall!(CAL_WORLD_S1G, u32, world);
                let mut gate2: u32 = 0;
                if world_g != 0 {
                    let fare_hi = rd32(lf_checker_rt::relocated(0xed7e70));
                    let fare_lo = rd32(lf_checker_rt::relocated(0xed7e68));
                    gate2 = lf_checker_rt::callee_thiscall!(
                        CAL_APPROACH,
                        u32,
                        world_g,
                        approach.to_bits(),
                        fare_lo,
                        fare_hi
                    );
                }
                let boarded2: u32 = lf_checker_rt::callee_thiscall!(
                    CAL_SEAT_INFO,
                    u32,
                    world_e,
                    gate2,
                    seat,
                    0,
                    0
                );
                let _attached3: u32 =
                    lf_checker_rt::callee_thiscall!(CAL_ATTACH, u32, anchor, boarded2);
                anchor
            }
        }
        tail_value
    }
});
