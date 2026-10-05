// original: 0x00BBD490 ped_assign_sequence_goto_achieve_heading (proposed)

/// Build a sequence task (go-to-point plus achieve-heading) and assign it.
///
/// `ped` resolves through the ped pool like the sibling builders; a zero
/// handle skips the lookup and a busy flag returns early with no task.
///
/// `angle` is wrapped into `[0, 360)` by repeated addition/subtraction of
/// 360 (NaN skips both loops; each loop is an exact ordered comparison) and
/// converted to radians in place; `m` below zero (ordered) becomes 0.1.
/// Both are written back to the incoming argument slots, so the stack check
/// is off for this function; the same values are compared as call arguments.
///
/// A sequence task (callee 3) is allocated and initialised on a pool object;
/// a movement task (callee 7) wraps a point-and-stand-still task (callee 6,
/// `(2, xyz_ptr, 0.5, 2.0, 0, 0)`) on another pool object, where `xyz_ptr`
/// points at the `(x, y, z)` arguments staged in the frame (compared through
/// a 3-word call-time snapshot with the address skipped). An achieve-heading
/// task (callee 9)
/// is built as `(xyz_ptr, angle_rad, m)` over the staged `(x, y, z)`, gets
/// `dur` scaled to radians at `+0xDC`, and joins the sequence with the
/// movement task (callee 8, twice). The sequence is assigned as
/// `(ped, task, slot)` with slot `TASK_SLOT` (callee 10). A failed movement
/// allocation skips the movement branch; a failed sequence allocation
/// assigns null. A failed heading allocation faults the original (a store
/// through null) and is covered by fault parity.
///
/// Original: 0x00BBD490 (cdecl, seven stack words, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00BBD490(ped: u32, x: u32, y: u32, z: u32, angle: u32, m: u32, dur: u32) -> u32 {
    unsafe {
        const PED_POOL: u32 = 0x018B6F1C;
        const TASK_POOL: u32 = 0x0167E2A0;
        const FULL_CIRCLE: u32 = 0x00FE8C1C;
        const DEG_TO_RAD: u32 = 0x00FE8728;
        const MIN_M: u32 = 0x00FE879C;
        const HALF: u32 = 0x00ED7F28;
        const TWO: u32 = 0x00ED7F2C;
        const PED_STATUS: u32 = 0x6C;
        const PED_BUSY_FLAG: u32 = 0x0E;
        const TASK_DUR: u32 = 0xDC;
        const TASK_SLOT: u32 = 0x44;

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
        let move_alloc: u32 = lf_checker_rt::callee_thiscall!(4, u32, task_pool);
        if move_alloc == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(8, u32, seq, 0);
        } else {
            let goto_alloc: u32 = lf_checker_rt::callee_thiscall!(5, u32, task_pool);
            let goto_task: u32;
            if goto_alloc == 0 {
                goto_task = 0;
            } else {
                let half = rd32(lf_checker_rt::relocated(HALF));
                let two = rd32(lf_checker_rt::relocated(TWO));
                goto_task = lf_checker_rt::callee_thiscall!(6, u32, goto_alloc, 2, xyz_ptr, half, two, 0, 0);
            }
            let move_task: u32 =
                lf_checker_rt::callee_thiscall!(7, u32, move_alloc, goto_task, 0, 0, 0);
            let _: u32 = lf_checker_rt::callee_thiscall!(8, u32, seq, move_task);
        }
        let head_alloc: u32 = lf_checker_rt::callee_thiscall!(11, u32, task_pool);
        let head_task: u32;
        if head_alloc == 0 {
            head_task = 0;
        } else {
            head_task = lf_checker_rt::callee_thiscall!(9, u32, head_alloc, xyz_ptr, arad.to_bits(), mf.to_bits());
        }
        let d = mul(f32::from_bits(dur), f32::from_bits(rd32(lf_checker_rt::relocated(DEG_TO_RAD))));
        wr32(head_task.wrapping_add(TASK_DUR), d.to_bits());
        let _: u32 = lf_checker_rt::callee_thiscall!(8, u32, seq, head_task);
        let _: u32 = lf_checker_rt::callee_cdecl!(10, u32, ped, seq, TASK_SLOT);
        0
    }
});
