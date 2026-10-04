// original: 0x00b32d50 task_rule_check (proposed)

/// Test whether a task satisfies the rule picked by a sub-task's kind.
///
/// `task` points to the task record, `sub` to a sub-task record whose first
/// word is a kind. Kinds 1, 22 and 23 ask whether the task's flag byte at
/// `+0x26c` has bit 2 set and one of the sub-task's child pointers at
/// `+0x28`/`+0x2c` equals the task's link word at `+0xb30`; kind 15 only asks
/// for the flag bit; kinds 17 and 18 ask for bit 26 of the task's word at
/// `+0x260`; kinds 3 and 27 hand the `+0x28` child, and kind 28 the `+0x2c`
/// child, to a check callee together with the task, provided the child is
/// non-null and carries flag bits `0x3c0` equal to `0xc0`. Any other kind
/// fails. A pass returns the deciding word with its low byte set to 1 (the
/// link word, the task pointer, or the callee answer); a fail returns the
/// word at hand with its low byte cleared.
///
/// Original: 0x00b32d50 (cdecl, two stack words; a range-checked jump table;
/// one two-word callee reached from two sites).
lf_checker_rt::export!(cdecl, rw_00b32d50(task: u32, sub: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0x26c;
        const FLAG_BIT: u8 = 4;
        const WORD_OFF: u32 = 0x260;
        const WORD_BIT: u32 = 0x4000000;
        const LINK_OFF: u32 = 0xb30;
        const CHILD_A: u32 = 0x28;
        const CHILD_B: u32 = 0x2c;
        const CHECK: u32 = 1;
        #[inline(always)]
        unsafe fn r32(o: u32, off: u32) -> u32 {
            unsafe { (o as *const u32).byte_add(off as usize).read_unaligned() }
        }
        let kind = r32(sub, 0);
        match kind {
            1 | 22 | 23 => {
                if (task as *const u8).byte_add(FLAG_OFF as usize).read() & FLAG_BIT == 0 {
                    return task & !0xFF;
                }
                let link = r32(task, LINK_OFF);
                if r32(sub, CHILD_A) == link || r32(sub, CHILD_B) == link {
                    link & !0xFF | 1
                } else {
                    link & !0xFF
                }
            }
            3 | 27 => {
                let child = r32(sub, CHILD_A);
                if child == 0 {
                    // The table index byte (1) with its low byte cleared.
                    return 0;
                }
                let masked = r32(child, CHILD_A) & 0x3c0;
                if masked != 0xc0 {
                    return masked & !0xFF;
                }
                let answer: u32 = lf_checker_rt::callee_cdecl!(CHECK, u32, task, child);
                if answer & 0xFF != 0 {
                    answer & !0xFF | 1
                } else {
                    answer & !0xFF
                }
            }
            15 => {
                if (task as *const u8).byte_add(FLAG_OFF as usize).read() & FLAG_BIT != 0 {
                    task & !0xFF | 1
                } else {
                    task & !0xFF
                }
            }
            17 | 18 => {
                if r32(task, WORD_OFF) & WORD_BIT != 0 {
                    task & !0xFF | 1
                } else {
                    task & !0xFF
                }
            }
            28 => {
                let child = r32(sub, CHILD_B);
                if child == 0 {
                    // The table index byte (4) with its low byte cleared.
                    return 0;
                }
                let masked = r32(child, CHILD_A) & 0x3c0;
                if masked != 0xc0 {
                    return masked & !0xFF;
                }
                let answer: u32 = lf_checker_rt::callee_cdecl!(CHECK, u32, task, child);
                if answer & 0xFF != 0 {
                    answer & !0xFF | 1
                } else {
                    answer & !0xFF
                }
            }
            _ => {
                let k = kind.wrapping_sub(1);
                if k > 0x1b {
                    k & !0xFF
                } else {
                    // The table index byte (5) with its low byte cleared.
                    0
                }
            }
        }
    }
});
