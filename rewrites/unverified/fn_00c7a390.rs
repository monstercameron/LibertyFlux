// original: 0x00C7A390 CTaskComplexScenario::CTaskComplexScenario

/// Construct the scenario-task base. Runs the complex-task base constructor, stores a0 at +0x14 and a2 at +0x1c, installs the relocated vtable, clears +0x18 and +0x21, stores (a2 != 0) at +0x20, calls the child-attach helper with (this, a1), then re-reads +0x1c and only when it is nonzero attaches the node there. Returns `this`. Original: thiscall, three stack words.
lf_checker_rt::export!(thiscall, rw_c7a390(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
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
                const VTABLE_VA: u32 = 0x00ED58C4;
                const CHILD: u32 = 0x1c;
                wr32(this.wrapping_add(0x14), a0);
                wr32(this.wrapping_add(CHILD), a2);
                wr32(this, lf_checker_rt::relocated(VTABLE_VA));
                wr32(this.wrapping_add(0x18), 0);
                wr8(this.wrapping_add(0x20), u8::from(a2 != 0));
                wr8(this.wrapping_add(0x21), 0);
                let _ = lf_checker_rt::callee_thiscall!(2, u32, this, a1);
                let child = rd32(this.wrapping_add(CHILD));
                if child != 0 {
                    let _ = lf_checker_rt::callee_thiscall!(3, u32, child, this.wrapping_add(CHILD));
                }
                this
        }
});
