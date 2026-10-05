// original: 0x00B81840 gta_thread_dtor_tail
/// Destroy a script thread object, tail-jumping to the base destructor.
///
/// Installs this class's function table, then tail-calls the base
/// destructor (callee 1), forwarding its result.
///
/// Original: 0x00B81840 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00B81840(this: u32) -> u32 {
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
        const VTABLE: u32 = 0x00EB3CCC;
        wr32(this, lf_checker_rt::relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(1, u32, this)
    }
});
