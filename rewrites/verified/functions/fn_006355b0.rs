// original: 0x006355b0 reloc_single (proposed)

/// Relocate the pointer in `*this`, then hand it to the node-link callee.
///
/// Range check and relocate-and-add as in `rw_00634c40`. When the relocated
/// word is zero the function returns at once; otherwise the node-link
/// callee runs with the relocated pointer in ecx and the re-read allocator
/// on the stack.
///
/// Returns the last callee answer reached: the slot-0 value on the
/// early-zero paths (untouched eax), -1 when the range check rejects, the
/// relocate answer when the relocated word is zero, else the node-link
/// answer.
///
/// Original: 0x006355b0 (thiscall, no stack words, three calls).
lf_checker_rt::export!(thiscall, rw_006355b0(this: u32) -> u32 {
    unsafe {
        const ALLOC: u32 = 0x04;
        const RANGE_CHECK: u32 = 1;
        const RELOCATE: u32 = 2;
        const NODE_LINK: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let slot0 = lf_checker_rt::tls_slot(0);
        let alloc = rd32(slot0.wrapping_add(ALLOC));
        if alloc == 0 || rd32(this) == 0 {
            wr32(this, 0);
            return slot0;
        }
        let rc: u32 = lf_checker_rt::callee_thiscall!(RANGE_CHECK, u32, rd32(alloc), this);
        if rc == 0xFFFF_FFFF {
            wr32(this, 0);
            return rc;
        }
        let delta: u32 = lf_checker_rt::callee_thiscall!(RELOCATE, u32, alloc, rd32(this));
        wr32(this, rd32(this).wrapping_add(delta));
        if rd32(this) == 0 {
            return delta;
        }
        let slot0b = lf_checker_rt::tls_slot(0);
        let allocb = rd32(slot0b.wrapping_add(ALLOC));
        lf_checker_rt::callee_thiscall!(NODE_LINK, u32, rd32(this), allocb)
    }
});
