// original: 0x00C7A1B0 CTaskComplexChatScenario::CTaskComplexChatScenario

/// Construct a chat-scenario task. Forwards (a0, a3, a4) to the scenario base constructor, installs the relocated vtable, stores a5 at +0x28, a6 at +0x30, a1 at +0x24, zero at +0x2c and a2 at +0x34, then attaches the node at +0x24 to owner a1. Returns `this`. Original: thiscall, seven stack words.
lf_checker_rt::export!(thiscall, rw_c7a1b0(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32 {
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

        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, a0, a3, a4);
                const VTABLE_VA: u32 = 0x00ED5C04;
                const NODE: u32 = 0x24;
                wr32(this, lf_checker_rt::relocated(VTABLE_VA));
                wr32(this.wrapping_add(0x28), a5);
                wr32(this.wrapping_add(0x30), a6);
                wr32(this.wrapping_add(NODE), a1);
                wr32(this.wrapping_add(0x2c), 0);
                wr32(this.wrapping_add(0x34), a2);
                let _ = lf_checker_rt::callee_thiscall!(2, u32, a1, this.wrapping_add(NODE));
                this
        }
});
