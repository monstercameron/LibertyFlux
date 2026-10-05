// original: 0x0069B2D0 rage::crAnimChannelRawBool::storage_size

/// Storage size of a raw boolean channel: element count plus header.
///
/// `this` is the channel object; the element count is the unsigned 16-bit
/// word at `+0xC` and the header is `0x10` bytes, so the size is
/// `count + 0x10`. Call-free; returns the size in `eax`.
///
/// Original: 0x0069B2D0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_0069B2D0(this: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0x0C;
        const HEADER_SIZE: u32 = 0x10;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        let count = rd16(this + COUNT_OFF);
        count.wrapping_add(HEADER_SIZE)
    }
});
