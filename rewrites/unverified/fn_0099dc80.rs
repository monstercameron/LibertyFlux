// original: 0x0099dc80 intro_sort_entry
/// Entry point of the 16-byte-record introsort.
///
/// Returns at once for an empty range. Otherwise it derives a depth budget
/// of twice the binary logarithm of the record count, sorts through callee 1
/// (the bounded quicksort) and finishes through callee 2 (the small-range
/// sorter). Ranges whose signed record count is zero or negative hang the
/// original (the halving loop never reaches one); the contract only feeds it
/// empty or positive ranges.
export!(cdecl, rw_0099dc80(a0: u32, a1: u32, aux: u32) -> () {
    unsafe {
        if a0 == a1 {
            return;
        }
        let count = (a1.wrapping_sub(a0) as i32) >> 4;
        let mut depth = 0u32;
        if count != 1 {
            let mut c = count;
            loop {
                c >>= 1;
                depth += 2;
                if c == 1 {
                    break;
                }
            }
        }
        callee_cdecl!(1, u32, a0, a1, 0, depth, aux);
        callee_cdecl!(2, u32, a0, a1, aux);
    }
});
