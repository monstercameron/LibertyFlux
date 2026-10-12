// original: 0x00C7A2C0 CTaskComplexMobileMakeCall::CTaskComplexMobileMakeCall

/// Construct a mobile-make-call task. Runs the complex-task base constructor, stores a1 at +0x14 and a2 at +0x40, installs the relocated vtable, clears +0x18 and the word at +0x48, stores 1 at +0x1c, and copies the NUL-terminated byte string at a0 to +0x20. Returns `this`. Original: thiscall, three stack words.
lf_checker_rt::export!(thiscall, rw_c7a2c0(this: u32, src: u32, a1: u32, a2: u32) -> u32 {
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
                const VTABLE_VA: u32 = 0x00ED5FDC;
                const NAME: u32 = 0x20;
                wr32(this.wrapping_add(0x14), a1);
                wr32(this.wrapping_add(0x40), a2);
                wr32(this, lf_checker_rt::relocated(VTABLE_VA));
                wr32(this.wrapping_add(0x18), 0);
                wr32(this.wrapping_add(0x1c), 1);
                wr16(this.wrapping_add(0x48), 0);
                let mut off = 0u32;
                loop {
                    let b = rd8(src.wrapping_add(off));
                    wr8(this.wrapping_add(NAME).wrapping_add(off), b);
                    if b == 0 { break; }
                    off += 1;
                }
                this
        }
});
