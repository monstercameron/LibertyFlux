// original: 0x00a94a20 stream_entry_is_active (proposed)

/// Whether a stream entry counts as active (1) or empty (0).
///
/// Active when the size word at `+0x08` has any bit above the low two set
/// or bit 11 of the flag word at `+0x0e` is set. Only the low result byte
/// is set; the upper bytes of `eax` keep the caller's value, so the
/// comparison covers `al` only. Pure leaf.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00a94a20(this: u32) -> u8 {
    unsafe {
        const ENT_SIZE: u32 = 0x08;
        const ENT_FLAGS: u32 = 0x0e;
        const SIZE_MASK: u32 = 0xffff_fffc;
        const PRESENT_BIT: u32 = 11;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        if rd32(this.wrapping_add(ENT_SIZE)) & SIZE_MASK != 0
            || (rd16(this.wrapping_add(ENT_FLAGS)) >> PRESENT_BIT) & 1 != 0
        {
            1
        } else {
            0
        }
    }
});
