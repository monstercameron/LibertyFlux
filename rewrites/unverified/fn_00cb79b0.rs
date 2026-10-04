// original: 0x00cb79b0 move_task_state_init
/// Initialise a move-task state block with its default constants (leaf).
///
/// Writes the default word block into the task object (`this`, thiscall,
/// no stack arguments): mode words at `+0x70`/`+0x74`, flag word at `+0x78`,
/// three far-distance words (`10000.0`) at `+0x40`/`+0x44`/`+0x48` and
/// `+0x90`/`+0x94`/`+0x98`, a `2.5` step at `+0x6C`, zeroes at `+0x50`,
/// `+0x54`, `+0x58`, `+0x68`, `+0xAC`, `+0xB0`, and clears all but bits 0
/// and 7 of the flag word at `+0xB4`. Three slots (`+0x4C`, `+0x9C`, `+0x5C`)
/// receive a float the original reads from its own realigned stack frame,
/// which no caller writes; under the checker's defined stack fill of 0
/// that value is `0.0` on both sides, so the rewrite stores `0.0`. No
/// return value (the original never writes eax), no calls.
lf_checker_rt::export!(thiscall, rw_00cb79b0(this: u32) -> u32 {
    unsafe {
        /// Far distance used for the default bounds (10000.0).
        const FAR: u32 = 0x461C4000;
        /// Default speed step (2.5).
        const STEP: u32 = 0x40200000;
        /// The unread stack slot reads the defined fill (0.0) on both sides.
        const FILL_SLOT: u32 = 0x00000000;
        /// Kept bits of the +0xB4 flag word.
        const KEEP_MASK: u32 = 0xFFFFFF81;
        #[inline(always)]
        unsafe fn wr(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        wr(this + 0x74, 3);
        wr(this + 0x70, 3);
        wr(this + 0x78, 1);
        wr(this + 0x4C, FILL_SLOT);
        wr(this + 0x40, FAR);
        wr(this + 0x44, FAR);
        wr(this + 0x48, FAR);
        wr(this + 0x9C, FILL_SLOT);
        wr(this + 0x90, FAR);
        wr(this + 0x94, FAR);
        wr(this + 0x98, FAR);
        let f = ((this + 0xB4) as *const u32).read_unaligned();
        wr(this + 0xB4, f & KEEP_MASK);
        wr(this + 0xAC, 0);
        wr(this + 0xB0, 0);
        wr(this + 0x6C, STEP);
        wr(this + 0x68, 0);
        wr(this + 0x50, 0);
        wr(this + 0x54, 0);
        wr(this + 0x58, 0);
        wr(this + 0x5C, FILL_SLOT);
        0
    }
});
