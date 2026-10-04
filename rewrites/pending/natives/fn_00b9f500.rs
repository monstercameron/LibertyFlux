// original: 0x00b9f500 GET_RANDOM_CHAR_IN_AREA_OFFSET_NO_SAVE
use lf_k2_rt::{callee_cdecl, export};
/// Finds a random character in a box volume (six floats, one out word).
///
/// Forwards the six float bound words and the trailing out-slot word to
/// the engine search function.
export!(cdecl, rw_00B9F500(ctx: *const u8) -> u32 {
    unsafe {
        let a = *(ctx.add(8) as *const *const u32);
        let f = |i: usize| f32::from_bits(*a.add(i)).to_bits();
        callee_cdecl!(1, u32, f(0), f(1), f(2), f(3), f(4), f(5), *a.add(6))
    }
});
