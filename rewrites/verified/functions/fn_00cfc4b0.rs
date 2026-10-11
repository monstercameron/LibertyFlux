// original: 0x00CFC4B0 CTaskComplexCombatPullFromCarSubtask::CTaskComplexCombatPullFromCarSubtask

/// Initialize a pull-from-car combat subtask. `kind` is forwarded to the combat base constructor; the derived vtable and a zero word at offset 0x40 are written, and `this` is returned.
lf_checker_rt::export!(thiscall, rw_cfc4b0(this: u32, kind: u32) -> u32 {
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
                const VTABLE_VA: u32 = 0x00EE011C;
                wr32(this, lf_checker_rt::relocated(VTABLE_VA));
                wr32(this.wrapping_add(0x40), 0);
                this
        }
});
