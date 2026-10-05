// original: 0x00a942b0 fiStreamingDevice::vf27

/// Whether the entry at `base + delta` is active: 1 yes, -1 no.
///
/// Adds the argument to the base key at `this+0x08`, resolves the entry
/// through the slot callee, and applies the empty test (size word at
/// `+0x08` clear above the low two bits and bit 11 of the flag word at
/// `+0x0e` clear). A null resolution or an empty entry gives -1.
///
/// Original: thiscall, one stack argument. One callee (thiscall, 1 arg).
lf_checker_rt::export!(thiscall, rw_00a942b0(this: u32, delta: u32) -> u32 {
    unsafe {
        const BASE_KEY: u32 = 0x08;
        const ENT_SIZE: u32 = 0x08;
        const ENT_FLAGS: u32 = 0x0e;
        const SIZE_MASK: u32 = 0xffff_fffc;
        const PRESENT_BIT: u32 = 11;
        const RESOLVE: u32 = 0;
        const ACTIVE: u32 = 1;
        const INACTIVE: u32 = 0xffff_ffff;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        let t = rd32(this.wrapping_add(BASE_KEY)).wrapping_add(delta);
        let ent: u32 = lf_checker_rt::callee_thiscall!(RESOLVE, u32, this, t);
        if ent == 0 {
            return INACTIVE;
        }
        if rd32(ent.wrapping_add(ENT_SIZE)) & SIZE_MASK != 0
            || (rd16(ent.wrapping_add(ENT_FLAGS)) >> PRESENT_BIT) & 1 != 0
        {
            ACTIVE
        } else {
            INACTIVE
        }
    }
});
