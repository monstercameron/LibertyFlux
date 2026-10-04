// original: 0x00cb86d0 CTaskComplexMoveGotoAchieveHeading::vf5
/// Update a goto-achieve-heading task through two tables (vf5, 2 calls).
///
/// With a non-null `a2` (thiscall, three stack arguments; `a0` and `a1`
/// are unread), runs slot 4 of its table and returns the answer cleared
/// when it equals `0x80`. Otherwise, with a null subtask at `[this + 8]`
/// or bit 0 of `[sub + 0xC]` set, returns the incoming eax with its low
/// byte forced to 1 (the contract pins incoming eax to a constant).
/// Otherwise runs slot `0x14` of the subtask's table with (T, T, `a2`),
/// where the first two slots are read half-overlapping the return address
/// and below-stack scratch and are skipped by the contract: a zero low
/// byte returns the answer cleared, else sets bit 1 of `[sub + 0xC]` and
/// returns the answer with its low byte forced to 1. Both callees are
/// intercepted through planted tables.
lf_checker_rt::export!(thiscall, rw_00cb86d0(this: u32, _a0: u32, _a1: u32, a2: u32) -> u32 {
    unsafe {
        /// Subtask offset in this object.
        const SUB_OFF: u32 = 8;
        /// Subtask flag byte and bits.
        const FLAG_OFF: u32 = 0xC;
        const SKIP_BIT: u8 = 1;
        const DONE_BIT: u8 = 2;
        /// Table slots of the two callees.
        const SLOT_A: u32 = 4;
        const SLOT_B: u32 = 0x14;
        /// Answer that ends the update early with the answer cleared.
        const STOP_TYPE: u32 = 0x80;
        /// Pinned incoming eax (see contract).
        const IN_EAX: u32 = 0xA5A5A5A5;
        type Slot0 = extern "thiscall" fn(u32) -> u32;
        type Slot3 = extern "thiscall" fn(u32, u32, u32, u32) -> u32;
        if a2 != 0 {
            let va = (a2 as *const u32).read_unaligned();
            let sa = ((va + SLOT_A) as *const u32).read_unaligned();
            let fa: Slot0 = core::mem::transmute(sa as usize);
            let r = fa(a2);
            if r == STOP_TYPE {
                return r & 0xFFFFFF00;
            }
        }
        let d = ((this + SUB_OFF) as *const u32).read_unaligned();
        if d == 0 {
            return (IN_EAX & 0xFFFFFF00) | 1;
        }
        let fb = ((d + FLAG_OFF) as *const u8).read();
        if fb & SKIP_BIT != 0 {
            return (IN_EAX & 0xFFFFFF00) | 1;
        }
        let vb = (d as *const u32).read_unaligned();
        let sb = ((vb + SLOT_B) as *const u32).read_unaligned();
        let fb_fn: Slot3 = core::mem::transmute(sb as usize);
        let v = fb_fn(d, 0, 0, a2);
        if (v & 0xFF) == 0 {
            v & 0xFFFFFF00
        } else {
            ((d + FLAG_OFF) as *mut u8).write(fb | DONE_BIT);
            (v & 0xFFFFFF00) | 1
        }
    }
});
