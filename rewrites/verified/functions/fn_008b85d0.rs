// original: 0x008b85d0 nth_present_pad
/// Nth present pad finder.
///
/// Returns the index of the `n`th present pad (0 means the first present pad),
/// chaining the present-pad scan from pad 0. Returns -1 when fewer than
/// `n`+1 pads are present.
export!(cdecl, rw_008b85d0(n: u32) -> u32 {
    unsafe {
        let mut found: u32 = callee_cdecl!(2, u32, 0);
        let mut left = n;
        if left == 0 {
            return found;
        }
        loop {
            left = left.wrapping_sub(1);
            if found == 0xFFFF_FFFF {
                return found;
            }
            found = callee_cdecl!(2, u32, found);
            if left == 0 {
                return found;
            }
        }
    }
});
