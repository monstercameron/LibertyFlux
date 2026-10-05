// original: 0x00cba1a0 CTaskComplexMoveGoToPointAnyMeans::vf17
/// Check a route fallback through two callees (vf17, 2 calls).
///
/// Runs slot `0x50` of the task's own table with (`this`, `a0`)
/// (thiscall, one stack argument). A non-zero low byte returns the
/// answer with its low byte forced to 1; a zero low byte runs the
/// direct callee with the same arguments and returns its answer with
/// its low byte cleared (the original's `(an instruction of the original)` clears only the
/// low byte). The indirect callee is intercepted through a planted
/// table, the direct one by patching.
lf_checker_rt::export!(thiscall, rw_00cba1a0(this: u32, a0: u32) -> u32 {
    unsafe {
        /// Table slot of the indirect check.
        const SLOT: u32 = 0x50;
        /// Direct callee id.
        const FALLBACK: u32 = 2;
        type Slot1 = extern "thiscall" fn(u32, u32) -> u32;
        let va = (this as *const u32).read_unaligned();
        let sa = ((va + SLOT) as *const u32).read_unaligned();
        let f: Slot1 = core::mem::transmute(sa as usize);
        let v = f(this, a0);
        if (v & 0xFF) != 0 {
            return (v & 0xFFFFFF00) | 1;
        }
        let w: u32 = lf_checker_rt::callee_thiscall!(FALLBACK, u32, this, a0);
        w & 0xFFFFFF00
    }
});
