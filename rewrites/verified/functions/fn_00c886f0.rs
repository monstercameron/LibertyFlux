// original: 0x00c886f0 audio_buffer_release (proposed)
///
/// If the attached flag at `this+0x24` is set and the buffer pointer at
/// `this+4` is non-null, releases the buffer through callee 1 (cdecl,
/// one argument). No memory is written and there is no return value.
/// Thiscall, no stack arguments.

lf_checker_rt::export!(thiscall, rw_00c886f0(this: u32) -> u32 {
    unsafe {
    #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        if ((this.wrapping_add(0x24)) as *const u8).read() != 0 {
            let p = rd32(this.wrapping_add(4));
            if p != 0 {
                lf_checker_rt::callee_cdecl!(1, u32, p);
            }
        }
        0
    }
});
