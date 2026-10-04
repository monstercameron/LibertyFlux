// original: 0x00cb7990 CTaskComplexMoveWander::vf6
/// Check whether a wander subtask can pass (vf6, 2 indirect calls).
///
/// Reads the subtask object twelve bytes below `this` (thiscall, no
/// stack arguments; the caller passes an interior pointer). A null object
/// returns the incoming eax with its low byte cleared (the contract pins
/// incoming eax to a constant, so this path is deterministic). Otherwise
/// runs slot `0x30` of the object's table on it, then slot `0x18` of the
/// answer's table, and returns the second answer with its low byte
/// replaced by whether that low byte was non-zero. Both callees are
/// intercepted through planted tables and answered by the checker.
lf_checker_rt::export!(thiscall, rw_00cb7990(this: u32) -> u32 {
    unsafe {
        /// Interior-pointer back-off to the subtask object.
        const BACK_OFF: u32 = 12;
        /// Table slots of the two callees.
        const SLOT_A: u32 = 0x30;
        const SLOT_B: u32 = 0x18;
        /// Pinned incoming eax (see contract); the null path clears its low byte.
        const IN_EAX: u32 = 0xA5A5A5A5;
        type Slot0 = extern "thiscall" fn(u32) -> u32;
        let c = (this.wrapping_sub(BACK_OFF) as *const u32).read_unaligned();
        if c == 0 {
            return IN_EAX & 0xFFFFFF00;
        }
        let fa = (c as *const u32).read_unaligned();
        let sa = ((fa + SLOT_A) as *const u32).read_unaligned();
        let fa_fn: Slot0 = core::mem::transmute(sa as usize);
        let e = fa_fn(c);
        let d = (e as *const u32).read_unaligned();
        let sb = ((d + SLOT_B) as *const u32).read_unaligned();
        let fb_fn: Slot0 = core::mem::transmute(sb as usize);
        let v = fb_fn(e);
        (v & 0xFFFFFF00) | u32::from((v & 0xFF) != 0)
    }
});
