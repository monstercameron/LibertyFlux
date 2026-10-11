// original: 0x00CFC2B0 CTaskComplexCombatExecutePedSubtask::CTaskComplexCombatExecutePedSubtask

/// Initialize an execute-ped combat subtask. `kind` is forwarded to the combat-subtask base constructor, then the derived vtable and a zero field at offset 0x40 are stored. Returns `this`.
lf_checker_rt::export!(thiscall, rw_cfc2b0(this: u32, kind: u32) -> u32 {
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
                const VTABLE_VA: u32 = 0x00EE01FC;
                wr32(this, lf_checker_rt::relocated(VTABLE_VA));
                wr32(this.wrapping_add(0x40), 0);
                this
        }
});
