// original: 0x006479f0 rage::ptxEmitRuleStd::vf10

/// Clear the rule's armed bits (bits 1 and 3 at `+0x104`) and, when `cb`
/// is non-null and the source's flag byte at `+0x254` has bit 1 set,
/// notify through the callback's virtual slot `+0x1c` with the rule
/// pointer. Stdcall, three stack arguments, no return value.
lf_checker_rt::export!(stdcall, rw_006479f0(src: u32, rule: u32, cb: u32) -> u32 {
    unsafe {
        const RULE_FLAGS: u32 = 0x104;
        const ARMED_MASK: u32 = 0xffff_fff5;
        const SOURCE_FLAG: u32 = 0x254;
        const READY_BIT: u8 = 0x02;
        const NOTIFY_SLOT: u32 = 0x1c;

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
        let ready =
            ((src.wrapping_add(SOURCE_FLAG)) as *const u8).read() & READY_BIT;
        if ready == 0 {
            return 0;
        }
        let slot = read_word(read_word(cb).wrapping_add(NOTIFY_SLOT));
        let notify: unsafe extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        notify(cb, rule);
        0
    }
});
