// original: 0x009271F0 input_float_publish
/// Publish float parameters for slot `a0`: three records plus a block ref.
///
/// Converts `a1` to float and publishes record `{0, a1 as f32, 0, 0}` under
/// the key from `0x0119D050`; publishes `{m12C, 0, m134, C / m114}` under
/// the key from `0x0119D044` where the `m` words come from the slot's row
/// and `C` is the constant at `0x00FE8D94`; publishes `{0, 0, m110, m114}`
/// under the key from `0x0119D04C`; then publishes a reference to the
/// slot's row at +0x80 under the key from `0x0119D034`. Slot rows stride
/// 0x110 from `0x0119F100`. All float work is single-precision SSE.
export!(cdecl, rw_009271F0(a0: u32, a1: u32) -> u32 {
    unsafe {
        let f1 = (a1 as i32) as f32;
        let s1 = [0u32, f1.to_bits(), 0, 0];
        callee_cdecl!(1, u32, *global::<u32>(0x119D050), s1.as_ptr() as u32);
        let b = relocated(0x119F100).wrapping_add(a0.wrapping_mul(0x110));
        let g = |off: u32| *((b.wrapping_add(off)) as *const u32);
        let (g110, g114, g12c, g134) = (g(0x10), g(0x14), g(0x2C), g(0x34));
        let ratio = f32::from_bits(*global::<u32>(0xFE8D94)) / f32::from_bits(g114);
        let s2 = [g12c, 0, g134, ratio.to_bits()];
        callee_cdecl!(1, u32, *global::<u32>(0x119D044), s2.as_ptr() as u32);
        let s3 = [0u32, 0, g110, g114];
        callee_cdecl!(1, u32, *global::<u32>(0x119D04C), s3.as_ptr() as u32);
        callee_cdecl!(2, u32, *global::<u32>(0x119D034), b.wrapping_add(0x80));
        0
    }
});
