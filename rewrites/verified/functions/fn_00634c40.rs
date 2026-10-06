// original: 0x00634c40 reloc_ptr_store_a (proposed)

/// Relocate the pointer in `*this` through the thread's allocator.
///
/// Same three-callee shape as `rw_006353b0` (range check, relocate-and-add,
/// notify with the re-read allocator), as a thiscall returning `this` on
/// every path. When the allocator or the target word is null, or the range
/// check answers -1, the target is zeroed first.
///
/// Returns `this` always.
///
/// Original: 0x00634c40 (thiscall, no stack words, three calls).
lf_checker_rt::export!(thiscall, rw_00634c40(this: u32) -> u32 {
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
        if alloc == 0 || rd32(this) == 0 {
            wr32(this, 0);
            return this;
        }
        let rc: u32 = lf_checker_rt::callee_thiscall!(RANGE_CHECK, u32, rd32(alloc), this);
        if rc == 0xFFFF_FFFF {
            wr32(this, 0);
            return this;
        }
        let delta: u32 = lf_checker_rt::callee_thiscall!(RELOCATE, u32, alloc, rd32(this));
        wr32(this, rd32(this).wrapping_add(delta));
        let slot0b = lf_checker_rt::tls_slot(0);
        let allocb = rd32(slot0b.wrapping_add(ALLOC));
        let _n: u32 = lf_checker_rt::callee_fastcall!(NOTIFY, u32, allocb, rd32(this));
        this
    }
});
