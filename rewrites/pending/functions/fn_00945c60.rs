// original: 0x00945c60 fixed_kind_table_search
/// Search the kind-2 table for a target value.
///
/// Fixed-kind sibling of `rw_00945bc0`: the table always comes from the
/// resolver called with kind 2, and the probe count is a full word compared
/// unsigned. Real callers pass small counts (past 256 the low-byte counter
/// would wrap); the contract keeps counts small.
lf_checker_rt::export!(stdcall, rw_00945c60(a1: u32, a2: u32) -> u32 {
    unsafe {
        let edi: u32 = lf_checker_rt::callee_stdcall!(1, u32, 2);
        if edi == 0 {
            return 0;
        }
        if a2 == 0 {
            return 0;
        }
        let target = *((a1.wrapping_add(4)) as *const u32);
        let base = *(edi.wrapping_add(0xf) as *const u8) as u32;
        let mut bl: u32 = 0;
        loop {
            let d = base.wrapping_sub(bl).wrapping_add(9) % 10;
            let p = edi.wrapping_add(0x11).wrapping_add(d * 4) as *const u32;
            if core::ptr::read_unaligned(p) == target {
                return 1;
            }
            bl = (bl + 1) & 0xff;
            if bl >= a2 {
                return 0;
            }
        }
    }
});
