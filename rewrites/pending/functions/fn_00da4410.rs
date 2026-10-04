// original: 0x00da4410 can_add_member_under_cap
// s10f12: whether a probe value may join the row found by keys (thiscall/2).
//
// A null probe is rejected. When no row matches the keys the join is allowed.
// Otherwise the row's live-entry count must be strictly below (signed) the
// cap stored in the row.
export!(thiscall, rw_s10f12(this: *const u8, a1: u32, a2: u32) -> u8 {
    unsafe {
        if a1 == 0 {
            return 0;
        }
        let find: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let found = find(this as u32, a1, a2);
        if found == 0 {
            return 1;
        }
        let count: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        let n = count(found);
        let cap = *((found as *const u32).add(0x3c / 4));
        ((n as i32) < (cap as i32)) as u8
    }
});
