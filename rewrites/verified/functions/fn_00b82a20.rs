// original: 0x00B82A20 flag_router_std
/// Route a flag event unless the object already carries it.
///
/// With kind 0 the event is skipped when flag `0x20000` or `0x40000` is
/// set at `F0`; otherwise it is skipped when any of the top three bits
/// at `F1` is set. Otherwise callee 1 is invoked with the sub byte, the
/// object, the sign-extended kind and 1.
///
/// Original: 0x00B82A20 (stdcall, three stack arguments, no return value).
lf_checker_rt::export!(stdcall, rw_00B82A20(obj: u32, kind: u32, sub: u32) -> u32 {
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
        const F0: u32 = 0x260;
        const F1: u32 = 0x210;
        let kb = (kind & 0xFF) as u8;
        if kb == 0 {
            let f = rd32(obj + F0);
            if f & 0x20000 != 0 {
                return 0;
            }
            if f & 0x40000 != 0 {
                return 0;
            }
        } else if rd32(obj + F1) & 0xE0000000 != 0 {
            return 0;
        }
        let sx = (kb as i8) as i32 as u32;
        lf_checker_rt::callee_stdcall!(1, u32, sub & 0xFF, obj, sx, 1);
        0
    }
});
