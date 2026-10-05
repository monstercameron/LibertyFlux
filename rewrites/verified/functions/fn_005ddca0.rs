// original: 0x005ddca0 TaskFE0E2C_dtor

/// Destroy a vtable-FE0E2C task, releasing four members in order.
///
/// Stamps `VTABLE`, releases each non-null member at `+0x20`, `+0x14`,
/// `+0x18`, `+0x44` in that order (clearing each slot), then
/// tail-calls the base destructor and returns its result.
///
/// Original: 0x005ddca0 (thiscall).
lf_checker_rt::export!(thiscall, rw_005ddca0(this: u32) -> u32 {
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
        const VTABLE: u32 = 0xfe0e2c;
        const RELEASE: u32 = 1;
        const TAIL_DTOR: u32 = 2;
        wr32(this, lf_checker_rt::relocated(VTABLE));
        for slot in [0x20u32, 0x14, 0x18, 0x44] {
            let c = rd32(this + slot);
            if c != 0 {
                lf_checker_rt::callee_thiscall!(RELEASE, u32, c, this + slot);
                wr32(this + slot, 0);
            }
        }
        // The original tail-jumps to the base destructor; its result is ours.
        lf_checker_rt::callee_thiscall!(TAIL_DTOR, u32, this)
    }
});
