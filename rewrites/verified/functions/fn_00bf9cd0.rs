// original: 0x00bf9cd0 task_xform_copy3

/// Run the normaliser and the transformer over scratch buffers (their frame
/// addresses are skipped in the call comparison; the buffers are never read
/// back), then copy three dwords from +0x1c into the target at +0x30.
///
/// Original: 0x00bf9cd0 (thiscall, 1 stack words).
lf_checker_rt::export!(thiscall, rw_00bf9cd0(this: u32, a0: u32) -> u32 {
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
        lf_checker_rt::callee_cdecl!(1, u32, p, r32u(this.wrapping_add(0x18)));
        lf_checker_rt::callee_thiscall!(2, u32, a0, p);
        w32u(a0.wrapping_add(0x30), r32u(this.wrapping_add(0x1c)));
        w32u(a0.wrapping_add(0x34), r32u(this.wrapping_add(0x20)));
        w32u(a0.wrapping_add(0x38), r32u(this.wrapping_add(0x24)));
        0
    }
});
