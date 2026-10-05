// original: 0x00BBD270 ped_assign_sequence_goto_drive_to_point (proposed)

/// Build a sequence task (go-to-point plus drive-to-point) and assign it.
///
/// `ped` resolves through the ped pool like the sibling builders; a zero
/// handle skips the lookup and a busy flag returns early with no task.
///
/// `angle` is wrapped into `[0, 360)` exactly like the sibling (ordered
/// comparisons, NaN skips both loops) and converted to radians in place;
/// `m` below zero (ordered) becomes 0.1. Both are written back to the
/// incoming argument slots, so the stack check is off; the same values are
/// compared as drive-task call arguments.
///
/// Option flags are folded from four byte arguments and the sign of `a3c`
/// (`0x20` when `b2c` is set or `a3c` is positive, `0x400` when `b30` is
/// set, `0x8000` when `b38` is clear, `0x200` when `b34` is set), and a
/// further flag word is 1 when the start-mode global reads non-negative
/// (signed). A sequence task (callee 3) is allocated; a movement task
/// (callee 7) wraps a point-and-stand-still task (callee 5,
/// `(2, xyz_ptr, 0.5, 2.0, 0, 0)`) over the staged `(x, y, z)`; a
/// drive-to-point task (callee 10) is built as
/// `(xyz_ptr, angle_rad, m, a20, a24, flags, a28, flag, x_arg)` with
/// `x_arg` being `a3c` when positive else -1. The drive task gets `a40`
/// scaled to radians at `+0xDC`, both subtasks join the sequence
/// (callee 8, twice), and the sequence is assigned as `(ped, task, slot)`
/// with slot `TASK_SLOT` (callee 11). Failed allocations contribute null
/// subtasks; a failed drive allocation faults the original (a store through
/// null) and is covered by fault parity.
///
/// Original: 0x00BBD270 (cdecl, fifteen stack words, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00BBD270(ped: u32, x: u32, y: u32, z: u32, angle: u32, m: u32, a20: u32, a24: u32, a28: u32, b2c: u32, b30: u32, b34: u32, b38: u32, a3c: u32, a40: u32) -> u32 {
    unsafe {
        const PED_POOL: u32 = 0x018B6F1C;
        const TASK_POOL: u32 = 0x0167E2A0;
        const START_MODE: u32 = 0x010475C4;
        const FULL_CIRCLE: u32 = 0x00FE8C1C;
        const DEG_TO_RAD: u32 = 0x00FE8728;
        const MIN_M: u32 = 0x00FE879C;
        const HALF: u32 = 0x00ED7F28;
        const TWO: u32 = 0x00ED7F2C;
        const PED_STATUS: u32 = 0x6C;
        const PED_BUSY_FLAG: u32 = 0x0E;
        const TASK_DUR: u32 = 0xDC;
        const TASK_SLOT: u32 = 0x47;

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
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        if ped != 0 {
            let pool = rd32(lf_checker_rt::relocated(PED_POOL));
            let obj: u32 = lf_checker_rt::callee_thiscall!(1, u32, pool, ped);
            let status = rd32(obj.wrapping_add(PED_STATUS));
            if status != 0 && rd8(status.wrapping_add(PED_BUSY_FLAG)) != 0 {
                return 0;
            }
        }
        let circle = f32::from_bits(rd32(lf_checker_rt::relocated(FULL_CIRCLE)));
        let mut a = f32::from_bits(angle);
        while a < 0.0 {
            a = add(a, circle);
        }
        while a >= circle {
            a = sub(a, circle);
        }
        let arad = mul(a, f32::from_bits(rd32(lf_checker_rt::relocated(DEG_TO_RAD))));
        let mut mf = f32::from_bits(m);
        if mf < 0.0 {
            mf = f32::from_bits(rd32(lf_checker_rt::relocated(MIN_M)));
        }
        let mut flags = 0u32;
        if (b2c as u8) != 0 || (a3c as i32) > 0 {
            flags = 0x20;
        }
        if (b30 as u8) != 0 {
            flags |= 0x400;
        }
        if (b38 as u8) == 0 {
            flags |= 0x8000;
        }
        if (b34 as u8) != 0 {
            flags |= 0x200;
        }
        let mode_neg = (rd32(lf_checker_rt::relocated(START_MODE)) as i32) < 0;
        let flagword = if mode_neg { 0u32 } else { 1u32 };
        let task_pool = rd32(lf_checker_rt::relocated(TASK_POOL));
        let xyz = [x, y, z];
        let xyz_ptr = xyz.as_ptr() as u32;
        let seq_alloc: u32 = lf_checker_rt::callee_thiscall!(2, u32, task_pool);
        let seq: u32;
        if seq_alloc == 0 {
            seq = 0;
        } else {
            seq = lf_checker_rt::callee_thiscall!(3, u32, seq_alloc);
        }
        let goto_alloc: u32 = lf_checker_rt::callee_thiscall!(4, u32, task_pool);
        let goto_task: u32;
        if goto_alloc == 0 {
            goto_task = 0;
        } else {
            let half = rd32(lf_checker_rt::relocated(HALF));
            let two = rd32(lf_checker_rt::relocated(TWO));
            goto_task = lf_checker_rt::callee_thiscall!(5, u32, goto_alloc, 2, xyz_ptr, half, two, 0, 0);
        }
        let move_alloc: u32 = lf_checker_rt::callee_thiscall!(6, u32, task_pool);
        let move_task: u32;
        if move_alloc == 0 {
            move_task = 0;
        } else {
            move_task = lf_checker_rt::callee_thiscall!(7, u32, move_alloc, goto_task, 0, 0, 0);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(8, u32, seq, move_task);
        let x_arg = if (a3c as i32) > 0 { a3c } else { 0xFFFFFFFFu32 };
        let drive_alloc: u32 = lf_checker_rt::callee_thiscall!(9, u32, task_pool);
        let drive_task: u32;
        if drive_alloc == 0 {
            drive_task = 0;
        } else {
            drive_task = lf_checker_rt::callee_thiscall!(10, u32, drive_alloc, xyz_ptr, arad.to_bits(), mf.to_bits(), a20, a24, flags, a28, flagword, x_arg);
        }
        let d = mul(f32::from_bits(a40), f32::from_bits(rd32(lf_checker_rt::relocated(DEG_TO_RAD))));
        wr32(drive_task.wrapping_add(TASK_DUR), d.to_bits());
        let _: u32 = lf_checker_rt::callee_thiscall!(8, u32, seq, drive_task);
        let _: u32 = lf_checker_rt::callee_cdecl!(11, u32, ped, seq, TASK_SLOT);
        0
    }
});
