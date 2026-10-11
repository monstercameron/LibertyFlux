// original: 0x00CFC360 CTaskComplexCombatFlankSubtask::CTaskComplexCombatFlankSubtask

/// Initialize a combat flank subtask. It forwards `kind` to its combat base constructor, installs its vtable, calls two vector or component constructors for subobjects at 0x40 and 0x4c, clears the listed state fields, and returns `this`.
lf_checker_rt::export!(thiscall, rw_cfc360(this: u32, kind: u32) -> u32 {
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

        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, kind);
                const VTABLE_VA: u32 = 0x00EDFE24;
                wr32(this, lf_checker_rt::relocated(VTABLE_VA));
                let _ = lf_checker_rt::callee_thiscall!(2, u32, this.wrapping_add(0x40));
                let _ = lf_checker_rt::callee_thiscall!(3, u32, this.wrapping_add(0x4c));
                wr32(this.wrapping_add(0x54), 0);
                wr8(this.wrapping_add(0x58), 0);
                for off in [0x60u32, 0x64, 0x68, 0x70, 0x74, 0x78, 0x80, 0x84, 0x88, 0xac] {
                    wr32(this.wrapping_add(off), 0);
                }
                this
        }
});
