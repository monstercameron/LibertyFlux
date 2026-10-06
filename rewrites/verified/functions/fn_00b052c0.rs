// original: 0x00b052c0 sift_down_range
/// Sift-down pass over an 8-byte-record span.
///
/// cdecl `(first, last, aux)`: `count = (last - first) >> 3` (SIGNED);
/// returns at once when `count < 2` (passthrough return, not compared).
/// Otherwise calls the sift helper (helper 1, cdecl
/// `(first, i, count, rec_lo, rec_hi, aux)`) for `i` from `(count-2)/2`
/// (SIGNED division) down to 0, where `(rec_lo, rec_hi)` are the two
/// words at `first + i*8`, read fresh for each call.
export!(cdecl, rw_00b052c0(a0: u32, a1: u32, a2: u32) -> u32 {
    let count = ((a1.wrapping_sub(a0)) as i32) >> 3;
    if count < 2 {
        return 0; // unchecked passthrough, see doc comment
    }
    let mut i = (count - 2) / 2;
    let mut ptr = a0.wrapping_add((i as u32).wrapping_mul(8));
    loop {
        let lo = unsafe { (ptr as *const u32).read_unaligned() };
        let hi = unsafe { ((ptr + 4) as *const u32).read_unaligned() };
        let _: u32 = callee_cdecl!(1, u32, a0, i as u32, count as u32, lo, hi, a2);
        if i == 0 {
            break;
        }
        i -= 1;
        ptr = ptr.wrapping_sub(8);
    }
    0 // unchecked: last helper answer or passthrough, see doc comment
});
