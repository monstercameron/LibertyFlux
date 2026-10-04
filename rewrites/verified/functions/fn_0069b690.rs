// original: 0x0069b690 rage::crAnimChannelStaticInt::vf17
/// Static-int uniformity check: verifies every array element equals its
/// predecessor, storing the first element to +0x08 along the way (even for
/// short arrays). Returns 1 when all match, 0 on the first mismatch.
lf_k2_rt::export!(thiscall, rw_0069b690(this: *mut u8, arr: *const u32, count: i32) -> u32 {
    unsafe {
        if count > 1 {
            let mut k = 1i32;
            while k < count {
                let prev = *((arr as *const u32).add((k as usize).wrapping_sub(1)));
                let cur = *((arr as *const u32).add(k as usize));
                if prev != cur {
                    return 0;
                }
                k += 1;
            }
        }
        let v = *(arr as *const u32);
        *((this.add(8)) as *mut u32) = v;
        1
    }
});
