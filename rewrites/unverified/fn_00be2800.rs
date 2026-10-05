// original: 0x00be2800 CTaskComplexInWater::vf19

/// In-water task update: pick the swimming behaviour for the pedestrian.
///
/// `this` is the complex task, `ped` the pedestrian. The update clears the
/// wading flag (bit `0x40000` at `+0x26C`) and runs the shared movement
/// prologue, then dispatches on the pedestrian's state:
///
/// - Seated (`+0x38` non-null and equal to `+0x7B4`): request a pool task
///   and configure it for the seat (`0xFA`, `0x1F4`, `-1.0`, `-1.0`).
/// - Otherwise, unless the water flag (byte `+0x219`) is set: roll a random
///   drift (`rand & 0xFFFF`, scaled by two constants, truncated toward zero
///   exactly like `cvttss2si`, subtracted from `0x1388`), look up the water
///   surface task, and check the running subtask's type (`0x2DE`).
///   Depending on what the surface, gauge and depth checks report, the
///   update configures a dive, climb, float or swim task from the pool, or a
///   plain movement task.
/// - With the water flag set, or when no branch applies: return a plain
///   movement task from the pool, or 0 when the pool is empty.
///
/// The depth probe takes a pointer to three words on the caller's frame
/// (`4.0`, `4.0`, `0`); the contract compares those plus a fourth zero word
/// under a zero stack fill. All float arithmetic keeps the original's
/// operand order.
///
/// Original: 0x00be2800 (thiscall, one stack word, returns a task pointer
/// or 0).
lf_checker_rt::export!(thiscall, rw_00be2800(this: u32, ped: u32) -> u32 {
    unsafe {
        const PED_FLAGS: u32 = 0x26c;
        const WADING: u32 = 0x4_0000;
        const PED_SEAT: u32 = 0x38;
        const PED_SEAT_REF: u32 = 0x7b4;
        const PED_WATER: u32 = 0x219;
        const PED_DEEP: u32 = 0x268;
        const DEEP_BIT: u32 = 0x200;
        const TASK_SUB: u32 = 0x08;
        const TASK_SURFACE: u32 = 0x44;
        const TASK_DRIFT: u32 = 0x4c;
        const TASK_ALT: u32 = 0x48;
        const TASK_FLAG68: u32 = 0x68;
        const TASK_LO: u32 = 0x6c;
        const TASK_HI: u32 = 0x70;
        const TASK_VEC: u32 = 0x20;
        const TYPE_SWIM: u32 = 0x2de;
        const VT_TYPE: u32 = 0x0c;
        const TASK_POOL: u32 = 0x167e2a0;
        const DRIFT_BASE: u32 = 0x1388;
        const SCALE_A: u32 = 0xfe8680;
        const SCALE_B: u32 = 0xeb9514;
        const PROBE_F1: u32 = 0xee1eb4;
        const PROBE_F0: u32 = 0xee1eb0;
        const FIVE: u32 = 0x40a0_0000;
        const NEG_ONE: u32 = 0xbf80_0000;

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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        /// Truncate toward zero exactly like `cvttss2si`: NaN or a value at
        /// or above 2^31 yields `i32::MIN`; anything else is `as` (which
        /// truncates and can only saturate downward there, identically).
        #[inline(always)]
        fn cvttss2si(f: f32) -> i32 {
            if f.is_nan() || f >= 2147483648.0 {
                i32::MIN
            } else {
                f as i32
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
                lf_checker_rt::callee_thiscall!(2, u32, pool)
            }
        }
        /// Plain movement task from the pool, or 0 when empty.
        unsafe fn movement_task() -> u32 {
            unsafe {
                let task = pool_task();
                if task == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(11, u32, task)
                }
            }
        }

        wr32(ped + PED_FLAGS, rd32(ped + PED_FLAGS) & !WADING);
        lf_checker_rt::callee_thiscall!(1, u32, this, ped);
        let seat = rd32(ped + PED_SEAT);
        if seat != 0 && seat == rd32(ped + PED_SEAT_REF) {
            let task = pool_task();
            if task == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(3, u32, task, 0xfa, 0x1f4, NEG_ONE, NEG_ONE);
        }
        if rd8(ped + PED_WATER) != 0 {
            return movement_task();
        }
        let roll = lf_checker_rt::callee_cdecl!(4, u32,) & 0xffff;
        let mut drift = roll as f32;
        drift = mul(drift, f32::from_bits(rd32(lf_checker_rt::relocated(SCALE_A))));
        drift = mul(drift, f32::from_bits(rd32(lf_checker_rt::relocated(SCALE_B))));
        wr32(
            this + TASK_DRIFT,
            DRIFT_BASE.wrapping_sub(cvttss2si(drift) as u32),
        );
        let surf = lf_checker_rt::callee_thiscall!(5, u32, this, ped);
        let sub = rd32(this + TASK_SUB);
        let mut swimming = 0u8;
        if sub != 0 {
            swimming = 1;
            if task_type(sub) != TYPE_SWIM {
                swimming = 0;
            }
        }
        let gauge = lf_checker_rt::callee_thiscall!(6, u32, ped);
        wr32(this + TASK_SURFACE, gauge);
        if gauge != 0 && rd8(gauge + 0x56) & 1 != 0 {
            let x = lf_checker_rt::callee_thiscall!(7, u32, gauge.wrapping_add(8));
            if x != 0
                && lf_checker_rt::callee_thiscall!(8, u32, ped, FIVE) & 0xff != 0
            {
                let z = lf_checker_rt::callee_thiscall!(
                    7,
                    u32,
                    rd32(this + TASK_SURFACE).wrapping_add(8)
                );
                let sel: u32;
                let mut skip_kind = false;
                if rd8(z + 0x26c) & 4 != 0 {
                    let veh = rd32(z + 0xb30);
                    if veh != 0 {
                        sel = veh;
                        skip_kind = true;
                    } else {
                        sel = rd32(z + 0xab0);
                    }
                } else {
                    sel = rd32(z + 0xab0);
                }
                if sel != 0
                    && (skip_kind || rd32(sel + 0x28) & 0x3c0 == 0x80)
                    && rd32(sel + 0x1304) == 2
                    && swimming == 0
                {
                    let task = pool_task();
                    if task == 0 {
                        return 0;
                    }
                    return lf_checker_rt::callee_thiscall!(
                        9,
                        u32,
                        task,
                        sel,
                        0xffff_fffa,
                        0x0b,
                        0,
                        0
                    );
                }
                let task = pool_task();
                let s3 = if task == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(10, u32, task, rd32(this + TASK_SURFACE))
                };
                let task2 = pool_task();
                if task2 == 0 {
                    return 0;
                }
                let task3 = pool_task();
                let a3 = if task3 == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(11, u32, task3)
                };
                return lf_checker_rt::callee_thiscall!(12, u32, task2, s3, a3, 2, 0);
            }
        }
        if surf != 0 {
            let a = rd32(surf + 0x18);
            if a != 0 && rd32(a + 0x1304) == 2 && swimming == 0 {
                let task = pool_task();
                if task == 0 {
                    return 0;
                }
                return lf_checker_rt::callee_thiscall!(
                    9,
                    u32,
                    task,
                    rd32(surf + 0x18),
                    rd32(surf + 0x24),
                    0x0b,
                    0,
                    0
                );
            }
        }
        if rd32(this + TASK_ALT) != 0
            && lf_checker_rt::callee_thiscall!(8, u32, ped, FIVE) & 0xff != 0
        {
            let task = pool_task();
            let g2 = if task == 0 {
                0
            } else {
                let frame = [0x4080_0000u32, 0x4080_0000, 0, 0];
                lf_checker_rt::callee_thiscall!(
                    13,
                    u32,
                    task,
                    rd32(this + TASK_ALT),
                    frame.as_ptr() as u32,
                    1,
                    0xffff_ffff,
                    0x4120_0000,
                    0x41a0_0000,
                    1
                )
            };
            return finish_float(this, g2);
        }
        if rd8(this + TASK_FLAG68) != 0
            && lf_checker_rt::callee_thiscall!(8, u32, ped, FIVE) & 0xff != 0
        {
            let task = pool_task();
            let g2 = if task == 0 {
                0
            } else {
                let f0 = rd32(lf_checker_rt::relocated(PROBE_F0));
                let f1 = rd32(lf_checker_rt::relocated(PROBE_F1));
                lf_checker_rt::callee_thiscall!(
                    14,
                    u32,
                    task,
                    2,
                    this + TASK_VEC,
                    f0,
                    f1,
                    0xffff_ffff,
                    1,
                    0,
                    0,
                    0,
                    1
                )
            };
            return finish_float(this, g2);
        }
        if rd32(ped + PED_DEEP) & DEEP_BIT == 0
            && (rd32(this + TASK_LO) as i32) < (rd32(this + TASK_HI) as i32)
        {
            let task = pool_task();
            if task == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(15, u32, task, 0x4220_0000, 0x4000_0000);
        }
        movement_task()
    }
});

/// Shared tail of the float-task paths: combine the probe result `g2` with
/// a fresh movement task.
#[inline(never)]
unsafe fn finish_float(_this: u32, g2: u32) -> u32 {
    unsafe {
        const TASK_POOL: u32 = 0x167e2a0;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let pool = rd32(lf_checker_rt::relocated(TASK_POOL));
        let task = lf_checker_rt::callee_thiscall!(2, u32, pool);
        if task == 0 {
            return 0;
        }
        let task3 = lf_checker_rt::callee_thiscall!(2, u32, pool);
        let a3 = if task3 == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(11, u32, task3)
        };
        lf_checker_rt::callee_thiscall!(12, u32, task, g2, a3, 2, 0)
    }
}
