// original: 0x00da42f0 find_obj_and_test_member
// s10f10: find a row object by keys, keep it when it holds a member (thiscall/3).
//
// Looks the row up by the two keys, then tests the probe value against the
// row's member lists. Returns the row when both steps hit, else null. Only
// the low byte of the test answer matters.
export!(thiscall, rw_s10f10(this: *const u8, x: u32, k1: u32, k2: u32) -> u32 {
    unsafe {
        let find: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let found = find(this as u32, k1, k2);
        if found == 0 {
            return 0;
        }
        let test: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        if test(found, x) & 0xFF == 0 {
            return 0;
        }
        found
    }
});
