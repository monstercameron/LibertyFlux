// original: 0x00B81810 gta_thread_ctor
/// Construct a script thread object.
///
/// Runs the base constructor (callee 1), installs this class's function
/// table, runs the state reset (callee 2), clears the field at `TAIL` and
/// returns `this`.
///
/// Original: 0x00B81810 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00B81810(this: u32) -> u32 {
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
        const TAIL: u32 = 0xA8;
        lf_checker_rt::callee_thiscall!(1, u32, this);
        wr32(this, lf_checker_rt::relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(2, u32, this);
        wr32(this + TAIL, 0);
        this
    }
});
