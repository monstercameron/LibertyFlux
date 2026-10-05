// original: 0x0069AC60 rage::crAnimChannelRawVector3::storage_size

/// Storage size of a raw Vector3 channel: one 16-byte slot per element.
///
/// `this` is the channel object; the element count is the unsigned 16-bit
/// word at `+0xC`. The size is `(count + 1) << 4`: one extra slot past the
/// last element (the sampler reads element `index + 1`). Call-free; returns
/// the size in `eax`.
///
/// Original: 0x0069AC60 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_0069AC60(this: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0x0C;
        const SLOT_SHIFT: u32 = 4;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        let count = rd16(this + COUNT_OFF);
        count.wrapping_add(1).wrapping_shl(SLOT_SHIFT)
    }
});
