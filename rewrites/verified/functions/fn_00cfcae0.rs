// original: 0x00CFCAE0 task_complex_ctor_with_two_words_00CFCAE0

/// Initialize a task object with two raw stack words. A different task base constructor runs, then a component constructor runs for the subobject at 0x18. The function installs the observed vtable, clears offset 0x14, copies the two words to offsets 0x24 and 0x28, and returns `this`.
lf_checker_rt::export!(thiscall, rw_cfcae0(this: u32, first_word: u32, second_word: u32) -> u32 {
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
                const VTABLE_VA: u32 = 0x00EE058C;
                wr32(this, lf_checker_rt::relocated(VTABLE_VA));
                wr32(this.wrapping_add(0x14), 0);
                let _ = lf_checker_rt::callee_thiscall!(2, u32, this.wrapping_add(0x18));
                wr32(this.wrapping_add(0x24), first_word);
                wr32(this.wrapping_add(0x28), second_word);
                this
        }
});
