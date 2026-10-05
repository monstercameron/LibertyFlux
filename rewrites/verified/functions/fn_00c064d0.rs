// original: 0x00c064d0 stream_set_name_if_present
/// Copy a name into this object when a non-empty one is given.
///
/// A null `name`, or one whose first byte is zero, leaves the object
/// untouched. Otherwise copies through the string helper (cdecl/3:
/// destination `this`, source `name`, length 0x20) and clears the flag byte
/// at `this`+0x1f. Returns the helper's answer (compared as EAX). Thiscall:
/// `this` in ECX, one stack word, callee cleans 4. The copy's tail past the
/// first 16 bytes is outside the checker's per-callee out-word cap and is
/// not observed (see `narrowed`).
lf_checker_rt::export!(thiscall, rw_00c064d0(this: u32, name: u32) -> u32 {
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
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        const COPY: u32 = 1;
        const NAME_LEN: u32 = 0x20;
        const FLAG_OFF: u32 = 0x1f;
        if name == 0 || rd8(name) == 0 {
            return 0;
        }
        let ans: u32 = lf_checker_rt::callee_cdecl!(COPY, u32, this, name, NAME_LEN);
        wr8(this + FLAG_OFF, 0);
        ans
    }
});
