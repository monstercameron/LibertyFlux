// original: 0x00BBE850 ped_assign_goto_task_timed_or_still (proposed)

/// Build a go-to-point task for a ped and assign it at slot 0x11.
///
/// `ped` is a script handle resolved through the ped pool (global at
/// `PED_POOL`). A zero handle skips the lookup; a resolved ped whose status
/// object (`+0x6C`) exists and whose flag byte (`+0x0E`) is set makes the
/// function return without building anything.
///
/// Otherwise a task is built two ways from `mode` (`[ebp+0x1C]`). When
/// `mode` is -2, the point-and-stand-still constructor (callee 4) runs on a
/// fresh task-pool object with `(arg4, xyz_ptr, 0.5, 2.0, 0, 1)`, where
/// `xyz_ptr` points at the `(x, y, z)` arguments staged in the frame. For
/// any other `mode`, the timed variant (callee 5) runs with
/// `(arg4, xyz_ptr, 0.5, 2.0, 20000)` instead, and a duration is derived as
/// `float(mode_or_20000) * 0.001` (with `mode` -1 meaning 20000), plus 2.0
/// when that product is above zero; the -2 path uses 0.0. The float
/// conversion is signed and the multiply/add run in the original's operand
/// order with the order pinned.
///
/// The built task (or null when its allocation failed) is then wrapped by
/// the attach call (callee 6) as `(task, 0, 0, duration_bits)` on another
/// fresh pool object, and the result is assigned as `(ped, task, slot)`
/// with slot `TASK_SLOT` (callee 7). When the wrap allocation fails, a null
/// task is assigned and the attach call is skipped.
///
/// Original: 0x00BBE850 (cdecl, six stack words, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00BBE850(ped: u32, x: u32, y: u32, z: u32, arg4: u32, mode: u32) -> u32 {
    unsafe {
        const PED_POOL: u32 = 0x018B6F1C;
        const TASK_POOL: u32 = 0x0167E2A0;
        const HALF: u32 = 0x00ED7F28;
        const TWO_A: u32 = 0x00ED7F2C;
        const MODE_DEFAULT: u32 = 0x00ED7F68;
        const MS_TO_S: u32 = 0x00FE86B4;
        const TWO_B: u32 = 0x00FE8A24;
        const PED_STATUS: u32 = 0x6C;
        const PED_BUSY_FLAG: u32 = 0x0E;
        const TASK_SLOT: u32 = 0x11;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        if ped != 0 {
            let pool = rd32(lf_checker_rt::relocated(PED_POOL));
            let obj: u32 = lf_checker_rt::callee_thiscall!(1, u32, pool, ped);
            let status = rd32(obj.wrapping_add(PED_STATUS));
            if status != 0 && rd8(status.wrapping_add(PED_BUSY_FLAG)) != 0 {
                return 0;
            }
        }
        let task_pool = rd32(lf_checker_rt::relocated(TASK_POOL));
        let half = rd32(lf_checker_rt::relocated(HALF));
        let two_a = rd32(lf_checker_rt::relocated(TWO_A));
        let xyz = [x, y, z];
        let xyz_ptr = xyz.as_ptr() as u32;
        let mut duration = 0u32;
        let task: u32;
        if mode == 0xFFFFFFFEu32 {
            let alloc: u32 = lf_checker_rt::callee_thiscall!(2, u32, task_pool);
            if alloc == 0 {
                task = 0;
            } else {
                task = lf_checker_rt::callee_thiscall!(4, u32, alloc, arg4, xyz_ptr, half, two_a, 0, 1);
            }
        } else {
            let def = rd32(lf_checker_rt::relocated(MODE_DEFAULT)) as i32;
            let m = if mode == 0xFFFFFFFFu32 { def } else { mode as i32 };
            let mut f = mul(m as f32, f32::from_bits(rd32(lf_checker_rt::relocated(MS_TO_S))));
            if f > 0.0 {
                f = add(f, f32::from_bits(rd32(lf_checker_rt::relocated(TWO_B))));
            }
            duration = f.to_bits();
            let alloc: u32 = lf_checker_rt::callee_thiscall!(2, u32, task_pool);
            if alloc == 0 {
                task = 0;
            } else {
                task = lf_checker_rt::callee_thiscall!(5, u32, alloc, arg4, xyz_ptr, half, two_a, def as u32);
            }
        }
        let wrap: u32 = lf_checker_rt::callee_thiscall!(3, u32, task_pool);
        if wrap == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(7, u32, ped, 0, TASK_SLOT);
        } else {
            let attached: u32 =
                lf_checker_rt::callee_thiscall!(6, u32, wrap, task, 0, 0, duration);
            let _: u32 = lf_checker_rt::callee_cdecl!(7, u32, ped, attached, TASK_SLOT);
        }
        0
    }
});
