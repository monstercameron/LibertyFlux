// original: 0x00C7A360 CTaskComplexPoliceSniperScenario::CTaskComplexPoliceSniperScenario

/// Construct a police-sniper-scenario task. Forwards (a0, a1, a2) to the scenario base constructor, installs the relocated vtable and clears +0x24. Returns `this`. Original: thiscall, three stack words.
lf_checker_rt::export!(thiscall, rw_c7a360(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
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

        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, a0, a1, a2);
                const VTABLE_VA: u32 = 0x00ED5994;
                wr32(this, lf_checker_rt::relocated(VTABLE_VA));
                wr32(this.wrapping_add(0x24), 0);
                this
        }
});
