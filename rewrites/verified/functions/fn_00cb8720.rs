// original: 0x00cb8720 CTaskComplexMoveGoToShelterAndWait::vf5
/// Update a shelter-wait task through two callees (vf5, 2 calls).
///
/// With a null subtask at `[this + 8]` returns the incoming eax with its
/// low byte cleared (thiscall, three stack arguments; the contract pins
/// incoming eax to a constant). When bit 0 of `[sub + 0xC]` is clear,
/// runs slot `0x14` of the subtask's table with (`a0`, `a1`, `a2`): a
/// zero low byte returns the answer cleared, else sets bit 1 of
/// `[sub + 0xC]`. Then, when `[this + 0x30]` is non-null, runs the direct
/// callee with it (its answer's upper bytes survive into the return) and
/// returns with the low byte forced to 1; a null word returns 1 at once
/// (the loaded zero with its low byte forced to 1). The indirect callee
/// is intercepted through a planted table, the direct one by patching.
lf_checker_rt::export!(thiscall, rw_00cb8720(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        /// Subtask offset and follow-up word in this object.
        const SUB_OFF: u32 = 8;
        const NEXT_OFF: u32 = 0x30;
        /// Subtask flag byte and bits.
        const FLAG_OFF: u32 = 0xC;
        const SKIP_BIT: u8 = 1;
        const DONE_BIT: u8 = 2;
        /// Table slot of the indirect callee.
        const SLOT: u32 = 0x14;
        /// Pinned incoming eax (see contract).
        const IN_EAX: u32 = 0xA5A5A5A5;
        /// Direct callee id.
        const FOLLOW: u32 = 2;
        type Slot3 = extern "thiscall" fn(u32, u32, u32, u32) -> u32;
        let d = ((this + SUB_OFF) as *const u32).read_unaligned();
        if d == 0 {
            return IN_EAX & 0xFFFFFF00;
        }
        let fb = ((d + FLAG_OFF) as *const u8).read();
        if fb & SKIP_BIT == 0 {
            let va = (d as *const u32).read_unaligned();
            let sa = ((va + SLOT) as *const u32).read_unaligned();
            let f: Slot3 = core::mem::transmute(sa as usize);
            let v = f(d, a0, a1, a2);
            if (v & 0xFF) == 0 {
                return v & 0xFFFFFF00;
            }
            ((d + FLAG_OFF) as *mut u8).write(fb | DONE_BIT);
        }
        let n = ((this + NEXT_OFF) as *const u32).read_unaligned();
        if n == 0 {
            return (n & 0xFFFFFF00) | 1;
        }
        let w: u32 = lf_checker_rt::callee_cdecl!(FOLLOW, u32, n);
        (w & 0xFFFFFF00) | 1
    }
});
