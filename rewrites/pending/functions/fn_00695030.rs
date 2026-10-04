// original: 0x00695030 filter_attach_clear
/// Attach the object to an owner record and clear its value array.
///
/// Records the owner pointer, asks the array helper to size the inline
/// array from the owner's count word, then zeroes every element. Returns
/// the helper's answer when the array is empty, else the array base.
export!(thiscall, rs80_695030(this: *mut u8, owner: *const u8) -> u32 {
    unsafe {
        *((this).add(0x14) as *mut u32) = owner as u32;
        let n = *((owner).add(0x14) as *const u16) as u32;
        let vec = (this as u32).wrapping_add(0x0C);
        let ans: u32 = callee_thiscall!(1, u32, vec, n);
        let count = *((this).add(0x10) as *const u16) as usize;
        let arr = *((this).add(0x0C) as *const u32) as *mut u32;
        for i in 0..count {
            *arr.add(i) = 0;
        }
        if count == 0 { ans } else { arr as u32 }
    }
});
