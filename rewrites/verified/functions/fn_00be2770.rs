// original: 0x00be2770 CTaskComplexControlMovement::vf19

/// Movement-control task update: steer the pedestrian, or defer to the base task.
///
/// `this` is the complex task, `arg` the pedestrian. When the pedestrian has
/// no vehicle (`+0xB30` null) or is not flagged as inside one (bit `0x04` of
/// the byte at `+0x26C`), the update is not this task's to do: it tail-jumps
/// to the base task update with the same arguments and returns its result.
///
/// Otherwise the task marks itself active (bit `0x08` at `+0x28`), measures
/// the child task at `+0x14` (virtual slot `+0x30` yields a gauge object,
/// slot `+0x08` on that a float), and scores the gauge. It then allocates a
/// fresh movement task from the task pool (global pointer): a null pool
/// result returns 0, otherwise the new task is constructed, stamped with the
/// small-score flag (score `<= 2`), given its virtual table and zeroed
/// links, and returned.
///
/// Original: 0x00be2770 (thiscall, one stack word, returns a task pointer,
/// the base update's result, or 0). Ends in a tail jump; the bytes from
/// 0x00be27f6 on are padding up to the next function.
lf_checker_rt::export!(thiscall, rw_00be2770(this: u32, arg: u32) -> u32 {
    unsafe {
        const PED_VEHICLE: u32 = 0xb30;
        const PED_FLAGS: u32 = 0x26c;
        const INSIDE_VEHICLE: u8 = 0x04;
        const TASK_STATE: u32 = 0x28;
        const STATE_ACTIVE: u32 = 0x08;
        const TASK_CHILD: u32 = 0x14;
        const VT_GAUGE: u32 = 0x30;
        const VT_MEASURE: u32 = 0x08;
        const TASK_POOL: u32 = 0x167e2a0;
        const NEW_VTABLE: u32 = 0xe89044;
        const NEW_FLAG: u32 = 0x1a;
        const NEW_LINK: u32 = 0x14;
        const NEW_WORD: u32 = 0x18;
        const NEW_WORD_VALUE: u16 = 0x0101;
        const SMALL_SCORE: i32 = 2;

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
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }

        if rd32(arg + PED_VEHICLE) == 0 || rd8(arg + PED_FLAGS) & INSIDE_VEHICLE == 0 {
            return lf_checker_rt::callee_thiscall!(4, u32, this, arg);
        }
        wr32(this + TASK_STATE, rd32(this + TASK_STATE) | STATE_ACTIVE);
        let child = rd32(this + TASK_CHILD);
        let gauge_slot = rd32(rd32(child) + VT_GAUGE);
        let gauge_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(gauge_slot as usize);
        let gauge = gauge_of(child);
        let measure_slot = rd32(rd32(gauge) + VT_MEASURE);
        let measure: extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute(measure_slot as usize);
        // Score classifier: cdecl of the measured float (the caller cleans the
        // stack; the gauge pointer left in ecx is not an argument).
        let score = lf_checker_rt::callee_cdecl!(1, u32, measure(gauge).to_bits());
        let pool = rd32(lf_checker_rt::relocated(TASK_POOL));
        let small = if (score as i32) <= SMALL_SCORE { 1u8 } else { 0u8 };
        let obj = lf_checker_rt::callee_thiscall!(2, u32, pool);
        if obj == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(3, u32, obj);
        wr8(obj + NEW_FLAG, small);
        wr32(obj, lf_checker_rt::relocated(NEW_VTABLE));
        wr32(obj + NEW_LINK, 0);
        wr16(obj + NEW_WORD, NEW_WORD_VALUE);
        obj
    }
});
