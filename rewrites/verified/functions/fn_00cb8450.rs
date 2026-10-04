// original: 0x00cb8450 CTaskComplexMovePatrolRoute::vf5
/// Update a patrol-route task from a route point (vf5, 1 call).
///
/// When `a1` is zero, zeroes the word at `[this + 0x24]` (thiscall, three
/// stack arguments); a null pointer there faults on both sides. Copies
/// the four route-point words at `[[a0 + 0x20] + 0x30 .. +0x3C]` into
/// `[this + 0x30 .. +0x3C]`. When bit 0 of `[sub + 0xC]` (sub is the
/// subtask at `[this + 8]`) is set the update counts as accepted at once;
/// otherwise runs slot `0x14` of the subtask's table with (`a0`, `a1`,
/// `a2`) and, on a non-zero low byte, sets bit 1 of `[sub + 0xC]`.
/// Finally forces bit 1 of `[this + 0x28]` to the accepted flag and
/// returns it (0 or 1). The callee is intercepted through a planted
/// table and answered by the checker.
lf_checker_rt::export!(thiscall, rw_00cb8450(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        /// Subtask, zero-target and status offsets in this object.
        const SUB_OFF: u32 = 8;
        const ZERO_OFF: u32 = 0x24;
        const STATUS_OFF: u32 = 0x28;
        /// Route-point offsets.
        const PT_OFF: u32 = 0x20;
        const PT_FIRST: u32 = 0x30;
        const DST_FIRST: u32 = 0x30;
        /// Subtask flag byte and bits.
        const FLAG_OFF: u32 = 0xC;
        const SKIP_BIT: u8 = 1;
        const DONE_BIT: u8 = 2;
        /// Table slot of the update callee.
        const SLOT: u32 = 0x14;
        type Slot3 = extern "thiscall" fn(u32, u32, u32, u32) -> u32;
        if a1 == 0 {
            let w = ((this + ZERO_OFF) as *const u32).read_unaligned();
            (w as *mut u32).write_unaligned(0);
        }
        let p = ((a0 + PT_OFF) as *const u32).read_unaligned();
        ((this + DST_FIRST) as *mut u32)
            .write_unaligned(((p + PT_FIRST) as *const u32).read_unaligned());
        ((this + DST_FIRST + 4) as *mut u32)
            .write_unaligned(((p + PT_FIRST + 4) as *const u32).read_unaligned());
        ((this + DST_FIRST + 8) as *mut u32)
            .write_unaligned(((p + PT_FIRST + 8) as *const u32).read_unaligned());
        ((this + DST_FIRST + 12) as *mut u32)
            .write_unaligned(((p + PT_FIRST + 12) as *const u32).read_unaligned());
        let d = ((this + SUB_OFF) as *const u32).read_unaligned();
        let fb = ((d + FLAG_OFF) as *const u8).read();
        let acc: u32 = if fb & SKIP_BIT != 0 {
            1
        } else {
            let va = (d as *const u32).read_unaligned();
            let sa = ((va + SLOT) as *const u32).read_unaligned();
            let f: Slot3 = core::mem::transmute(sa as usize);
            let v = f(d, a0, a1, a2);
            if (v & 0xFF) == 0 {
                0
            } else {
                ((d + FLAG_OFF) as *mut u8).write(fb | DONE_BIT);
                1
            }
        };
        let b = acc + acc;
        let s = ((this + STATUS_OFF) as *const u32).read_unaligned();
        let t = (s ^ b) & 2;
        ((this + STATUS_OFF) as *mut u32).write_unaligned(s ^ t);
        (((this + STATUS_OFF) as *const u32).read_unaligned() >> 1) & 1
    }
});
