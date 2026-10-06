// original: 0x006A0100 stride2_copy
/// Copy `count` bytes from even source positions: `dst[i] = src[2*i]`.
///
/// `count` is signed: nothing is copied when it is zero or negative (`jle`
/// / `jl`). Returns `dst` (the entry value of eax is never changed).
/// Original: cdecl, three stack words (dst, src, count), no calls.
lf_checker_rt::export!(cdecl, rw_006a0100(dst: u32, src: u32, count: u32) -> u32 {
    unsafe {
        let n = count as i32;
        if n > 0 {
            let mut i = 0i32;
            while i < n {
                let b = (src.wrapping_add((i as u32).wrapping_mul(2)) as *const u8).read();
                (dst.wrapping_add(i as u32) as *mut u8).write(b);
                i += 1;
            }
        }
        dst
    }
});