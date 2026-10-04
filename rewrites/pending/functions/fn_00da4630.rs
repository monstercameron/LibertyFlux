// original: 0x00da4630 NativeImpl_IS_CHAR_USING_MAP_ATTRACTOR_3
// s10f15: whether any row in the list accepts the probe (thiscall/1).
//
// Tries each row pointer in turn through the membership test; only the low
// byte of each answer matters. True on the first accept, false when no row
// accepts or the list is empty.
export!(thiscall, rw_s10f15(this: *const u8, x: u32) -> u8 {
    unsafe {
        let begin = *(this as *const u32);
        let end = *(this.add(4) as *const u32);
        let n = (end.wrapping_sub(begin) as i32) >> 2;
        let test: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let mut i = 0i32;
        while i < n {
            let p = *((begin as *const u32).offset(i as isize));
            if test(p, x) & 0xFF != 0 {
                return 1;
            }
            i += 1;
        }
        0
    }
});
