// original: 0x00cb84d0 CTaskComplexMoveTimedGoto::vf5
/// Update a timed-goto task through two table calls (vf5, 2 calls).
///
/// When bit 0 of `[sub + 0xC]` (sub is the subtask at `[this + 8]`) is
/// clear, runs slot `0x14` of the subtask's table with (saved-ebp slot,
/// `a1`, `a2`) (thiscall, three stack arguments; `a0` is unread) and, on
/// a zero low byte, returns the answer cleared; otherwise sets bit 1 of
/// `[sub + 0xC]`. The first slot forwards the original's incoming ebp,
/// a caller value the rewrite cannot read, so the rewrite passes zero
/// and the contract skips it. With `a1 == 1` and a non-null `a2`, runs
/// slot `0x4C` of `a2`'s table; a non-zero low byte returns set. With no
/// call having run, the return carries the incoming eax, which the
/// contract pins to a constant. Otherwise, when `[this + 0x4C]` is
/// non-zero, writes 1 to `[this + 0x4D]` and subtracts
/// (tick - `[this + 0x44]`) from `[this + 0x48]`, where tick is the global
/// at file address `0x011735B4`. Both callees are intercepted through
/// planted tables.
lf_checker_rt::export!(thiscall, rw_00cb84d0(this: u32, _a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        use lf_checker_rt::global;
        /// Subtask and timer offsets in this object.
        const SUB_OFF: u32 = 8;
        const T0_OFF: u32 = 0x44;
        const T1_OFF: u32 = 0x48;
        const ARM_OFF: u32 = 0x4C;
        const SET_OFF: u32 = 0x4D;
        /// Subtask flag byte and bits.
        const FLAG_OFF: u32 = 0xC;
        const SKIP_BIT: u8 = 1;
        const DONE_BIT: u8 = 2;
        /// Table slots of the two callees.
        const SLOT_A: u32 = 0x14;
        const SLOT_B: u32 = 0x4C;
        /// Tick global (file VA).
        const TICK_G: u32 = 0x011735B4;
        /// Pinned incoming eax (see contract).
        const IN_EAX: u32 = 0xA5A5A5A5;
        type Slot3 = extern "thiscall" fn(u32, u32, u32, u32) -> u32;
        type Slot0 = extern "thiscall" fn(u32) -> u32;
        let d = ((this + SUB_OFF) as *const u32).read_unaligned();
        let fb = ((d + FLAG_OFF) as *const u8).read();
        let mut e: u32;
        if fb & SKIP_BIT == 0 {
            let va = (d as *const u32).read_unaligned();
            let sa = ((va + SLOT_A) as *const u32).read_unaligned();
            let f: Slot3 = core::mem::transmute(sa as usize);
            let v = f(d, 0, a1, a2);
            if (v & 0xFF) == 0 {
                return v & 0xFFFFFF00;
            }
            ((d + FLAG_OFF) as *mut u8).write(fb | DONE_BIT);
            e = v;
        } else {
            e = IN_EAX;
        }
        if a1 != 1 {
            return (e & 0xFFFFFF00) | 1;
        }
        if a2 != 0 {
            let vb = (a2 as *const u32).read_unaligned();
            let sb = ((vb + SLOT_B) as *const u32).read_unaligned();
            let g: Slot0 = core::mem::transmute(sb as usize);
            let w = g(a2);
            if (w & 0xFF) != 0 {
                return (w & 0xFFFFFF00) | 1;
            }
            e = w;
        }
        if ((this + ARM_OFF) as *const u8).read() == 0 {
            return (e & 0xFFFFFF00) | 1;
        }
        ((this + SET_OFF) as *mut u8).write(1);
        let diff =
            (*global::<u32>(TICK_G)).wrapping_sub(
                ((this + T0_OFF) as *const u32).read_unaligned());
        let t1 = ((this + T1_OFF) as *const u32).read_unaligned();
        ((this + T1_OFF) as *mut u32).write_unaligned(t1.wrapping_sub(diff));
        (diff & 0xFFFFFF00) | 1
    }
});
