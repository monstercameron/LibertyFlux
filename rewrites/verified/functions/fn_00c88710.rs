// original: 0x00c88710 audio_buffer_detach (proposed)
///
/// If the attached flag at `this+0x24` is set and the buffer pointer at
/// `this+4` is non-null, releases the buffer through callee 1 (cdecl,
/// one argument) and then clears `this+4`, the flag at `this+0x24` and
/// `this+8`. No return value. Thiscall, no stack arguments.

lf_checker_rt::export!(thiscall, rw_00c88710(this: u32) -> u32 {
    unsafe {
    #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
    #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        if ((this.wrapping_add(0x24)) as *const u8).read() != 0 {
            let p = rd32(this.wrapping_add(4));
            if p != 0 {
                lf_checker_rt::callee_cdecl!(1, u32, p);
                wr32(this.wrapping_add(4), 0);
                ((this.wrapping_add(0x24)) as *mut u8).write(0);
                wr32(this.wrapping_add(8), 0);
            }
        }
        0
    }
});
