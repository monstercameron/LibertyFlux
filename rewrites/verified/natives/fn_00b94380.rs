// original: 0x00b94380 GET_BITS_IN_RANGE
use lf_k2_rt::{callee_cdecl, export};
// GET_BITS_IN_RANGE: forward (value, lo, hi); store the full answer
// through the return slot and return it.
export!(cdecl, rw_00B94380(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        let bits = callee_cdecl!(1, u32, *a, *a.add(1), *a.add(2));
        *ret_slot(ctx) = bits;
        bits
    }
});
