// original: 0x00C7A200 CTaskComplexDrivingScenario::CTaskComplexDrivingScenario

/// Construct a driving-scenario task. Forwards (a0, 0, a1) to the scenario base constructor, installs the relocated vtable, stores a1 at +0x24, and only when a1 is nonzero attaches the node at +0x24 to owner a1. Returns `this`. Original: thiscall, two stack words.
lf_checker_rt::export!(thiscall, rw_c7a200(this: u32, a0: u32, a1: u32) -> u32 {
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

        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, a0, 0, a1);
                const VTABLE_VA: u32 = 0x00ED5B34;
                const NODE: u32 = 0x24;
                wr32(this, lf_checker_rt::relocated(VTABLE_VA));
                wr32(this.wrapping_add(NODE), a1);
                if a1 != 0 {
                    let _ = lf_checker_rt::callee_thiscall!(2, u32, a1, this.wrapping_add(NODE));
                }
                this
        }
});
