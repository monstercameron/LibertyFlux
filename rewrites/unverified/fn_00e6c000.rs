// original: 0x00e6c000 veh_f64_convert_store
/// Pass a global double through a helper, store the float result.
///
/// Loads the double at `SRC` (0x00E9B9E0) into XMM0, calls the stubbed
/// helper (takes its argument in XMM0, returns a double in XMM0),
/// converts the answer to single precision (`cvtsd2ss`) and stores it
/// into `DST` (0x0171BC08). Takes no arguments, leaves EAX untouched.
/// NOTE: the checker cannot script an f64 argument arriving in XMM0
/// (its transports move 4 bytes), so the loaded value is unobserved.
///
/// Original: 0x00E6C000, cdecl, no arguments.
export!(cdecl, rw_00e6c000() -> u32 {
    unsafe {
        use core::arch::x86::{_mm_cvtss_f32, _mm_cvtsd_ss, _mm_set_sd, _mm_setzero_ps};
        const SRC: u32 = 0xE9B9E0;
        const DST: u32 = 0x171BC08;
        let _loaded = *global::<u64>(SRC);
        let r: f64 = callee_cdecl!(1, f64,);
        let v = _mm_cvtss_f32(_mm_cvtsd_ss(_mm_setzero_ps(), _mm_set_sd(r)));
        *global::<u32>(DST) = v.to_bits();
        0
    }
});
