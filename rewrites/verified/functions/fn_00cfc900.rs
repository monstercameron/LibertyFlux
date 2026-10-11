// original: 0x00CFC900 CTaskComplexSetAndGuardArea::CTaskComplexSetAndGuardArea

/// Initialize a set-and-guard-area task from eight stack words: two source pointers, three f32 bit patterns, one raw word, and two bytes. The first source contributes three words; the second contributes fields at 0, 4, 8, 0x10, 0x14, 0x18, 0x20, 0x24 and 0x28. Its low three bits replace the task flag bits while all higher bits are retained. The constructor returns `this`.
lf_checker_rt::export!(thiscall, rw_cfc900(this: u32, position: u32, value_a_bits: u32, value_b_bits: u32, mode_word: u32, value_c_bits: u32, source: u32, flag_a: u32, flag_b: u32) -> u32 {
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
                const VTABLE_VA: u32 = 0x00EE026C;
                wr32(this, lf_checker_rt::relocated(VTABLE_VA));
                for lane in 0..3u32 { wr32(this.wrapping_add(0x20 + lane * 4), rd32(position.wrapping_add(lane * 4))); }
                for lane in 0..3u32 { wr32(this.wrapping_add(0x30 + lane * 4), rd32(source.wrapping_add(lane * 4))); }
                for lane in 0..3u32 { wr32(this.wrapping_add(0x40 + lane * 4), rd32(source.wrapping_add(0x10 + lane * 4))); }
                for lane in 0..2u32 { wr32(this.wrapping_add(0x50 + lane * 4), rd32(source.wrapping_add(0x20 + lane * 4))); }
                let old_bits = rd32(this.wrapping_add(0x58));
                let source_bits = rd32(source.wrapping_add(0x28));
                wr32(this.wrapping_add(0x58), (old_bits & !7) | (source_bits & 7));
                wr32(this.wrapping_add(0x60), value_a_bits);
                wr32(this.wrapping_add(0x64), value_b_bits);
                wr32(this.wrapping_add(0x68), mode_word);
                wr32(this.wrapping_add(0x6c), value_c_bits);
                wr8(this.wrapping_add(0x70), flag_a as u8);
                wr8(this.wrapping_add(0x71), flag_b as u8);
                this
        }
});
