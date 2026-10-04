// original: 0x00b8caf0 GET_ROUTE_SIZE
use lf_k2_rt::{callee_cdecl, export};
/// Returns the route count in the return slot.
///
/// Calls the engine function with no arguments and stores its full
/// 32-bit answer in the return slot.
export!(cdecl, rw_00B8CAF0(ctx: *const u8) -> u32 {
    unsafe {
        let ans: u32 = callee_cdecl!(1, u32,);
        let ret = *(ctx as *const *mut u32);
        *ret = ans;
        ans
    }
});
