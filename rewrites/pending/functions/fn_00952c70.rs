// original: 0x00952c70 store_adjusted_value
/// Take bits 3-5 of the source object's flag byte as a table index, add the
/// table entry to the helper's answer for the source object, store the sum
/// through the destination pointer and return it.
export!(cdecl, rw_00952c70(dst: *mut u32, src: *const u8) -> u32 {
    unsafe {
        let index = (*src.add(0x10) >> 3) & 7;
        let answer: u32 = callee_thiscall!(1, u32, src as u32);
        let base = *global::<u32>(0x011f7000 + (index as u32) * 4);
        let sum = answer.wrapping_add(base);
        *dst = sum;
        sum
    }
});
