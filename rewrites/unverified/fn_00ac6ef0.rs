// original: 0x00AC6EF0 stream_set_scaled_param (proposed)

/// Store `scale * arg` into the parameter slot when it differs, then notify.
///
/// The original multiplies the scale global by the float argument, compares
/// the product with the stored slot (ordered comparison: NaN always differs,
/// +0 and -0 are equal) and returns unchanged on equality (cdecl, one word).
/// Otherwise it stores the product and calls the notify callee with
/// (handle, slot address, 1, 5). The multiply order is the original's.
/// No value is returned.
lf_checker_rt::export!(cdecl, rw_00AC6EF0(arg: u32) -> u32 {
    unsafe {
        const SCALE: u32 = 0x0103F250;
        const SLOT: u32 = 0x0103F260;
        const HANDLE: u32 = 0x0154E014;
        const NOTIFY: u32 = 1;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        unsafe {
            let scale: f32 = (lf_checker_rt::relocated(SCALE) as *const f32).read_unaligned();
            let prod = mul(scale, f32::from_bits(arg));
            let cur: f32 = (lf_checker_rt::relocated(SLOT) as *const f32).read_unaligned();
            if cur == prod {
                return 0;
            }
            (lf_checker_rt::relocated(SLOT) as *mut f32).write_unaligned(prod);
            let handle = (lf_checker_rt::relocated(HANDLE) as *const u32).read_unaligned();
            lf_checker_rt::callee_cdecl!(NOTIFY, u32, handle, lf_checker_rt::relocated(SLOT), 1u32, 5u32);
            0
        }
    }
});
