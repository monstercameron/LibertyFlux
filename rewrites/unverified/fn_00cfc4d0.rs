// original: 0x00CFC4D0 CTaskComplexCombatRetreatSubtask::CTaskComplexCombatRetreatSubtask

/// Initialize a retreat subtask, forwarding `kind` to its combat base constructor. It writes three f32 defaults at offsets 0x40, 0x44 and 0x60, clears other scalar fields, invokes the component constructor at 0x6c, and returns `this`.
lf_checker_rt::export!(thiscall, rw_cfc4d0(this: u32, kind: u32) -> u32 {
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
                const VTABLE_VA: u32 = 0x00EDFF04;
                wr32(this, lf_checker_rt::relocated(VTABLE_VA));
                wr32(this.wrapping_add(0x40), 0x4100_0000);
                wr32(this.wrapping_add(0x44), 0x40a0_0000);
                for off in [0x50u32, 0x54, 0x58] { wr32(this.wrapping_add(off), 0); }
                wr32(this.wrapping_add(0x60), 0x40f0_0000);
                wr32(this.wrapping_add(0x64), 0);
                wr16(this.wrapping_add(0x68), 0x100);
                let _ = lf_checker_rt::callee_thiscall!(2, u32, this.wrapping_add(0x6c));
                wr32(this.wrapping_add(0x78), 0x3f80_0000);
                wr32(this.wrapping_add(0x7c), u32::MAX);
                this
        }
});
