// original: 0x00b32bb0 subtask_clear (proposed)

/// Clear a sub-task record back to its idle state.
///
/// `this` points to the record. The id word, the four words at `+0x10` to
/// `+0x1c`, the two words at `+0x20` and `+0x24`, both child pointers at
/// `+0x28` and `+0x2c`, the two words at `+0x30` and `+0x34`, and the word at
/// `+0x44` are all zeroed. Each non-null child pointer is first released
/// through a callee that takes the pointer's own address. When the timer flag
/// byte at `+0x40` is set, the elapsed byte at `+0x41` is set and the spare
/// word at `+0x3c` is reduced by the shared tick count minus the stamp word
/// at `+0x38` (all wrapping). Returns that same difference when the timer
/// flag was set, otherwise the last callee answer, or 0 when no callee ran.
///
/// Original: 0x00b32bb0 (thiscall, no stack words; one one-word callee
/// reached from two sites).
lf_checker_rt::export!(thiscall, rw_00b32bb0(this: u32) -> u32 {
    unsafe {
        const CHILD_A: u32 = 0x28;
        const CHILD_B: u32 = 0x2c;
        const STAMP: u32 = 0x38;
        const SPARE: u32 = 0x3c;
        const TIMER_FLAG: u32 = 0x40;
        const ELAPSED: u32 = 0x41;
        const TICKS: u32 = 0x011735b4;
        const RELEASE: u32 = 1;
        #[inline(always)]
        unsafe fn w32(o: u32, off: u32, v: u32) {
            unsafe { (o as *mut u32).byte_add(off as usize).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn r32(o: u32, off: u32) -> u32 {
            unsafe { (o as *const u32).byte_add(off as usize).read_unaligned() }
        }
        w32(this, 0x00, 0);
        w32(this, 0x10, 0);
        w32(this, 0x14, 0);
        w32(this, 0x18, 0);
        w32(this, 0x20, 0);
        w32(this, 0x24, 0);
        let mut answer = 0u32;
        if r32(this, CHILD_A) != 0 {
            answer = lf_checker_rt::callee_thiscall!(
                RELEASE,
                u32,
                r32(this, CHILD_A),
                this.wrapping_add(CHILD_A)
            );
        }
        if r32(this, CHILD_B) != 0 {
            answer = lf_checker_rt::callee_thiscall!(
                RELEASE,
                u32,
                r32(this, CHILD_B),
                this.wrapping_add(CHILD_B)
            );
        }
        w32(this, CHILD_A, 0);
        w32(this, CHILD_B, 0);
        w32(this, 0x30, 0);
        w32(this, 0x34, 0);
        if (this as *const u8).byte_add(TIMER_FLAG as usize).read() != 0 {
            (this as *mut u8).byte_add(ELAPSED as usize).write(1);
            let ticks = lf_checker_rt::global::<u32>(TICKS).read();
            let elapsed = ticks.wrapping_sub(r32(this, STAMP));
            w32(this, SPARE, r32(this, SPARE).wrapping_sub(elapsed));
            answer = elapsed;
        }
        w32(this, 0x44, 0);
        answer
    }
});
