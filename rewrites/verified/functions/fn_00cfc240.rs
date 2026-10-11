// original: 0x00CFC240 CTaskComplexCombatChargeSubtask::CTaskComplexCombatChargeSubtask

/// Initialize a combat charge subtask after its base constructor runs. `this` is the task object; `kind` is the forwarded stack word. The constructor stores the relocated vtable pointer at offset zero and returns `this`.
lf_checker_rt::export!(thiscall, rw_cfc240(this: u32, kind: u32) -> u32 {
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
                const VTABLE_VA: u32 = 0x00EDFF74;
                wr32(this, lf_checker_rt::relocated(VTABLE_VA));
                this
        }
});
