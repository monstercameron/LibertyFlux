// original: 0x00B82A70 flag_router_this
/// Route a flag event for `obj` unless filtered out.
///
/// Needs the table enabled (`FLAG` set). Kind 0 consults the probe
/// (callee 1) after the `F0` mask check; other kinds only face the `F1`
/// top-bits check. Callee 2 searches with a frame out-slot; on success
/// callee 3 is invoked with the found byte, the object, the
/// sign-extended kind and 1.
///
/// Original: 0x00B82A70 (thiscall, two stack arguments, no return value).
lf_checker_rt::export!(thiscall, rw_00B82A70(this: u32, obj: u32, kind: u32) -> u32 {
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
        const FLAG: u32 = 0x1810;
        const F0: u32 = 0x260;
        const F1: u32 = 0x210;
        if rd8(this + FLAG) == 0 {
            return 0;
        }
        let kb = (kind & 0xFF) as u8;
        let mut flagword = this & 0xFFFF_FF00;
        if kb == 0 {
            if rd32(obj + F0) & 0x60000 != 0 {
                return 0;
            }
            flagword |= lf_checker_rt::callee_thiscall!(1, u32, obj) & 0xFF;
        } else if rd32(obj + F1) & 0xE0000000 != 0 {
            return 0;
        }
        let w = unsafe { (obj.wrapping_add(0x2E) as *const u16).read_unaligned() as i16 as i32 as u32 };
        let mut frame: u32 = 0;
        let ans = lf_checker_rt::callee_thiscall!(2, u32, this, &mut frame as *mut u32 as u32, kind, w, flagword);
        if ans & 0xFF == 0 {
            return 0;
        }
        let sx = (kb as i8) as i32 as u32;
        lf_checker_rt::callee_thiscall!(3, u32, this, frame & 0xFF, obj, sx, 1);
        0
    }
});
