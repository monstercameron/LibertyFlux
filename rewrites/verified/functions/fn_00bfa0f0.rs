// original: 0x00bfa0f0 task_solve_copy3

/// Solve through two scratch-buffer calls (frame addresses skipped; buffers
/// never read back), store the solver answer at +0x10, then copy three
/// dwords from the source at +0x30 into +0x14.
///
/// Original: 0x00bfa0f0 (thiscall, 1 stack words).
lf_checker_rt::export!(thiscall, rw_00bfa0f0(this: u32, a0: u32) -> u32 {
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
        let mut scratch = [0u32; 8];
        let p = scratch.as_mut_ptr() as u32;
        lf_checker_rt::callee_thiscall!(1, u32, a0, p);
        let ans: u32 = lf_checker_rt::callee_cdecl!(2, u32, p);
        w32u(this.wrapping_add(0x10), ans);
        w32u(this.wrapping_add(0x14), r32u(a0.wrapping_add(0x30)));
        w32u(this.wrapping_add(0x18), r32u(a0.wrapping_add(0x34)));
        w32u(this.wrapping_add(0x1c), r32u(a0.wrapping_add(0x38)));
        0
    }
});
