// original: 0x00cb7b80 formation_task_init
/// Initialise a formation task block, then filter the argument (1 call).
///
/// Writes the default block into the task object (`this`, thiscall, one
/// float stack argument): flag bits at `+0x54` (keep `~0x3CC`, set
/// `0x30`), the tick global (file address `0x011735B4`) into `+0x74` and
/// `+0x58`, `8.0`/`20.0` at `+0x24`/`+0x28`, `2000` at `+0x5C`, flag byte
/// at `+0x60`, zeroes at `+0x30`/`+0x34`/`+0x38`/`+0x70`, and `-1001.5`
/// bits at `+0x18`. Slot `+0x3C` receives a float the original reads from
/// its own realigned stack frame, which no caller writes; under the
/// checker's defined stack fill of 0 that value is `0.0` on both sides.
/// Then calls the filter callee with (`this`, `a0`) and returns the tick.
/// The callee is intercepted by the checker.
lf_checker_rt::export!(thiscall, rw_00cb7b80(this: u32, a0: u32) -> u32 {
    unsafe {
        use lf_checker_rt::global;
        /// Tick global copied into +0x74/+0x58 and returned (file VA).
        const TICK_G: u32 = 0x011735B4;
        const FLAG_OFF: u32 = 0x54;
        const FLAG_KEEP: u32 = 0xFFFFFC33;
        const FLAG_SET: u32 = 0x30;
        /// The unread stack slot reads the defined fill (0.0) on both sides.
        const FILL_SLOT: u32 = 0x00000000;
        /// Callee id of the filter.
        const FILTER: u32 = 1;
        #[inline(always)]
        unsafe fn wr(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let f = ((this + FLAG_OFF) as *const u32).read_unaligned();
        wr(this + 0x70, 0);
        wr(this + FLAG_OFF, (f & FLAG_KEEP) | FLAG_SET);
        let t1 = *global::<u32>(TICK_G);
        wr(this + 0x74, t1);
        wr(this + 0x24, 0x41000000);
        wr(this + 0x28, 0x41A00000);
        let t2 = *global::<u32>(TICK_G);
        wr(this + 0x58, t2);
        wr(this + 0x5C, 0x7D0);
        ((this + 0x60) as *mut u8).write(1);
        wr(this + 0x3C, FILL_SLOT);
        wr(this + 0x30, 0);
        wr(this + 0x34, 0);
        wr(this + 0x38, 0);
        wr(this + 0x18, 0xC479C000);
        let _: u32 = lf_checker_rt::callee_thiscall!(FILTER, u32, this, a0);
        t2
    }
});
