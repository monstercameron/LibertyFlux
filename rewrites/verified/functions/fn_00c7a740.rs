// original: 0x00c7a740 ctask_wait_seat_free_dtor

/// Destroy a scenario task in two vtable stages.
/// Installs `VTABLE_OWN`, releases the member at `+0x18` when set (member
/// destructor callee on the slot, then nulled), installs `VTABLE_BASE`
/// and tail-calls the base destructor, returning its result.
/// Original: 0x00c7a740 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00c7a740(this: u32) -> u32 {
    unsafe {
        const VTABLE_OWN: u32 = 0x00ED5D34;
        const VTABLE_BASE: u32 = 0x00ED5CD4;
        const OFF_MEMBER: u32 = 0x18;
        const MEMBER_DTOR: u32 = 1;
        const BASE_DTOR: u32 = 2;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        wr32(this, lf_checker_rt::relocated(VTABLE_OWN));
        let held = rd32(this.wrapping_add(OFF_MEMBER));
        if held != 0 {
            lf_checker_rt::callee_thiscall!(MEMBER_DTOR, u32, held, this.wrapping_add(OFF_MEMBER));
            wr32(this.wrapping_add(OFF_MEMBER), 0);
        }
        wr32(this, lf_checker_rt::relocated(VTABLE_BASE));
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this)
    }
});
