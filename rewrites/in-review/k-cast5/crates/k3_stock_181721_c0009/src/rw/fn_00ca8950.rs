// original: 0x00ca8950 CEventHandler::vf27
/// Answer a close-quarters threat event: when the handler's ped and the
/// event's target ped both validate their weapon state, steer around the
/// threat by building an evasive-step task.
///
/// `handler` points to the event handler (`+0x04` holds its ped, `+0x0c`
/// receives the new task). `event` points to the event record (`+0x20` is
/// the target ped, `+0x10`/`+0x14` the threat point). The second and third
/// stack arguments are not read.
///
/// Guards, in order: a null target returns with nothing stored (the value
/// is whatever the caller left in the return register, so live trials
/// always pass a target); an own-ped flag (bit 0x100000 at `+0x264`)
/// returns the own ped; a target weapon check (manager lookup, weapon info
/// bit 5 of the word at `+0x20`) and an own weapon check (a second lookup
/// whose low result byte must be clear) either fall through to the main
/// body or return the check's value with nothing stored.
///
/// The main body works in the horizontal plane: it normalises the offset
/// from the target position to the threat point (a zero offset stays zero
/// instead of dividing), builds a perpendicular push vector from the two
/// ped positions, flips it when a sidedness dot product is not positive,
/// adds the target-minus-own position delta, normalises again, and hands
/// the unit vector to the evasive-step constructor. A random draw scaled
/// by 100/32768 is then compared against bounds chosen from bits of a
/// target word (below 3 always rejects, 3 caps at 0x46, above at 0x50);
/// only a draw under the cap allocates a pool slot and constructs the
/// task, whose result is stored at `handler+0x0c` and returned. A null
/// slot stores zero and returns zero.
///
/// Original: 0x00ca8950 (thiscall, three stack words; the second and third
/// are not read).
lf_checker_rt::export!(thiscall, rw_00ca8950(handler: u32, event: u32, _a2: u32, _a3: u32) -> u32 {
    unsafe {
        const HANDLER_PED: u32 = 0x04;
        const HANDLER_TASK: u32 = 0x0c;
        const EVT_POINT_X: u32 = 0x10;
        const EVT_POINT_Y: u32 = 0x14;
        const EVT_TARGET: u32 = 0x20;
        const PED_POS: u32 = 0x20;
        const POS_X: u32 = 0x30;
        const POS_Y: u32 = 0x34;
        const POS_Z: u32 = 0x38;
        const OPED_FLAGS: u32 = 0x264;
        const OPED_ARMED_BIT: u32 = 0x0010_0000;
        const PED_WMGR: u32 = 0x2b0;
        const MGR_SLOT: u32 = 0x18;
        const INFO_FLAGS: u32 = 0x20;
        const INFO_ARMED_BIT: u32 = 5;
        const TPED_MODE: u32 = 0x224;
        const MODE_BITS: u32 = 0x2e8;
        const POOL_GLOBAL: u32 = 0x0167e2a0;
        const RAND_LO: i32 = 0x32;
        const RAND_HI_HIGH: i32 = 0x50;
        const RAND_HI_MID: i32 = 0x46;
        const RAND_SCALE_HI: f32 = f32::from_bits(0x3800_0000); // 2^-15
        const RAND_SCALE_LO: f32 = f32::from_bits(0x42c8_0000); // 100.0
        const ONE: f32 = 1.0;
        const NEG_ONE: f32 = -1.0;
        const ZERO: f32 = 0.0;
        const SIGN_BIT: u32 = 0x8000_0000;
        const WEAPON_MGR: u32 = 1;
        const WEAPON_INFO: u32 = 2;
        const OWN_CHECK: u32 = 3;
        const RAND: u32 = 4;
        const ALLOC: u32 = 5;
        const EVASIVE: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        /// Inverse length that maps a zero squared length to zero instead
        /// of dividing (the original's `ucomiss`/`lahf` NaN-aware test).
        #[inline(always)]
        fn inv_len(len2: f32) -> f32 {
            if core::hint::black_box(len2) == core::hint::black_box(ZERO) {
                ZERO
            } else {
                div(ONE, core::hint::black_box(len2).sqrt())
            }
        }

        let target = rd32(event + EVT_TARGET);
        if target == 0 {
            // Unreachable in live trials: the value returned here is the
            // caller's leftover in the return register, which a rewrite
            // cannot observe. Contract pins the target non-null.
            return 0;
        }
        let own = rd32(handler + HANDLER_PED);
        if rd32(own + OPED_FLAGS) & OPED_ARMED_BIT != 0 {
            return own;
        }
        let mgr: u32 = lf_checker_rt::callee_thiscall!(WEAPON_MGR, u32, target + PED_WMGR);
        if mgr != 0 {
            let mgr2: u32 = lf_checker_rt::callee_thiscall!(WEAPON_MGR, u32, target + PED_WMGR);
            let info: u32 = lf_checker_rt::callee_cdecl!(WEAPON_INFO, u32, rd32(mgr2 + MGR_SLOT));
            if (rd32(info + INFO_FLAGS) >> INFO_ARMED_BIT) & 1 == 0 {
                let own_mgr: u32 =
                    lf_checker_rt::callee_thiscall!(WEAPON_MGR, u32, own + PED_WMGR);
                if own_mgr != 0 {
                    let own_mgr2: u32 =
                        lf_checker_rt::callee_thiscall!(WEAPON_MGR, u32, own + PED_WMGR);
                    let check: u32 = lf_checker_rt::callee_thiscall!(OWN_CHECK, u32, own_mgr2);
                    if (check as u8) != 0 {
                        return check;
                    }
                }
            }
        }

        let posa = rd32(target + PED_POS);
        let posb = rd32(own + PED_POS);
        let ax = rdf(posa + POS_X);
        let ay = rdf(posa + POS_Y);
        let az = rdf(posa + POS_Z);
        let dx = sub(rdf(event + EVT_POINT_X), ax);
        let dy = sub(rdf(event + EVT_POINT_Y), ay);
        let delta_x = sub(ax, rdf(posb + POS_X));
        let delta_y = sub(ay, rdf(posb + POS_Y));
        let delta_z = sub(az, rdf(posb + POS_Z));
        let inv = inv_len(add(mul(dy, dy), mul(dx, dx)));
        let nx = mul(dx, inv);
        let ny = mul(dy, inv);
        // The original multiplies several terms by zero and subtracts the
        // results; the zeros matter when the terms are not finite, so every
        // step below follows its operand order exactly.
        let zx = mul(nx, ZERO);
        let z2 = mul(mul(inv, ZERO), ZERO);
        let mut px = sub(z2, ny);
        let mut py = sub(nx, z2);
        let pz0 = mul(ny, ZERO);
        let side_a = mul(px, ax);
        let mut pz = sub(pz0, zx);
        let acc1 = add(mul(ay, py), side_a);
        let acc1 = add(acc1, mul(az, pz));
        let acc1_neg = f32::from_bits(acc1.to_bits() ^ SIGN_BIT);
        let acc2 = add(mul(rdf(posb + POS_Y), py), mul(rdf(posb + POS_X), px));
        let acc2 = add(acc2, mul(rdf(posb + POS_Z), pz));
        let facing = add(acc1_neg, acc2);
        if !(core::hint::black_box(facing) > core::hint::black_box(ZERO)) {
            px = mul(px, NEG_ONE);
            py = mul(py, NEG_ONE);
            pz = mul(pz, NEG_ONE);
        }
        let qx = add(px, delta_x);
        let qy = add(py, delta_y);
        let qz = add(pz, delta_z);
        let s = inv_len(add(add(mul(qy, qy), mul(qx, qx)), mul(qz, qz)));
        let mut step = [mul(qx, s), mul(qy, s), mul(qz, s)];

        let mode = rd32(rd32(target + TPED_MODE) + MODE_BITS) & 7;
        let (lo, hi) = if mode >= 4 {
            (RAND_LO, RAND_HI_HIGH)
        } else if mode >= 3 {
            (RAND_LO, RAND_HI_MID)
        } else {
            (0, 0)
        };
        let draw: u32 = lf_checker_rt::callee_cdecl!(RAND, u32,);
        let scaled = mul(mul((draw & 0xffff) as f32, RAND_SCALE_HI), RAND_SCALE_LO);
        // In range and finite for every masked draw, so this truncates
        // exactly like the original's `cvttss2si`.
        let roll = core::hint::black_box(scaled) as i32;
        if roll >= lo && roll >= hi {
            return roll as u32;
        }
        let pool = rd32(lf_checker_rt::relocated(POOL_GLOBAL));
        let slot: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, pool);
        if slot == 0 {
            ((handler + HANDLER_TASK) as *mut u32).write_unaligned(0);
            return 0;
        }
        let task: u32 = lf_checker_rt::callee_thiscall!(
            EVASIVE,
            u32,
            slot,
            target,
            step.as_mut_ptr() as u32,
            1,
            0
        );
        ((handler + HANDLER_TASK) as *mut u32).write_unaligned(task);
        task
    }
});
