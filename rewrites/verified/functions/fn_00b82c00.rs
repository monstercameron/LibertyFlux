// original: 0x00B82C00 slot_array_init
/// Initialise the 140 flag slots, then mark the table enabled.
///
/// Invokes the slot constructor (callee 1) on each `STRIDE`-byte slot in
/// turn, then sets the enable byte `FLAG`.
///
/// Original: 0x00B82C00 (thiscall, no stack arguments, no return value).
lf_checker_rt::export!(thiscall, rw_00B82C00(this: u32) -> u32 {
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
        const SLOTS: u32 = 0x8C;
        const STRIDE: u32 = 0x2C;
        const FLAG: u32 = 0x1810;
        let mut i = 0u32;
        while i < SLOTS {
            lf_checker_rt::callee_thiscall!(1, u32, this.wrapping_add(i.wrapping_mul(STRIDE)));
            i += 1;
        }
        wr8(this + FLAG, 1);
        0
    }
});
