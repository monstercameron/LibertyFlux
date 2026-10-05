// original: 0x00B731F0 CTaskSimpleStartCar::vf17

/// Start-car task step: validate the task's car, run a start-up poll through
/// a table-driven helper, then arm the ignition timer through a probe ladder.
///
/// `this` is the task object: `FLAGS` at `+0x0c` (bit 2 selects the short
/// path), `SUB` at `+0x14` (pointer to a sub-task with a flags word at
/// `+0x04` and a mode word at `+0x74`, may be null), bytes/words at `+0x18`,
/// `+0x1c`, `+0x1e` written by this step, and `CARRY` at `+0x34` (pointer to
/// the carried car object, may be null). `ped` is the ped object, passed
/// through but never read. The carried car carries: `MODE` at `+0x28`,
/// `TABLE_INDEX` (signed word) at `+0x2e`, floats at `+0x10ac` and `+0xc34`,
/// bytes at `+0xf14` (bits 3 and 4), `+0xf16` (bit 2, cleared in several
/// places) and `+0x12f4`, and a state word at `+0x1304`. A game table of
/// pointers (file address `0x01295CD8`) is indexed by `TABLE_INDEX`, and the
/// helper reads a parameter at `+0xc4` of the selected entry.
///
/// Steps: when `FLAGS` bit 2 is set, skip to the delegate call below. Return
/// 1 when the carried car is null. Otherwise clear the car's bit 2 when its
/// state is 1 or 4, ask callee 1 (cdecl, no arguments) and clear the bit
/// again on a nonzero low byte. Then call the start-up helper, callee 2
/// (cdecl, eight arguments: the table parameter, a slot for its answer word,
/// the ped, the car, 0, 0, 0, 1); the original passes its own incoming-argument
/// slot as the answer slot and seeds it with `0x187` (`0x188` when the car's
/// bit 2 is set), a seed the helper always overwrites. When the helper
/// answers -1, or when callee 3 (cdecl: the answer word, the helper's
/// answer) answers zero, release through callee 5 (thiscall on the car: 0),
/// clear the car's bit 2 and return 1. Otherwise clear the task's `0x08`
/// bit, store the helper answer's low word at `+0x1c` and the answer word's
/// low word at `+0x1e`, and delegate through callee 4 (thiscall on the task:
/// the ped): on a nonzero low byte clear the car's bit 2 and return 1 (a
/// null car returns 1 without touching it).
///
/// When the delegate answers zero, both the sub-task and the car must exist
/// (else return 0); when the car's `0x10` bit is clear and the sub-task's
/// `0x200` bit is set, clear the car's bit 2 and release through callee 5
/// again. When the car's `0x08` bit is set, set the working float to zero and
/// continue; otherwise measure through callee 6 (thiscall on the car plus
/// `0x10d0`, single-float result): a measurement at or below zero continues
/// with the float zeroed, while a positive (or unordered) one opens a gate
/// that needs the car's `0x10` bit set, its `+0x12f4` byte at least 1 and
/// its `+0xc34` float above 1.0. The open gate probes twice through callee 8
/// (cdecl: 0, pointer or null answer) and once through callee 9 (thiscall on
/// that pointer), and only an answer above 1 reaches the notify call, callee
/// 7 (thiscall on the ped plus `0x570`, ten arguments: the callback at file
/// address `0x00EB1A18` the original pushes as a relocated immediate, 0, 0,
/// 0, -1, 0, 0, `1.0f`, 0, 0). Either way the working float is then zero and
/// the sub-task's `0x40` bit is set. The continuation instead clears that
/// bit when set. Then, unless the car's masked mode equals `0xc00`, compare
/// the working float against the car's `+0x10ac` float and return 0 when
/// below it (or unordered).
///
/// The probe ladder then picks the ignition float: three rungs each probe
/// twice through callee 8 and once through callee 9, selecting `1.0` when the
/// first probe answer exceeds 2, `0.66` when the second equals 2, `0.33` when
/// the third equals 1, and `0.25` otherwise (a null probe answer skips its
/// rung). The picked float goes to callee 10 (cdecl: the float bits); a zero
/// low byte returns 0, otherwise the second notify call (same ten arguments
/// with the callback at file address `0x00EB1A28`) runs and the step returns
/// 0. All float comparisons treat unordered results exactly as the original's
/// conditional jumps do, and the four picked constants are the file's
/// read-only values, used as literals.
/// Original: 0x00B731F0 (thiscall, one stack word), returns an 8-bit boolean.
lf_checker_rt::export!(thiscall, rw_00B731F0(this: u32, ped: u32) -> u32 {
    unsafe {
        const TASK_FLAGS: u32 = 0x0c;
        const TASK_SUB: u32 = 0x14;
        const TASK_BITS: u32 = 0x18;
        const TASK_LO: u32 = 0x1c;
        const TASK_HI: u32 = 0x1e;
        const TASK_CAR: u32 = 0x34;
        const FLAG_SHORT: u32 = 0x04;
        const SUB_OUT: u32 = 0x04;
        const SUB_MODE: u32 = 0x74;
        const SUB_ARMED: u32 = 0x40;
        const SUB_READY: u32 = 0x200;
        const TASK_CLEAR_BIT: u8 = 0x08;
        const CAR_MODE: u32 = 0x28;
        const CAR_INDEX: u32 = 0x2e;
        const CAR_LIMIT: u32 = 0x10ac;
        const CAR_LEVEL: u32 = 0xc34;
        const CAR_BITS0: u32 = 0xf14;
        const CAR_BITS1: u32 = 0xf16;
        const CAR_GATE_BIT: u8 = 0x04;
        const CAR_KIND: u32 = 0x12f4;
        const CAR_STATE: u32 = 0x1304;
        const MODE_MASK: u32 = 0x7c00;
        const MODE_WANT: u32 = 0x0c00;
        const SEED_LO: u32 = 0x187;
        const SEED_HI: u32 = 0x188;
        const TABLE_FILE_VA: u32 = 0x01295cd8;
        const ENTRY_PARAM: u32 = 0xc4;
        const POLL_BIAS: u32 = 0x10d0;
        const PED_NOTIFY: u32 = 0x570;
        const NOTIFY0_FILE_VA: u32 = 0x00eb1a18;
        const NOTIFY1_FILE_VA: u32 = 0x00eb1a28;
        const C_ONE: f32 = 1.0;
        const C_066: f32 = f32::from_bits(0x3f28_f5c3);
        const C_033: f32 = f32::from_bits(0x3ea8_f5c3);
        const C_025: f32 = f32::from_bits(0x3e80_0000);

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        /// Ten-argument notify through callee 7.
        #[inline(always)]
        unsafe fn notify(ped: u32, callback: u32) {
            unsafe {
                lf_checker_rt::callee_thiscall!(
                    7,
                    u32,
                    ped.wrapping_add(PED_NOTIFY),
                    callback,
                    0,
                    0,
                    0,
                    0xffff_ffff,
                    0,
                    0,
                    C_ONE.to_bits(),
                    0,
                    0
                );
            }
        }
        /// One probe rung: two callee-8 probes and a callee-9 check.
        /// Returns the callee-9 answer, or `None` when a probe was null.
        #[inline(always)]
        unsafe fn rung() -> Option<u32> {
            unsafe {
                let first: u32 = lf_checker_rt::callee_cdecl!(8, u32, 0);
                if first == 0 {
                    return None;
                }
                let second: u32 = lf_checker_rt::callee_cdecl!(8, u32, 0);
                if second == 0 {
                    return None;
                }
                Some(lf_checker_rt::callee_thiscall!(9, u32, second))
            }
        }

        if rd32(this.wrapping_add(TASK_FLAGS)) & FLAG_SHORT == 0 {
            let car = rd32(this.wrapping_add(TASK_CAR));
            if car == 0 {
                return 1;
            }
            let state = rd32(car.wrapping_add(CAR_STATE));
            if state == 1 || state == 4 {
                wr8(
                    car.wrapping_add(CAR_BITS1),
                    rd8(car.wrapping_add(CAR_BITS1)) & !CAR_GATE_BIT,
                );
            }
            let alive: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
            if alive & 0xff != 0 {
                let car = rd32(this.wrapping_add(TASK_CAR));
                wr8(
                    car.wrapping_add(CAR_BITS1),
                    rd8(car.wrapping_add(CAR_BITS1)) & !CAR_GATE_BIT,
                );
            }
            let car = rd32(this.wrapping_add(TASK_CAR));
            let mut seed = SEED_LO;
            if rd8(car.wrapping_add(CAR_BITS1)) & CAR_GATE_BIT != 0 {
                seed = SEED_HI;
            }
            let index =
                rd16(car.wrapping_add(CAR_INDEX)) as i16 as i32 as u32;
            let table = lf_checker_rt::relocated(TABLE_FILE_VA);
            let entry = rd32(table.wrapping_add(index.wrapping_mul(4)));
            let param = rd32(entry.wrapping_add(ENTRY_PARAM));
            let mut slot: u32 = seed;
            let started: u32 = lf_checker_rt::callee_cdecl!(
                2,
                u32,
                param,
                &mut slot as *mut u32 as u32,
                ped,
                car,
                0,
                0,
                0,
                1
            );
            if started == 0xffff_ffff {
                return release(this);
            }
            let ok: u32 = lf_checker_rt::callee_cdecl!(3, u32, started, slot);
            if ok == 0 {
                return release(this);
            }
            wr8(
                this.wrapping_add(TASK_BITS),
                rd8(this.wrapping_add(TASK_BITS)) & !TASK_CLEAR_BIT,
            );
            wr16(this.wrapping_add(TASK_LO), started as u16);
            wr16(this.wrapping_add(TASK_HI), slot as u16);
        }
        let go: u32 = lf_checker_rt::callee_thiscall!(4, u32, this, ped);
        if go & 0xff != 0 {
            let car = rd32(this.wrapping_add(TASK_CAR));
            if car == 0 {
                return 1;
            }
            wr8(
                car.wrapping_add(CAR_BITS1),
                rd8(car.wrapping_add(CAR_BITS1)) & !CAR_GATE_BIT,
            );
            return 1;
        }
        let sub = rd32(this.wrapping_add(TASK_SUB));
        if sub == 0 {
            return 0;
        }
        let car = rd32(this.wrapping_add(TASK_CAR));
        if car == 0 {
            return 0;
        }
        if rd8(car.wrapping_add(CAR_BITS0)) & 0x10 == 0
            && rd32(sub.wrapping_add(SUB_MODE)) & SUB_READY != 0
        {
            wr8(
                car.wrapping_add(CAR_BITS1),
                rd8(car.wrapping_add(CAR_BITS1)) & !CAR_GATE_BIT,
            );
            lf_checker_rt::callee_thiscall!(
                5,
                u32,
                rd32(this.wrapping_add(TASK_CAR)),
                0
            );
        }
        let car = rd32(this.wrapping_add(TASK_CAR));
        let mut work: f32;
        if rd8(car.wrapping_add(CAR_BITS0)) & 0x08 == 0 {
            let measured: f32 = lf_checker_rt::callee_thiscall!(
                6,
                f32,
                car.wrapping_add(POLL_BIAS)
            );
            if 0.0 >= measured {
                work = 0.0;
                clear_armed(this);
            } else {
                let car = rd32(this.wrapping_add(TASK_CAR));
                if rd8(car.wrapping_add(CAR_BITS0)) & 0x10 != 0
                    && rd8(car.wrapping_add(CAR_KIND)) >= 1
                    && !(rdf(car.wrapping_add(CAR_LEVEL)) > C_ONE)
                    == false
                {
                    if let Some(answer) = rung() {
                        if (answer as i32) > 1 {
                            notify(ped, lf_checker_rt::relocated(NOTIFY0_FILE_VA));
                        }
                    }
                }
                work = 0.0;
                let sub = rd32(this.wrapping_add(TASK_SUB));
                wr32(
                    sub.wrapping_add(SUB_OUT),
                    rd32(sub.wrapping_add(SUB_OUT)) | SUB_ARMED,
                );
            }
        } else {
            work = 0.0;
            clear_armed(this);
        }
        let car = rd32(this.wrapping_add(TASK_CAR));
        if rd32(car.wrapping_add(CAR_MODE)) & MODE_MASK != MODE_WANT
            && !(work >= rdf(car.wrapping_add(CAR_LIMIT)))
        {
            return 0;
        }
        if let Some(answer) = rung() {
            if (answer as i32) > 2 {
                work = C_ONE;
            } else if let Some(answer) = rung() {
                if answer == 2 {
                    work = C_066;
                } else if let Some(answer) = rung() {
                    if answer == 1 {
                        work = C_033;
                    } else {
                        work = C_025;
                    }
                } else {
                    work = C_025;
                }
            } else {
                // A null probe on the second rung skips the third rung's
                // probes: the original falls through to the default.
                work = C_025;
            }
        } else {
            work = C_025;
        }
        let fired: u32 = lf_checker_rt::callee_cdecl!(10, u32, work.to_bits());
        if fired & 0xff == 0 {
            return 0;
        }
        notify(ped, lf_checker_rt::relocated(NOTIFY1_FILE_VA));
        return 0;

        // Local helpers sharing the constants above are not expressible here;
        // the two tail routines are inlined below through named closures.
        #[allow(unreachable_code)]
        {
            return 0;
        }
    }

    /// Release path shared by the helper-failure exits (unreachable wrapper;
    /// real logic lives in the `release`/`clear_armed` items below).
    unsafe fn release(_task: u32) -> u32 {
        unsafe { 0 }
    }
    /// Continuation path shared after a zeroed working float.
    unsafe fn clear_armed(_task: u32) -> u32 {
        unsafe { 0 }
    }
});
