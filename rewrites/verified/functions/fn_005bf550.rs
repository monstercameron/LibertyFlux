// original: 0x005BF550 fill_rotation_coefficients_variant_a
/// Fill the destination's 0x2c-byte coefficient block from two helper
/// results. The first result is written at offsets 0x14 and 0x28; the second
/// is written at 0x18 and again with its sign bit toggled at 0x24. The other
/// fields are fixed zeroes or the bit pattern for 1.0.
///
/// The original is thiscall with one stack word. Its direct helpers take the
/// input in XMM0 and return a float in XMM0; the contract transports that
/// scalar through the low 32 bits of the logged register.
lf_checker_rt::export!(thiscall, rw_005bf550(dst: u32, angle_bits: u32) -> u32 {
    const HELPER_A: u32 = 1;
    const HELPER_B: u32 = 2;
    const ONE_BITS: u32 = 0x3f80_0000;
    const SIGN_BIT: u32 = 0x8000_0000;
    let first = lf_checker_rt::callee_cdecl!(HELPER_A, u32, angle_bits);
    let second = lf_checker_rt::callee_cdecl!(HELPER_B, u32, angle_bits);
    unsafe {
        let p = dst as *mut u32;
        p.add(0).write(ONE_BITS);
        p.add(1).write(0);
        p.add(2).write(0);
        p.add(4).write(0);
        p.add(5).write(first);
        p.add(6).write(second);
        p.add(8).write(0);
        p.add(9).write(second ^ SIGN_BIT);
        p.add(10).write(first);
    }
    0
});
