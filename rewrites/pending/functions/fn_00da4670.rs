// original: 0x00da4670 find_obj_and_test_member_bool
// s10f16: find a row by keys and test the probe against it (thiscall/3).
//
// Boolean twin of s10f10: true only when the row exists and the membership
// test's low byte is nonzero.
export!(thiscall, rw_s10f16(this: *const u8, x: u32, k1: u32, k2: u32) -> u8 {
    unsafe {
        let find: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let found = find(this as u32, k1, k2);
        if found == 0 {
            return 0;
        }
        let test: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        (test(found, x) & 0xFF != 0) as u8
    }
});
