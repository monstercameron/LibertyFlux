// original: 0x00bf90a0 task_ctor_3c_copy3

/// Initialise a task of kind 0x3c through the 0x3e sub-initialiser, then
/// copy three dwords from the pointed-to block.
///
/// Original: 0x00bf90a0 (thiscall, 4 stack words).
lf_checker_rt::export!(thiscall, rw_00bf90a0(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn r8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn w8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn r32u(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn w32u(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        lf_checker_rt::callee_thiscall!(1, u32, this, a0, a1, a2);
        w8(this, 0x3c);
        w32u(this + 0x1c, r32u(a3));
        w32u(this + 0x20, r32u(a3.wrapping_add(4)));
        w32u(this + 0x24, r32u(a3.wrapping_add(8)));
        w8(this + 0x14, 0);
        w8(this + 2, 5);
        0
    }
});
