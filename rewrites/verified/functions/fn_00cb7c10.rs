// original: 0x00cb7c10 slide_task_state_init
/// Initialise a slide-task state block with its defaults (leaf).
///
/// Writes the default word block into the task object (`this`, thiscall,
/// no stack arguments): flag bits at `+0xC4` (keep `~0xFFFF7FF0`, set
/// `0x80008`), three near-distance words just under `10000.0` at `+0x60`,
/// `+0x64`, `+0x68`, zeroes at `+0x70` through `+0x88`, at `+0xAC`,
/// `+0xB8`, a zero half-word at `+0xC0`, flag bytes at `+0xB0`/`+0xBC`,
/// the 2.0-style constant copied from the global at file address
/// `0x01050E30` into `+0xA4`, and the tick global at `0x011735B4` into
/// `+0xA8` and `+0xB4` (the value is also returned in eax). Three slots
/// (`+0x6C`, `+0x7C`, `+0x8C`) receive a float the original reads from its
/// own realigned stack frame, which no caller writes; under the checker's
/// defined stack fill of 0 that value is `0.0` on both sides. No calls.
lf_checker_rt::export!(thiscall, rw_00cb7c10(this: u32) -> u32 {
    unsafe {
        use lf_checker_rt::global;
        /// Global holding the copied 2.0-style default (file VA).
        const DEFAULT_G: u32 = 0x01050E30;
        /// Global tick copied into +0xA8/+0xB4 and returned (file VA).
        const TICK_G: u32 = 0x011735B4;
        /// Near-distance default (bits just under 10000.0).
        const NEAR: u32 = 0x461C3C00;
        /// The unread stack slot reads the defined fill (0.0) on both sides.
        const FILL_SLOT: u32 = 0x00000000;
        const FLAG_KEEP: u32 = 0xFFFF800F;
        const FLAG_SET: u32 = 0x00080008;
        #[inline(always)]
        unsafe fn wr(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let f = ((this + 0xC4) as *const u32).read_unaligned();
        wr(this + 0xC4, (f & FLAG_KEEP) | FLAG_SET);
        wr(this + 0x6C, FILL_SLOT);
        wr(this + 0x60, NEAR);
        wr(this + 0x64, NEAR);
        wr(this + 0x68, NEAR);
        wr(this + 0xA4, *global::<u32>(DEFAULT_G));
        wr(this + 0x7C, FILL_SLOT);
        wr(this + 0x70, 0);
        wr(this + 0x74, 0);
        wr(this + 0x78, 0);
        wr(this + 0x80, 0);
        wr(this + 0x84, 0);
        wr(this + 0x88, 0);
        wr(this + 0x8C, FILL_SLOT);
        ((this + 0xC0) as *mut u16).write_unaligned(0);
        let tick = *global::<u32>(TICK_G);
        wr(this + 0xA8, tick);
        wr(this + 0xAC, 0);
        ((this + 0xB0) as *mut u8).write(1);
        let tick2 = *global::<u32>(TICK_G);
        wr(this + 0xB4, tick2);
        wr(this + 0xB8, 0);
        ((this + 0xBC) as *mut u8).write(1);
        tick2
    }
});
