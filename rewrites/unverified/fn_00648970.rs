// original: 0x00648970 rage::ptxEmitRuleStd::vf18

/// Clear the rule's armed bit (bit 1 at `+0x104`) and, when `cb` is
/// non-null, notify through its virtual slot `+0x3c` with the rule
/// pointer. Stdcall, two stack arguments, no return value.
lf_checker_rt::export!(stdcall, rw_00648970(rule: u32, cb: u32) -> u32 {
    unsafe {
        const RULE_FLAGS: u32 = 0x104;
        const ARMED_MASK: u32 = 0xffff_fffd;
        const NOTIFY_SLOT: u32 = 0x3c;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        let flags = read_word(rule.wrapping_add(RULE_FLAGS));
        (rule.wrapping_add(RULE_FLAGS) as *mut u32)
            .write_unaligned(flags & ARMED_MASK);
        if cb == 0 {
            return 0;
        }
        let slot = read_word(read_word(cb).wrapping_add(NOTIFY_SLOT));
        let notify: unsafe extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        notify(cb, rule);
        0
    }
});
