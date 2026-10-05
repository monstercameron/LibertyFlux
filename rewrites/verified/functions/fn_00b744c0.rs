// original: 0x00b744c0 task_target_acquire (proposed)

/// Acquire the task's target object for `arg` and configure it by its kind.
///
/// Runs the locate callee (thiscall on `this`, one stack word: `arg`;
/// answer ignored), then the fetch callee (thiscall on `arg+0x78`, stack
/// words -1, 1000.0f, `this+0x1c`, `this+0x20`) and stores its answer at
/// `this+0x18`. Bit 15 of the answer's second word selects the configure
/// mode: set means mode 1, clear means mode 2. Either way calls the
/// configure callee (thiscall on the answer, stack words `this`, the
/// constant table 0xb6fe90, the mode) and returns its answer.
///
/// Original: 0x00b744c0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b744c0(this: u32, arg: u32) -> u32 {
    unsafe {
        const TARGET_OFF: u32 = 0x18;
        const FETCH_THIS_OFF: u32 = 0x78;
        const FETCH_A_OFF: u32 = 0x1c;
        const FETCH_B_OFF: u32 = 0x20;
        const KIND_WORD_OFF: u32 = 4;
        const KIND_BIT: u32 = 15;
        const FETCH_TIMEOUT: u32 = 0xffff_ffff;
        const FETCH_RANGE: u32 = 0x447a0000; // 1000.0f
        const TABLE_FILE_VA: u32 = 0xb6fe90;
        const LOCATE: u32 = 1;
        const FETCH: u32 = 2;
        const CONFIGURE: u32 = 3;
        let _: u32 = lf_checker_rt::callee_thiscall!(LOCATE, u32, this, arg);
        let fetch_this = ((arg + FETCH_THIS_OFF) as *const u32).read_unaligned();
        let fa = ((this + FETCH_A_OFF) as *const u32).read_unaligned();
        let fb = ((this + FETCH_B_OFF) as *const u32).read_unaligned();
        let target: u32 =
            lf_checker_rt::callee_thiscall!(FETCH, u32, fetch_this, fb, fa, FETCH_RANGE, FETCH_TIMEOUT);
        ((this + TARGET_OFF) as *mut u32).write_unaligned(target);
        let kind = ((target + KIND_WORD_OFF) as *const u32).read_unaligned();
        let mode = if (kind >> KIND_BIT) & 1 == 0 { 2u32 } else { 1u32 };
        let table = lf_checker_rt::relocated(TABLE_FILE_VA);
        lf_checker_rt::callee_thiscall!(CONFIGURE, u32, target, mode, table, this)
    }
});
