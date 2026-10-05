// original: 0x005dd9c0 CTaskComplexTreatAccident::vf20

/// Probe a treat-accident task's children, returning the first child or null.
///
/// Calls the slot-`+0x0c` method of the child at `+0x08` and of its own
/// child at `+0x14`, expecting `0x11d` then `0x3b6`; any mismatch or a
/// missing second child returns the first child. Otherwise, when the
/// member at `+0x14` is present and its busy bit (`+0x26c`, bit 2) is
/// clear, probes `+0x224 + 0x2e0` with `(0x51f, 0)`; a zero low byte
/// returns 0, else the first child. The stack argument is unread.
///
/// Original: 0x005dd9c0 (thiscall, one unread stack word).
lf_checker_rt::export!(thiscall, rw_005dd9c0(this: u32, arg: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        const WANT_A: u32 = 0x11d;
        const WANT_B: u32 = 0x3b6;
        const BUSY_BIT: u8 = 4;
        const KIND: u32 = 0x51f;
        const PROBE_A: u32 = 1;
        const PROBE_B: u32 = 2;
        const PROBE_C: u32 = 3;
        let _ = arg;
        let o1 = rd32(this + 8);
        let vt1 = rd32(o1);
        let pa: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(rd32(vt1 + 0x0c) as usize) };
        if pa(o1) != WANT_A {
            return o1;
        }
        let o2 = rd32(o1 + 0x14);
        if o2 == 0 {
            return o1;
        }
        let vt2 = rd32(o2);
        let pb: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(rd32(vt2 + 0x0c) as usize) };
        if pb(o2) != WANT_B {
            return o1;
        }
        let o3 = rd32(this + 0x14);
        if o3 == 0 {
            return 0;
        }
        if rd8(o3 + 0x26c) & BUSY_BIT != 0 {
            return 0;
        }
        let base = rd32(o3 + 0x224);
        let r = lf_checker_rt::callee_thiscall!(PROBE_C, u32, base.wrapping_add(0x2e0), KIND, 0);
        if (r as u8) == 0 {
            return 0;
        }
        o1
    }
});
