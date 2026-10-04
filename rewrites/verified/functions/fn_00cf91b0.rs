// original: 0x00cf91b0 climb_task_register (proposed)

/// Registers a climb task in the live list unless already present: looks the
/// task's class up in the global table by the signed word at `+0x2e` and
/// returns early when its flag word at `+0x40` has bit 16 clear. Otherwise
/// scans the list's items for the task; when absent (growing the list through
/// the grow helper first when full, whose realloc effects the stub does not
/// model) appends the task and bumps the count. Returns 1 in the low byte in
/// every case; the upper bytes keep path-dependent leftovers (the table
/// entry pointer, the slot address, or the new count) which the rewrite
/// reproduces exactly.
///
/// Original: 0x00cf91b0 (cdecl, two stack words: task, list).
lf_checker_rt::export!(cdecl, rw_00cf91b0(task: u32, list: u32) -> u32 {
    unsafe {
        const TABLE_ADDR: u32 = 0x01295cd8;
        const FLAG_OFFSET: u32 = 0x40;
        const FLAG_BIT: u32 = 0x1_0000;
        const CLASS_OFFSET: u32 = 0x2e;
        const GROW_CALLEE: u32 = 1;
        const GROW_BY: u32 = 0x10;
        let class = ((task + CLASS_OFFSET) as *const i16).read_unaligned() as i32;
        let table = lf_checker_rt::relocated(TABLE_ADDR);
        let entry = ((table + (class as u32).wrapping_mul(4)) as *const u32).read_unaligned();
        let flags = ((entry + FLAG_OFFSET) as *const u32).read_unaligned();
        if flags & FLAG_BIT == 0 {
            return (entry & 0xffff_ff00) | 1;
        }
        let count = ((list + 4) as *const u16).read_unaligned() as i32;
        let mut found_at: u32 = 0;
        let mut found = false;
        if count > 0 {
            let items = ((list + 0) as *const u32).read_unaligned();
            let mut i = 0i32;
            while i < count {
                let slot = items.wrapping_add((i as u32).wrapping_mul(4));
                if ((slot) as *const u32).read_unaligned() == task {
                    found = true;
                    found_at = slot;
                    break;
                }
                i += 1;
            }
        }
        if found {
            return (found_at & 0xffff_ff00) | 1;
        }
        let cap = ((list + 6) as *const u16).read_unaligned();
        if (count as u16) == cap {
            lf_checker_rt::callee_thiscall!(GROW_CALLEE, u32, list, GROW_BY);
        }
        let n = ((list + 4) as *const u16).read_unaligned();
        let base = ((list + 0) as *const u32).read_unaligned();
        let slot = base.wrapping_add((n as u32).wrapping_mul(4));
        ((list + 4) as *mut u16).write_unaligned(n.wrapping_add(1));
        (slot as *mut u32).write_unaligned(task);
        (((n as u32).wrapping_add(1)) & 0xffff_ff00) | 1
    }
});
