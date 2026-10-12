// original: 0x00C7A3E0 CTaskComplexSeatedScenario::CTaskComplexSeatedScenario

/// Construct a seated-scenario task. Forwards (a0, a1, a2) to the scenario base constructor, installs the relocated vtable, stores (a2 != 0) at +0x24, the dword 0x10000 at +0x25 (so +0x27 reads 1), and clears +0x2c and +0x30. Returns `this`. Original: thiscall, three stack words.
lf_checker_rt::export!(thiscall, rw_c7a3e0(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
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
                const VTABLE_VA: u32 = 0x00ED59FC;
                wr32(this, lf_checker_rt::relocated(VTABLE_VA));
                wr8(this.wrapping_add(0x24), u8::from(a2 != 0));
                wr32(this.wrapping_add(0x25), 0x10000);
                wr32(this.wrapping_add(0x2c), 0);
                wr32(this.wrapping_add(0x30), 0);
                this
        }
});
