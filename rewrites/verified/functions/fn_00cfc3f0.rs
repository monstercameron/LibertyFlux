// original: 0x00CFC3F0 CTaskComplexCombatInvestigateSubtask::CTaskComplexCombatInvestigateSubtask

/// Initialize an investigate subtask after forwarding `kind` to the combat base constructor. Preserve only the high six bits of the flag byte at 0x48, set the vtable, write the observed sentinel and clear the listed state fields. Returns `this`.
lf_checker_rt::export!(thiscall, rw_cfc3f0(this: u32, kind: u32) -> u32 {
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
                const VTABLE_VA: u32 = 0x00EDFFE4;
                let flags = rd8(this.wrapping_add(0x48)) & 0xfc;
                wr8(this.wrapping_add(0x48), flags);
                wr32(this, lf_checker_rt::relocated(VTABLE_VA));
                wr32(this.wrapping_add(0x40), u32::MAX);
                for off in [0x44u32, 0x4c, 0x50, 0x54, 0x58, 0x60, 0x70, 0x74, 0x78] {
                    wr32(this.wrapping_add(off), 0);
                }
                this
        }
});
