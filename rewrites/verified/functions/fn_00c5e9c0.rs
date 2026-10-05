// original: 0x00c5e9c0 gang_task_factory (proposed)

/// Build a gang-related task by id: allocate it from the task pool and run
/// its constructor, returning the task pointer or zero.
///
/// `this` is the requesting task (a word at `+0x18` passed to several
/// constructors, the address of `+0x30` passed to one). `id` selects the
/// kind: 0x387 builds a move-to-point task then attaches it through the
/// shared tail, 0xca builds a timed task, 0x38b builds a seek task then the
/// shared tail, 0x3fc resolves the ped's weapon id through callee 5 and
/// builds a projectile or gun task depending on bit 6 of the info word at
/// `+0x20`. Any other id returns zero. `ped` (used only for 0x3fc) carries
/// weapon slots at `+0x2b0` with the selected id at `(slot+3)*3`. A failed
/// pool allocation returns zero, except on the 0x387/0x38b head where it
/// clears the subtask slot and still runs the shared tail.
///
/// Original: 0x00c5e9c0 (thiscall, two stack words; frameless).
lf_checker_rt::export!(thiscall, rw_00c5e9c0(this: u32, id: u32, ped: u32) -> u32 {
    unsafe {
        const CAL_ALLOC: u32 = 0;
        const CAL_MOVE: u32 = 1;
        const CAL_ATTACH: u32 = 2;
        const CAL_TIMED: u32 = 3;
        const CAL_SEEK: u32 = 4;
        const CAL_WINFO: u32 = 5;
        const CAL_PROJ: u32 = 6;
        const CAL_GUN: u32 = 7;
        const POOL: u32 = 0x0167e2a0;
        const K_2_0: u32 = 0x00ed7f2c;
        const K_0_5: u32 = 0x00ed7f28;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let pool = rd32(lf_checker_rt::relocated(POOL));
        let alloc = |pool: u32| -> u32 {
            lf_checker_rt::callee_thiscall!(CAL_ALLOC, u32, pool)
        };
        // Shared tail: allocate the attach task and run it over `sub`.
        let tail = |pool: u32, sub: u32| -> u32 {
            let t = alloc(pool);
            if t == 0 {
                return 0;
            }
            lf_checker_rt::callee_thiscall!(CAL_ATTACH, u32, t, sub, 0, 0, 0)
        };

        if (id as i32) > 0x38b {
            if id != 0x3fc {
                return 0;
            }
            let slot = rd32(ped.wrapping_add(0x2b0));
            let idx = slot.wrapping_add(3).wrapping_mul(3);
            let wid = rd32(ped.wrapping_add(idx.wrapping_mul(4)).wrapping_add(0x2b0));
            let info: u32 = lf_checker_rt::callee_cdecl!(CAL_WINFO, u32, wid);
            if (rd32(info.wrapping_add(0x20)).wrapping_shr(6) & 1) != 0 {
                let t = alloc(pool);
                if t == 0 {
                    return 0;
                }
                return lf_checker_rt::callee_thiscall!(
                    CAL_PROJ,
                    u32,
                    t,
                    rd32(this.wrapping_add(0x18))
                );
            }
            let t = alloc(pool);
            if t == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(
                CAL_GUN,
                u32,
                t,
                5,
                rd32(this.wrapping_add(0x18)),
                0,
                0x40a00000,
                0,
                1,
                1,
                0x3f800000
            );
        }
        if id == 0x38b {
            let t = alloc(pool);
            let sub = if t == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(
                    CAL_SEEK,
                    u32,
                    t,
                    rd32(this.wrapping_add(0x18)),
                    0xc350,
                    0x3e8,
                    0x3f800000,
                    0x40000000,
                    0x40000000,
                    1
                )
            };
            return tail(pool, sub);
        }
        if id == 0xca {
            let t = alloc(pool);
            if t == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(CAL_TIMED, u32, t, 0x64);
        }
        if id == 0x387 {
            let t = alloc(pool);
            let sub = if t == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(
                    CAL_MOVE,
                    u32,
                    t,
                    3,
                    this.wrapping_add(0x30),
                    rd32(lf_checker_rt::relocated(K_0_5)),
                    rd32(lf_checker_rt::relocated(K_2_0)),
                    0,
                    0
                )
            };
            return tail(pool, sub);
        }
        0
    }
});
