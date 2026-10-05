// original: 0x008DCAD0 command_buffer_space_check (proposed)

/// Command-buffer space check (proposed name).
///
/// Checks the command buffer has room for `need` more bytes. The used
/// count at +0x1c plus `need` (wrapping) below 2 MiB means room and the
/// sum is returned. Otherwise the buffer wraps: used is reset to 0, the
/// wrapped flag global is cleared and the slot-free marker byte at
/// `this + slot_stride + 8` is set, returning the old stride.
///
/// Original: 0x008DCAD0 (thiscall: `this` in ECX, `need`, unused).
lf_checker_rt::export!(thiscall, rw_008dcad0(this: u32, need: u32, _unused: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        const USED: u32 = 0x1c;
        const STRIDE: u32 = 0x14;
        const CAPACITY: u32 = 0x200000;
        const WRAPPED_FLAG: u32 = 0x010327A8;
        let end = rd32(this + USED).wrapping_add(need);
        if end < CAPACITY {
            return end;
        }
        let stride = rd32(this + STRIDE);
        (this.wrapping_add(USED) as *mut u32).write_unaligned(0);
        lf_checker_rt::global::<u8>(WRAPPED_FLAG).write(0);
        (this.wrapping_add(stride).wrapping_add(8) as *mut u8).write(1);
        stride
    }
});
