// original: 0x00cbbbc0 speed_apply_unless_flag
/// Apply the stored speed unless the chain flag says skip (1 call).
///
/// Loads the flag at `[[a0 + 0xA80] + 0x50]` (thiscall, one stack
/// argument) and shifts it right by one. When bit 0 of the shifted value
/// is set, returns it at once; otherwise calls the speed applier with
/// (`a0`, the float at `[this + 0x20]`) and returns its answer. The callee
/// is intercepted and answered by the checker.
lf_checker_rt::export!(thiscall, rw_00cbbbc0(this: u32, a0: u32) -> u32 {
    unsafe {
        /// Chain and flag offsets in the argument's objects.
        const CHAIN_OFF: u32 = 0xA80;
        const FLAG_OFF: u32 = 0x50;
        /// Speed slot in this object passed to the callee.
        const SPEED_OFF: u32 = 0x20;
        /// Callee id of the speed applier.
        const APPLY: u32 = 1;
        let o = ((a0 + CHAIN_OFF) as *const u32).read_unaligned();
        let fl = ((o + FLAG_OFF) as *const u32).read_unaligned();
        let s = fl >> 1;
        if (s & 1) != 0 {
            return s;
        }
        let v = ((this + SPEED_OFF) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(APPLY, u32, a0, v)
    }
});
