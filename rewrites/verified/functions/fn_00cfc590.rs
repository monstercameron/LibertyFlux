// original: 0x00CFC590 CTaskComplexCombatSubtask::CTaskComplexCombatSubtask

/// Initialize the combat-subtask base state from `context`. The base constructor runs first; a flag bit is cleared, the vtable and context pointer are stored, and several fields are reset. If `context` is nonzero, a direct helper is called with that value and the address of the context field. The function returns `this`.
lf_checker_rt::export!(thiscall, rw_cfc590(this: u32, context: u32) -> u32 {
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
                const VTABLE_VA: u32 = 0x00EDFCD4;
                let flags = rd8(this.wrapping_add(0x1d)) & 0xfe;
                wr8(this.wrapping_add(0x1d), flags);
                wr32(this.wrapping_add(0x18), context);
                wr32(this, lf_checker_rt::relocated(VTABLE_VA));
                wr8(this.wrapping_add(0x1c), 0);
                for off in [0x28u32, 0x30, 0x34, 0x38] { wr32(this.wrapping_add(off), 0); }
                if context != 0 {
                    let _ = lf_checker_rt::callee_thiscall!(2, u32, context, this.wrapping_add(0x18));
                }
                wr32(this.wrapping_add(0x14), 0);
                wr32(this.wrapping_add(0x20), 0xc8);
                this
        }
});
