// original: 0x00C76D60 rage::pgBaseWrapper<rage::datOwner<rage::phBound>>::vf0

/// Destroy a physics-bound wrapper. Runs the dictionary destructor, frees the object only when bit 0 of the flags word is set, and returns `this`. Original: thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_c76d60(this: u32, flags: u32) -> u32 {
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

        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
                if flags & 1 != 0 {
                    let _ = lf_checker_rt::callee_cdecl!(2, u32, this);
                }
                this
        }
});
