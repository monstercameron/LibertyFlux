// original: 0x00901150 input_index_in_range (proposed)
/// Report whether either index is outside the table bounds.
///
/// Returns 1 when `a` or `b` is negative or above `limit - 1`, where `limit`
/// is the static dimension word; otherwise 0. All four comparisons are
/// signed (`cmovs`/`cmovg`), and the limit-minus-one wraps. Cdecl with two
/// stack words.
export!(cdecl, rw_00901150(a: u32, b: u32) -> u32 {
    unsafe {
        /// Static table dimension (file VA).
        const DIM: u32 = 0x010344E4;
        let n = (global::<u32>(DIM)).read_unaligned();
        let lim = n.wrapping_sub(1) as i32;
        let x = a as i32;
        let y = b as i32;
        if x < 0 || x > lim || y < 0 || y > lim {
            1
        } else {
            0
        }
    }
});
