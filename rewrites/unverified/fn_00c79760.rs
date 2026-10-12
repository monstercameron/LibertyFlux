// original: 0x00C79760 CAnimatedBuilding::CAnimatedBuilding

/// Construct an animated building in place. Runs the base dynamic-entity constructor, sets flag bits 0 and 2 at +0x24, keeps bits 7..30 of the word at +0x28 while clearing bits 0..6 except setting bit 6, installs the relocated vtable and returns `this`. Original: thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_c79760(this: u32) -> u32 {
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
                const FLAGS: u32 = 0x24;
                const MODE: u32 = 0x28;
                const VTABLE_VA: u32 = 0x00ED46A4;
                wr32(this.wrapping_add(FLAGS), rd32(this.wrapping_add(FLAGS)) | 5);
                wr32(this.wrapping_add(MODE), (rd32(this.wrapping_add(MODE)) & 0xffff_fc7f) | 0x40);
                wr32(this, lf_checker_rt::relocated(VTABLE_VA));
                this
        }
});
