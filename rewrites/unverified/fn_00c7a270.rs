// original: 0x00C7A270 CTaskComplexMobileChatScenario::CTaskComplexMobileChatScenario

/// Construct a mobile-chat-scenario task. Forwards (a0, 0, 0) to the scenario base constructor, installs the relocated vtable, clears +0x24, +0x30..0x32 and +0x34, and stores 1 at +0x28, or 5 when a0 equals 43. Returns `this`. Original: thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_c7a270(this: u32, a0: u32) -> u32 {
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

        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, a0, 0, 0);
                const VTABLE_VA: u32 = 0x00ED5F74;
                const MOBILE_MODE: u32 = 43;
                wr32(this, lf_checker_rt::relocated(VTABLE_VA));
                wr32(this.wrapping_add(0x24), 0);
                wr32(this.wrapping_add(0x28), 1);
                wr16(this.wrapping_add(0x30), 0);
                wr8(this.wrapping_add(0x32), 0);
                wr32(this.wrapping_add(0x34), 0);
                if a0 == MOBILE_MODE {
                    wr32(this.wrapping_add(0x28), 5);
                }
                this
        }
});
