// original: 0x00B81730 thread_slot_set_vt
/// Replace slot `idx`'s object, retiring the old one through hooks.
///
/// When the slot already holds `val` nothing happens. Otherwise the old
/// object (if any) is released (callee 1), the slot takes `val`, the set
/// is notified (callee 2) and the new chain tail is probed (callee 3):
/// a negative probe releases the new object again and clears the slot.
///
/// Original: 0x00B81730 (thiscall, two stack arguments, no return value).
lf_checker_rt::export!(thiscall, rw_00B81730(this: u32, val: u32, idx: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        const SLOTS: u32 = 0x14;
        let slot = this.wrapping_add(SLOTS).wrapping_add(idx.wrapping_mul(4));
        let old = rd32(slot);
        if old == val {
            return 0;
        }
        if old != 0 {
            lf_checker_rt::callee_thiscall!(1, u32, old, 1);
        }
        wr32(slot, val);
        lf_checker_rt::callee_thiscall!(2, u32, this, val);
        let cur = rd32(slot);
        if cur == 0 {
            return 0;
        }
        let mut tail = cur;
        loop {
            let nxt = rd32(tail + 8);
            if nxt == 0 {
                break;
            }
            tail = nxt;
        }
        let vt = rd32(tail);
        let target = rd32(vt + 8);
        let probe: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(target as usize) };
        if probe(tail) & 0xFF != 0 {
            return 0;
        }
        let victim = rd32(slot);
        if victim != 0 {
            lf_checker_rt::callee_thiscall!(1, u32, victim, 1);
        }
        wr32(slot, 0);
        0
    }
});
