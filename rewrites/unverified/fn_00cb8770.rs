// original: 0x00cb8770 CTaskComplexMoveWander::vf5
/// Update a wander task through three tables and a sink (vf5, 4 calls).
///
/// With a non-null `a2` (thiscall, three stack arguments; `a0` and `a1`
/// are unread), runs slot 4 of its table. A `6` answer runs the flag/C
/// chain: a set bit `0x80` in `[this + 0xB0]` returns cleared, a null
/// subtask at `[this + 8]` returns cleared (never taken under this
/// contract), else slot `0xC` of the subtask's table runs: `0x3BE`
/// returns cleared, anything else sets bit `0x80`, bumps `[a2 + 4]` and
/// returns cleared. Any other `a2` answer, or a null `a2`, takes the
/// B-side: slot `0x14` of the subtask's table runs with (saved-esi slot,
/// scratch slot, `a2`), where the first two slots come from the original's
/// saved registers and scratch and are skipped by the contract; a zero low
/// byte returns the answer cleared, else bit 1 of `[sub + 0xC]` is set.
/// Then a non-null `[this + 0x44]` runs the direct sink, `[this + 0x44]`
/// is cleared, bit 4 of `[this + 0xB0]` is set, and the last answer
/// returns with its low byte forced to 1. The return carries the last
/// callee answer, or the pinned incoming eax on a path the contract never
/// takes. All callees are intercepted (three through planted tables).
lf_checker_rt::export!(thiscall, rw_00cb8770(this: u32, _a0: u32, _a1: u32, a2: u32) -> u32 {
    unsafe {
        /// Subtask, sink-word and flag offsets in this object.
        const SUB_OFF: u32 = 8;
        const NEXT_OFF: u32 = 0x44;
        const FLAG_OFF: u32 = 0xB0;
        /// Subtask flag byte bits.
        const SUB_FLAG_OFF: u32 = 0xC;
        const SKIP_BIT: u8 = 1;
        const DONE_BIT: u8 = 2;
        /// Flag bits in +0xB0.
        const SEEN_BIT: u8 = 0x80;
        const SINK_BIT: u8 = 4;
        /// Table slots.
        const SLOT_A: u32 = 4;
        const SLOT_C: u32 = 0xC;
        const SLOT_B: u32 = 0x14;
        /// Distinguishing answers.
        const SIX: u32 = 6;
        const TYPE_BAD: u32 = 0x3BE;
        /// Pinned incoming eax (see contract; dead-path carry only).
        const IN_EAX: u32 = 0xA5A5A5A5;
        /// Direct sink id.
        const SINK: u32 = 4;
        type Slot0 = extern "thiscall" fn(u32) -> u32;
        type Slot3 = extern "thiscall" fn(u32, u32, u32, u32) -> u32;
        let mut e: u32 = IN_EAX;
        if a2 != 0 {
            let va = (a2 as *const u32).read_unaligned();
            let sa = ((va + SLOT_A) as *const u32).read_unaligned();
            let fa: Slot0 = core::mem::transmute(sa as usize);
            let r = fa(a2);
            if r == SIX {
                let fl = ((this + FLAG_OFF) as *const u8).read();
                if fl & SEEN_BIT != 0 {
                    return r & 0xFFFFFF00;
                }
                let sub = ((this + SUB_OFF) as *const u32).read_unaligned();
                if sub == 0 {
                    return r & 0xFFFFFF00;
                }
                let vc = (sub as *const u32).read_unaligned();
                let sc = ((vc + SLOT_C) as *const u32).read_unaligned();
                let fc: Slot0 = core::mem::transmute(sc as usize);
                let r2 = fc(sub);
                if r2 == TYPE_BAD {
                    return r2 & 0xFFFFFF00;
                }
                ((this + FLAG_OFF) as *mut u8).write(fl | SEEN_BIT);
                let c4 = ((a2 + 4) as *const u32).read_unaligned();
                ((a2 + 4) as *mut u32).write_unaligned(c4.wrapping_add(1));
                return r2 & 0xFFFFFF00;
            }
            e = r;
        }
        let b = ((this + SUB_OFF) as *const u32).read_unaligned();
        let fb = ((b + SUB_FLAG_OFF) as *const u8).read();
        if fb & SKIP_BIT == 0 {
            let vb = (b as *const u32).read_unaligned();
            let sb = ((vb + SLOT_B) as *const u32).read_unaligned();
            let fb_fn: Slot3 = core::mem::transmute(sb as usize);
            let w = fb_fn(b, 0, 0, a2);
            if (w & 0xFF) == 0 {
                return w & 0xFFFFFF00;
            }
            ((b + SUB_FLAG_OFF) as *mut u8).write(fb | DONE_BIT);
            e = w;
        }
        let n = ((this + NEXT_OFF) as *const u32).read_unaligned();
        if n != 0 {
            e = lf_checker_rt::callee_cdecl!(SINK, u32, n);
        }
        ((this + NEXT_OFF) as *mut u32).write_unaligned(0);
        let fl2 = ((this + FLAG_OFF) as *const u8).read();
        ((this + FLAG_OFF) as *mut u8).write(fl2 | SINK_BIT);
        (e & 0xFFFFFF00) | 1
    }
});
