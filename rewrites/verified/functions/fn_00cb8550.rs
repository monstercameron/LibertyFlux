// original: 0x00cb8550 CTaskComplexMoveFormation::vf5
/// Update a formation task through its subtask table (vf5, 1 call).
///
/// With a null subtask at `[this + 8]` or bit 0 of `[sub + 0xC]` set,
/// returns the incoming eax with its low byte forced to 1 (thiscall,
/// three stack arguments; the contract pins incoming eax to a constant,
/// so these paths are deterministic). Otherwise runs slot `0x14` of the
/// subtask's table with (`a0`, `a1`, `a2`): a zero low byte returns the
/// answer cleared, else sets bit 1 of `[sub + 0xC]` and returns the
/// answer with its low byte forced to 1. The callee is intercepted
/// through a planted table and answered by the checker.
lf_checker_rt::export!(thiscall, rw_00cb8550(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        /// Subtask offset in this object.
        const SUB_OFF: u32 = 8;
        /// Subtask flag byte and bits.
        const FLAG_OFF: u32 = 0xC;
        const SKIP_BIT: u8 = 1;
        const DONE_BIT: u8 = 2;
        /// Table slot of the update callee.
        const SLOT: u32 = 0x14;
        /// Pinned incoming eax (see contract).
        const IN_EAX: u32 = 0xA5A5A5A5;
        type Slot3 = extern "thiscall" fn(u32, u32, u32, u32) -> u32;
        let d = ((this + SUB_OFF) as *const u32).read_unaligned();
        if d == 0 {
            return (IN_EAX & 0xFFFFFF00) | 1;
        }
        let fb = ((d + FLAG_OFF) as *const u8).read();
        if fb & SKIP_BIT != 0 {
            return (IN_EAX & 0xFFFFFF00) | 1;
        }
        let va = (d as *const u32).read_unaligned();
        let sa = ((va + SLOT) as *const u32).read_unaligned();
        let f: Slot3 = core::mem::transmute(sa as usize);
        let v = f(d, a0, a1, a2);
        if (v & 0xFF) == 0 {
            v & 0xFFFFFF00
        } else {
            ((d + FLAG_OFF) as *mut u8).write(fb | DONE_BIT);
            (v & 0xFFFFFF00) | 1
        }
    }
});
