// original: 0x00BBE480 ped_assign_follow_navmesh_route_task (proposed)

/// Build a follow-navmesh-route task for a ped and assign it at slot 0x1B.
///
/// `ped` resolves through the ped pool like the sibling builders; a zero
/// handle skips the lookup and a busy flag returns early with no task.
///
/// `mode` selects a speed class: -1 means the default from the globals
/// (50000), -2 means -1, anything else is used as is. It is converted to
/// float (signed), scaled by 0.001, raised by 2.0 when above zero, and
/// staged for the attach call; the multiply/add run in the original's
/// operand order with the order pinned.
///
/// A route task (callee 3) is built on a fresh pool object as
/// `(a18, xyz_ptr, a20, 3.0, mode_val, 1, 0, 0, a20, 1)` over the staged
/// `(x, y, z)`, then flagged at `+0xD8`. When `b24` is set the task is
/// further flagged and gets a 4.0 float at `+0x60` and a constant at
/// `+0x64`. When `b28` is set, `a2c` scaled to radians feeds the heading
/// helper (callee 4, float in and out) whose answer lands at `+0x68`
/// alongside another flag, while `a30` truncated toward zero (exact
/// `cvttss2si` emulation: out-of-range and NaN give `0x80000000`) lands at
/// `+0xD4` as a half word. The task is wrapped by the attach call
/// (callee 6) as `(task, 0, 0, staged_float)` and assigned as
/// `(ped, task, slot)` with slot `TASK_SLOT` (callee 7); a failed wrap
/// allocation assigns null. A failed route allocation faults the original
/// (a flag-or through null) and is covered by fault parity.
///
/// Original: 0x00BBE480 (cdecl, eleven stack words, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00BBE480(ped: u32, x: u32, y: u32, z: u32, a18: u32, mode: u32, a20: u32, b24: u32, b28: u32, a2c: u32, a30: u32) -> u32 {
    unsafe {
        const PED_POOL: u32 = 0x018B6F1C;
        const TASK_POOL: u32 = 0x0167E2A0;
        const MODE_DEFAULT: u32 = 0x00EE1EB8;
        const THREE: u32 = 0x00EE1EB4;
        const FOUR: u32 = 0x00EE1ED8;
        const MS_TO_S: u32 = 0x00FE86B4;
        const ZERO_F: u32 = 0x00FE8628;
        const TWO: u32 = 0x00FE8A24;
        const DEG_TO_RAD: u32 = 0x00FE8728;
        const PED_STATUS: u32 = 0x6C;
        const PED_BUSY_FLAG: u32 = 0x0E;
        const TASK_FLAGS: u32 = 0xD8;
        const TASK_F60: u32 = 0x60;
        const TASK_C64: u32 = 0x64;
        const TASK_F68: u32 = 0x68;
        const TASK_HD4: u32 = 0xD4;
        const C64_VALUE: u32 = 0xC47A0000;
        const TASK_SLOT: u32 = 0x1B;

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
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
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
        fn cvtt_truncate(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                0x80000000u32 as i32
            } else {
                x as i32
            }
        }

        if ped != 0 {
            let pool = rd32(lf_checker_rt::relocated(PED_POOL));
            let obj: u32 = lf_checker_rt::callee_thiscall!(1, u32, pool, ped);
            let status = rd32(obj.wrapping_add(PED_STATUS));
            if status != 0 && rd8(status.wrapping_add(PED_BUSY_FLAG)) != 0 {
                return 0;
            }
        }
        let mode_val: u32;
        if mode == 0xFFFFFFFFu32 {
            mode_val = rd32(lf_checker_rt::relocated(MODE_DEFAULT));
        } else if mode == 0xFFFFFFFEu32 {
            mode_val = 0xFFFFFFFFu32;
        } else {
            mode_val = mode;
        }
        let mut f = mul((mode_val as i32) as f32, f32::from_bits(rd32(lf_checker_rt::relocated(MS_TO_S))));
        let zero = f32::from_bits(rd32(lf_checker_rt::relocated(ZERO_F)));
        if f > zero {
            f = add(f, f32::from_bits(rd32(lf_checker_rt::relocated(TWO))));
        }
        let task_pool = rd32(lf_checker_rt::relocated(TASK_POOL));
        let xyz = [x, y, z];
        let xyz_ptr = xyz.as_ptr() as u32;
        let route_alloc: u32 = lf_checker_rt::callee_thiscall!(2, u32, task_pool);
        let three = rd32(lf_checker_rt::relocated(THREE));
        let task: u32;
        if route_alloc == 0 {
            task = 0;
        } else {
            task = lf_checker_rt::callee_thiscall!(3, u32, route_alloc, a18, xyz_ptr, a20, three, mode_val, 1, 0, 0, a20, 1);
        }
        wr32(task.wrapping_add(TASK_FLAGS), rd32(task.wrapping_add(TASK_FLAGS)) | 0x10000000);
        if (b24 as u8) != 0 {
            wr32(task.wrapping_add(TASK_FLAGS), rd32(task.wrapping_add(TASK_FLAGS)) | 0x300000);
            wr32(task.wrapping_add(TASK_F60), rd32(lf_checker_rt::relocated(FOUR)));
            wr32(task.wrapping_add(TASK_C64), C64_VALUE);
        }
        if (b28 as u8) != 0 {
            let scaled = mul(f32::from_bits(a2c), f32::from_bits(rd32(lf_checker_rt::relocated(DEG_TO_RAD))));
            let heading: f32 = lf_checker_rt::callee_cdecl!(4, f32, scaled.to_bits());
            let slots = cvtt_truncate(f32::from_bits(a30));
            wr32(task.wrapping_add(TASK_FLAGS), rd32(task.wrapping_add(TASK_FLAGS)) | 0x40000000);
            wr32(task.wrapping_add(TASK_F68), heading.to_bits());
            wr16(task.wrapping_add(TASK_HD4), slots as u16);
        }
        let wrap: u32 = lf_checker_rt::callee_thiscall!(5, u32, task_pool);
        if wrap == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(7, u32, ped, 0, TASK_SLOT);
        } else {
            let attached: u32 =
                lf_checker_rt::callee_thiscall!(6, u32, wrap, task, 0, 0, f.to_bits());
            let _: u32 = lf_checker_rt::callee_cdecl!(7, u32, ped, attached, TASK_SLOT);
        }
        0
    }
});
