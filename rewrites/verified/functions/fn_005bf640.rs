// original: 0x005BF640 fill_rotation_coefficients_variant_c
/// Fill the destination's 0x2c-byte coefficient block from two helper
/// results. The first result is stored at offsets 0 and 0x14; the second at
/// offsets 4 and 0x10 with its sign bit toggled at 0x10. The remaining fields
/// are zeroes except for the final 1.0 word.
///
/// The original is thiscall with one stack word and uses two direct helpers
/// whose scalar XMM0 arguments are supplied by the contract transport.
lf_checker_rt::export!(thiscall, rw_005bf640(dst: u32, angle_bits: u32) -> u32 {
    const HELPER_A: u32 = 1;
    const HELPER_B: u32 = 2;
    const ONE_BITS: u32 = 0x3f80_0000;
    const SIGN_BIT: u32 = 0x8000_0000;
    let first = lf_checker_rt::callee_cdecl!(HELPER_A, u32, angle_bits);
    let second = lf_checker_rt::callee_cdecl!(HELPER_B, u32, angle_bits);
    unsafe {
        let p = dst as *mut u32;
        p.add(0).write(first);
        p.add(1).write(second);
        p.add(2).write(0);
        p.add(4).write(second ^ SIGN_BIT);
        p.add(5).write(first);
        p.add(6).write(0);
        p.add(8).write(0);
        p.add(9).write(0);
        p.add(10).write(ONE_BITS);
    }
    0
});
