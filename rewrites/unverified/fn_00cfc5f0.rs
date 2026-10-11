// original: 0x00CFC5F0 CTaskComplexGoIntoCover::CTaskComplexGoIntoCover

/// Initialize a go-into-cover task from an optional owner pointer, a three-word source vector, an f32 bit pattern and a byte flag. It forwards no arguments to the base constructor, copies those values into the task, and calls a direct helper for the owner field only when the owner pointer is nonzero.
lf_checker_rt::export!(thiscall, rw_cfc5f0(this: u32, owner: u32, source: u32, value_bits: u32, flag_word: u32) -> u32 {
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
                const VTABLE_VA: u32 = 0x00EE037C;
                wr32(this, lf_checker_rt::relocated(VTABLE_VA));
                for lane in 0..3u32 { wr32(this.wrapping_add(0x20 + lane * 4), rd32(source.wrapping_add(lane * 4))); }
                wr32(this.wrapping_add(0x30), owner);
                wr32(this.wrapping_add(0x34), value_bits);
                wr8(this.wrapping_add(0x38), flag_word as u8);
                if owner != 0 {
                    let _ = lf_checker_rt::callee_thiscall!(2, u32, owner, this.wrapping_add(0x30));
                }
                this
        }
});
