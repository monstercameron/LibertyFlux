// original: 0x00b01ea0 heapify_range
/// Build a max-heap over (end-base)/8 records by sifting down from the
/// middle to the front. Returns the last sift-down result.
/// Note: inputs with fewer than 2 records return the incoming EAX, which a
/// Rust rewrite cannot observe; the contract constrains the range so that
/// path never runs (it is 6 instructions returning garbage).
export!(cdecl, rw_00b01ea0(base: u32, end: u32, x: u32) -> u32 {
    let count = (end.wrapping_sub(base) as i32) >> 3;
    let mut k = (count - 2) / 2;
    let mut bx = base.wrapping_add((k as u32).wrapping_mul(8));
    let mut last: u32 = unsafe {
        let w0 = (bx as *const u32).read_unaligned();
        let w1 = (bx.wrapping_add(4) as *const u32).read_unaligned();
        callee_cdecl!(1, u32, base, k as u32, count as u32, w0, w1, x)
    };
    if k != 0 {
        loop {
            let w1 = unsafe { (bx.wrapping_sub(4) as *const u32).read_unaligned() };
            bx = bx.wrapping_sub(8);
            k -= 1;
            let w0 = unsafe { (bx as *const u32).read_unaligned() };
            last = callee_cdecl!(1, u32, base, k as u32, count as u32, w0, w1, x);
            if k == 0 {
                break;
            }
        }
    }
    last
});
