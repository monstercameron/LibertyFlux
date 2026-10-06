// original: 0x006353b0 reloc_ptr_cdecl (proposed)

/// Relocate the pointer in `*tgt` through the thread's allocator, in place.
///
/// The allocator is reached through thread-local slot 0 (`S0`), whose
/// word at `+ALLOC` points at it. When the allocator or the target word is
/// null the target is zeroed. Otherwise the range-check callee validates
/// the target address against the allocator's first word; a -1 answer
/// zeroes the target. On success the relocate callee maps the old pointer
/// to a delta that is added onto the target, and the notify callee
/// observes the new pointer with the re-read allocator.
///
/// Returns the notify callee's answer on success, -1 when the range check
/// rejects, and the slot-0 value when the allocator or target was null
/// (the original's untouched eax on those paths).
///
/// Original: 0x006353b0 (cdecl, one stack word, three calls).
lf_checker_rt::export!(cdecl, rw_006353b0(tgt: u32) -> u32 {
    unsafe {
        const ALLOC: u32 = 0x04;
        const RANGE_CHECK: u32 = 1;
        const RELOCATE: u32 = 2;
        const NOTIFY: u32 = 3;

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
        if alloc == 0 || rd32(tgt) == 0 {
            wr32(tgt, 0);
            return slot0;
        }
        let rc: u32 = lf_checker_rt::callee_thiscall!(RANGE_CHECK, u32, rd32(alloc), tgt);
        if rc == 0xFFFF_FFFF {
            wr32(tgt, 0);
            return rc;
        }
        let delta: u32 = lf_checker_rt::callee_thiscall!(RELOCATE, u32, alloc, rd32(tgt));
        wr32(tgt, rd32(tgt).wrapping_add(delta));
        let slot0b = lf_checker_rt::tls_slot(0);
        let allocb = rd32(slot0b.wrapping_add(ALLOC));
        lf_checker_rt::callee_fastcall!(NOTIFY, u32, allocb, rd32(tgt))
    }
});
