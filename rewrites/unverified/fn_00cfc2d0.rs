// original: 0x00CFC2D0 CTaskComplexCombatFireSubtask::CTaskComplexCombatFireSubtask

/// Initialize a fire subtask from a vector pointer and four additional words. The fifth word is an f32 bit pattern compared with zero; the byte at 0x44 records ordered greater-than, so NaN yields false. The source vector is copied to 0x50..0x58, and the low bits of two flags combine with the existing high nibble at 0x60. The constructor writes its default scalar fields and returns `this`.
lf_checker_rt::export!(thiscall, rw_cfc2d0(this: u32, kind: u32, source: u32, primary_flags: u32, secondary_flags: u32, value_bits: u32) -> u32 {
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
                const VTABLE_VA: u32 = 0x00EDFD44;
                const STORED_VALUE: u32 = 0x40;
                const VALUE_IS_POSITIVE: u32 = 0x44;
                const SOURCE_VECTOR: u32 = 0x50;
                const FLAG_BYTE: u32 = 0x60;
                let value = f32::from_bits(value_bits);
                let is_positive = u8::from(value > 0.0);
                let old_flags = rd8(this.wrapping_add(FLAG_BYTE));
                let combined_flags = (old_flags & 0xf0) | ((secondary_flags as u8 & 1) << 1) | (primary_flags as u8 & 1);
                wr8(this.wrapping_add(VALUE_IS_POSITIVE), is_positive);
                wr32(this, lf_checker_rt::relocated(VTABLE_VA));
                wr32(this.wrapping_add(STORED_VALUE), value_bits);
                wr32(this.wrapping_add(0x48), u32::MAX);
                for lane in 0..3u32 {
                    wr32(this.wrapping_add(SOURCE_VECTOR + lane * 4), rd32(source.wrapping_add(lane * 4)));
                }
                wr8(this.wrapping_add(FLAG_BYTE), combined_flags);
                wr32(this.wrapping_add(0x64), 0);
                wr32(this.wrapping_add(0x68), 0);
                wr32(this.wrapping_add(0x6c), 0x4000_0000);
                wr8(this.wrapping_add(0x70), 0);
                this
        }
});
