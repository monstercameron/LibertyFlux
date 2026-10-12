// original: 0x00C76C90 CMloModelInfo::vf2

/// Clear an interior model info. When the slot array at +0x70 is present, destroys it, frees it and nulls the field, then tail-calls the base model-info clear. Returns the base result. Original: thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_c76c90(this: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
        #[inline(always)]
        unsafe fn wr32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }
        #[inline(always)]
        unsafe fn rd16(address: u32) -> u16 { unsafe { (address as *const u16).read_unaligned() } }
        #[inline(always)]
        unsafe fn wr16(address: u32, value: u16) { unsafe { (address as *mut u16).write_unaligned(value) } }
        #[inline(always)]
        unsafe fn rd8(address: u32) -> u8 { unsafe { (address as *const u8).read() } }
        #[inline(always)]
        unsafe fn wr8(address: u32, value: u8) { unsafe { (address as *mut u8).write(value) } }

        let slots = rd32(this.wrapping_add(0x70));
                if slots != 0 {
                    let _ = lf_checker_rt::callee_thiscall!(1, u32, slots);
                    let _ = lf_checker_rt::callee_cdecl!(2, u32, slots);
                    wr32(this.wrapping_add(0x70), 0);
                }
                lf_checker_rt::callee_thiscall!(3, u32, this)
        }
});
