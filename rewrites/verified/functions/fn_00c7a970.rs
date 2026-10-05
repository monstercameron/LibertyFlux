// original: 0x00c7a970 CTaskComplexWaitForDoorToBeOpen::vf0

/// Deleting destructor with an inline vtable reset.
/// Stores `VTABLE_BASE` over the object, runs the base destructor callee,
/// then frees through the heap singleton when bit 0 of `flags` is set.
/// Returns `this` either way.
/// Original: 0x00c7a970 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c7a970(this: u32, flags: u32) -> u32 {
    unsafe {
        const VTABLE_BASE: u32 = 0x00ED5CD4;
        const HEAP: u32 = 0x0167E2A0;
        const BASE_DTOR: u32 = 1;
        const FREE: u32 = 2;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        wr32(this, lf_checker_rt::relocated(VTABLE_BASE));
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this);
        if (flags & 1) != 0 {
            let heap = rd32(lf_checker_rt::relocated(HEAP));
            lf_checker_rt::callee_thiscall!(FREE, u32, heap, this);
        }
        this
    }
});
