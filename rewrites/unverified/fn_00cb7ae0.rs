// original: 0x00cb7ae0 heading_filter_reset
/// Reset a heading filter through the float normaliser (1 call).
///
/// Passes the speed at `[this + 0x28]` (thiscall, no stack arguments)
/// through the float callee and stores the answer back into `+0x28` and
/// its copy at `+0x2C`. Then zeroes the scratch words, keeps flag bits
/// `~0x18C` and sets `0x50` at `+0xB0`, and copies the 8.0/20.0 globals
/// (file addresses `0x01050E78`/`0x01050E7C`) into `+0xA4`/`+0xA8`.
/// Returns the new flag word. The callee is intercepted by the checker
/// and answers on the x87 register.
lf_checker_rt::export!(thiscall, rw_00cb7ae0(this: u32) -> u32 {
    unsafe {
        use lf_checker_rt::global;
        /// Speed slot filtered by the callee, and its copy.
        const SPEED_OFF: u32 = 0x28;
        const SPEED_COPY: u32 = 0x2C;
        /// Flag word kept/set masks.
        const FLAG_OFF: u32 = 0xB0;
        const FLAG_KEEP: u32 = 0xFFFFFE73;
        const FLAG_SET: u32 = 0x50;
        /// Globals copied into +0xA4/+0xA8 (file VAs).
        const G_FIRST: u32 = 0x01050E78;
        const G_SECOND: u32 = 0x01050E7C;
        const DST_FIRST: u32 = 0xA4;
        const DST_SECOND: u32 = 0xA8;
        /// Callee id of the float normaliser.
        const NORMALISE: u32 = 1;
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let arg = rd(this + SPEED_OFF);
        let v: f32 = lf_checker_rt::callee_cdecl!(NORMALISE, f32, arg);
        wr(this + SPEED_OFF, v.to_bits());
        wr(this + SPEED_COPY, v.to_bits());
        let f = rd(this + FLAG_OFF);
        let f2 = (f & FLAG_KEEP) | FLAG_SET;
        wr(this + 0x3C, 0);
        wr(this + 0x44, 0);
        wr(this + 0x64, 0);
        wr(this + 0xAC, 0);
        wr(this + 0x60, 0);
        wr(this + FLAG_OFF, f2);
        wr(this + 0x20, 0);
        wr(this + 0x40, 0);
        wr(this + DST_FIRST, *global::<u32>(G_FIRST));
        wr(this + DST_SECOND, *global::<u32>(G_SECOND));
        wr(this + 0x24, 0);
        f2
    }
});
