// original: 0x00CFC460 CTaskComplexCombatPersueInCarSubtask::CTaskComplexCombatPersueInCarSubtask

/// Initialize a pursue-in-car combat subtask from an entity-like pointer. The base constructor receives that pointer. The result flag at 0x44 is one only when the pointed object has a nonzero word at 0xb30 and bit 2 set in its byte at 0x26c; other fields are reset before returning `this`.
lf_checker_rt::export!(thiscall, rw_cfc460(this: u32, entity: u32) -> u32 {
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

        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, entity);
                const VTABLE_VA: u32 = 0x00EE0054;
                wr32(this, lf_checker_rt::relocated(VTABLE_VA));
                wr32(this.wrapping_add(0x40), 0);
                wr8(this.wrapping_add(0x44), u8::from(rd32(entity.wrapping_add(0xb30)) != 0 && (rd8(entity.wrapping_add(0x26c)) & 4) != 0));
                wr32(this.wrapping_add(0x48), 0);
                wr16(this.wrapping_add(0x4c), 0);
                this
        }
});
