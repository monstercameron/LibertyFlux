// original: 0x00CFC260 CTaskComplexCombatClosestTargetInArea::CTaskComplexCombatClosestTargetInArea

/// Initialize a combat task that copies a three-word source vector and two scalar bit patterns. The four stack words are a source pointer, first scalar bits, mode word, and second scalar bits. The base constructor runs first; the original thiscall removes all four stack words.
lf_checker_rt::export!(thiscall, rw_cfc260(this: u32, source: u32, first_scalar_bits: u32, mode_word: u32, second_scalar_bits: u32) -> u32 {
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
                const VTABLE_VA: u32 = 0x00EE03D4;
                const VECTOR_SOURCE_X: u32 = 0;
                const VECTOR_TARGET_X: u32 = 0x20;
                const SCALAR_ONE: u32 = 0x30;
                const MODE_BYTE: u32 = 0x34;
                const SCALAR_TWO: u32 = 0x38;
                wr32(this, lf_checker_rt::relocated(VTABLE_VA));
                for lane in 0..3u32 {
                    let word = rd32(source.wrapping_add(VECTOR_SOURCE_X + lane * 4));
                    wr32(this.wrapping_add(VECTOR_TARGET_X + lane * 4), word);
                }
                wr32(this.wrapping_add(SCALAR_ONE), first_scalar_bits);
                wr8(this.wrapping_add(MODE_BYTE), mode_word as u8);
                wr32(this.wrapping_add(SCALAR_TWO), second_scalar_bits);
                this
        }
});
