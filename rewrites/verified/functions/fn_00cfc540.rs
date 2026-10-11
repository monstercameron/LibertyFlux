// original: 0x00CFC540 CTaskComplexCombatSeekCoverSubtask::CTaskComplexCombatSeekCoverSubtask

/// Initialize a seek-cover combat subtask. `kind` goes to the base constructor, `source` supplies three copied words, and the two flag words contribute their low bits to the low two bits of the byte at 0x40 while retaining its other bits except the low pair. The byte at 0x60 is cleared.
lf_checker_rt::export!(thiscall, rw_cfc540(this: u32, kind: u32, source: u32, first_flag: u32, second_flag: u32) -> u32 {
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
                const VTABLE_VA: u32 = 0x00EDFE94;
                let old_flags = rd8(this.wrapping_add(0x40));
                let flags = (old_flags & 0xfc) | ((second_flag as u8 & 1) << 1) | (first_flag as u8 & 1);
                wr32(this, lf_checker_rt::relocated(VTABLE_VA));
                wr8(this.wrapping_add(0x40), flags);
                for lane in 0..3u32 { wr32(this.wrapping_add(0x50 + lane * 4), rd32(source.wrapping_add(lane * 4))); }
                wr8(this.wrapping_add(0x60), 0);
                this
        }
});
