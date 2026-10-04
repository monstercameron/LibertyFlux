// original: 0x00b8ba30 ADD_BLIP_FOR_COORD
use lf_k2_rt::{callee_cdecl, export};
// ADD_BLIP_FOR_COORD: forward (x, y, z, flags) to the blip engine call.
// The coordinates travel as raw float bits; the handler keeps no result.
export!(cdecl, rw_00B8BA30(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        callee_cdecl!(1, u32, *a, *a.add(1), *a.add(2), *a.add(3))
    }
});
