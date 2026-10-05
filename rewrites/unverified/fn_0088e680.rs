// original: 0x0088E680 rage::audVoiceSoft::vf4

/// Resume this software voice on its child voice, unless it is not marked
/// for resume.
///
/// `this` points to the voice. When bit 3 of the flag byte at `+0x8c` is
/// clear there is nothing to do. Otherwise the voice's own restart entry
/// (callee 1) runs first; then the child at `+0x130` is told to resume
/// (callee 2) with a mode bit folded out of the voice flags (bit 1 of the
/// flags or-ed with bit 4 shifted down); finally bit 3 of the voice flags
/// is cleared.
///
/// Original: 0x0088E680 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_0088E680(this: u32) -> ()) {
    unsafe {
        const FLAGS: u32 = 0x8c;
        const NEEDS_RESUME: u8 = 0x08;
        const CHILD: u32 = 0x130;
        const RESTART_SELF: u32 = 1;
        const RESUME_CHILD: u32 = 2;

        let flags = ((this + FLAGS) as *const u8).read();
        if flags & NEEDS_RESUME == 0 {
            return;
        }
        lf_checker_rt::callee_thiscall!(RESTART_SELF, u32, this);
        let flags = ((this + FLAGS) as *const u8).read();
        let mode = (((flags >> 3) | flags) >> 1) & 1;
        let obj = ((this + CHILD) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(RESUME_CHILD, u32, obj, mode as u32);
        let flags = (this + FLAGS) as *mut u8;
        flags.write(flags.read() & !NEEDS_RESUME);
    }
});
