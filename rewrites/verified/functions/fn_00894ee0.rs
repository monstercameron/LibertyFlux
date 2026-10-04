// original: 0x00894ee0 init_named_object (proposed)

/// Initialise an object with scale, rates and a composed name.
///
/// `this` points at a large object. The function stores three unit scales
/// at `+0x538`/`+0x53c`/`+0x540`, an enable byte at `+0x346`, a rate of 15.0
/// at `+0x340`, and zeroes at `+0x338`, `+0x530`/`+0x534` (the last two are
/// then overwritten with `arg0`/`arg1`). It copies the `arg2` string into
/// the name field at `+0x1d50`, or the first global string when `arg2` is
/// null; copies that name into a frame buffer; appends the second global
/// string; and hands the composed path to the first callee (thiscall on
/// `this + 0x350`, taking the frame pointer). It then clears the byte at
/// `+0x347`, records `arg3` at `+0x31c`, runs the second callee (which
/// takes no arguments at all: its body never reads `ecx` or the stack,
/// so the stale `ecx` it inherits is dead) and returns the third callee's
/// answer (thiscall on `this`, no stack arguments).
///
/// The original builds the frame string with byte loops and a dword-plus-
/// bytes tail copy; the rewrite copies the same bytes including the
/// terminator. All string inputs in the proof are bounded so the composed
/// path fits the frame and the callee snapshot.
///
/// Original: 0x00894ee0 (thiscall, four stack words).
lf_checker_rt::export!(thiscall, rw_00894ee0(this: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32) -> u32 {
    unsafe {
        const ONE: u32 = 0x3F80_0000;
        const RATE15: u32 = 0x4170_0000;
        const NAME_OFF: u32 = 0x1D50;
        const SUB_OFF: u32 = 0x350;
        const GSTR1: u32 = 0x0103_03B0;
        const GSTR2: u32 = 0x0103_03B4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        /// Copy bytes including the terminator.
        unsafe fn strcpy1(dst: u32, src: u32) {
            unsafe {
                let mut d = dst;
                let mut s = src;
                loop {
                    let b = (s as *const u8).read();
                    (d as *mut u8).write(b);
                    s = s.wrapping_add(1);
                    d = d.wrapping_add(1);
                    if b == 0 {
                        break;
                    }
                }
            }
        }

        wr32(this + 0x538, ONE);
        wr32(this + 0x53c, ONE);
        wr32(this + 0x540, ONE);
        (this as *mut u8).wrapping_add(0x346).write(1);
        wr32(this + 0x340, RATE15);
        wr32(this + 0x338, 0);
        wr32(this + 0x530, 0);
        wr32(this + 0x534, 0);
        let src = if arg2 == 0 { rd32(lf_checker_rt::relocated(GSTR1)) } else { arg2 };
        strcpy1(this + NAME_OFF, src);
        wr32(this + 0x534, arg1);
        wr32(this + 0x530, arg0);
        let mut buf = [0u8; 256];
        strcpy1(buf.as_mut_ptr() as u32, this + NAME_OFF);
        let mut i = 0usize;
        while buf[i] != 0 {
            i += 1;
        }
        let mut s = rd32(lf_checker_rt::relocated(GSTR2));
        loop {
            let b = (s as *const u8).read();
            buf[i] = b;
            i += 1;
            s = s.wrapping_add(1);
            if b == 0 {
                break;
            }
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(
            1, u32, this.wrapping_add(SUB_OFF), buf.as_mut_ptr() as u32);
        (this as *mut u8).wrapping_add(0x347).write(0);
        wr32(this + 0x31c, arg3);
        let _: u32 = lf_checker_rt::callee_cdecl!(2, u32,);
        lf_checker_rt::callee_thiscall!(3, u32, this)
    }
});
