// original: 0x00a3b4a0 vehicle_split_transcend (proposed)

/// Evaluate the double-precision transcendent (id 1, cdecl/3) on a float.
///
/// Widens `f` to `f64`, calls the callee with a scratch slot and the
/// double's two words, stores the callee's out-double narrowed to `f32`
/// into `*out`, and returns its ST0 double narrowed to `f32`. Cdecl/2,
/// returns the float on x87 ST0.
lf_checker_rt::export!(cdecl, rw_00a3b4a0(f_bits: u32, out: u32) -> f32 {
    unsafe {
        const FN: u32 = 1;
        let d = f32::from_bits(f_bits) as f64;
        let bits = d.to_bits();
        let mut w: f64 = 0.0;
        // Stack layout matches the original exactly: the double's two
        // words sit below the out-pointer, so it is argument 2.
        let st: f64 = lf_checker_rt::callee_cdecl!(
            FN,
            f64,
            bits as u32,
            (bits >> 32) as u32,
            core::ptr::addr_of_mut!(w) as u32
        );
        core::ptr::write_unaligned(out as *mut u32, (w as f32).to_bits());
        st as f32
    }
});
