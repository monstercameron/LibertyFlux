// original: 0x00CFC640 CTaskComplexHelicopterStrafe::CTaskComplexHelicopterStrafe

/// Initialize a helicopter-strafe task from an optional object pointer. After its base constructor, the pointer is stored at 0x14 and a direct helper is called for nonnull values. If the pointed object has a nonnull word at 0x20, four words are copied from that target plus 0x30; otherwise they are copied from the object plus 0x10. Several fields are then cleared and `this` is returned.
lf_checker_rt::export!(thiscall, rw_cfc640(this: u32, object: u32) -> u32 {
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
                const VTABLE_VA: u32 = 0x00EE00C4;
                wr32(this.wrapping_add(0x14), object);
                wr32(this, lf_checker_rt::relocated(VTABLE_VA));
                wr32(this.wrapping_add(0x40), 0);
                for off in [0x50u32, 0x54, 0x58, 0x28, 0x24, 0x20] { wr32(this.wrapping_add(off), 0); }
                let object_slot = this.wrapping_add(0x14);
                let current_object = rd32(object_slot);
                if current_object != 0 {
                    let _ = lf_checker_rt::callee_thiscall!(2, u32, current_object, object_slot);
                    let current_object = rd32(object_slot);
                    if current_object != 0 {
                        let nested = rd32(current_object.wrapping_add(0x20));
                        let vector = if nested != 0 { nested.wrapping_add(0x30) } else { current_object.wrapping_add(0x10) };
                        for lane in 0..4u32 { wr32(this.wrapping_add(0x20 + lane * 4), rd32(vector.wrapping_add(lane * 4))); }
                    }
                }
                for off in [0x38u32, 0x34, 0x30] { wr32(this.wrapping_add(off), 0); }
                this
        }
});
