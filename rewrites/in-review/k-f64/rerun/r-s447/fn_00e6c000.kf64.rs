// ---- 0x00e6c000 veh_f64_convert_store ----
// original: 0x00e6c000 veh_f64_convert_store
// Adapted by lane k-f64 from r-s447's deferred rewrite for the doubles
// extension (unadopted): the helper call now crosses with the 8-byte
// transport and the full double answer. Ready for a stock re-run, not verified.
/// Pass a global double through a helper, store the float result.
///
/// Loads the double at `SRC` (0x00E9B9E0) into XMM0, calls the stubbed
/// helper (takes its argument in XMM0, returns a double in XMM0),
/// converts the answer to single precision (`cvtsd2ss`) and stores it
/// into `DST` (0x0171BC08). Takes no arguments, leaves EAX untouched.
/// NOTE (k-f64): the double argument compares bit for bit now (low 8
/// bytes of XMM0); the double answer arrives as the callee's u64 return.
///
/// Original: 0x00E6C000, cdecl, no arguments.
export!(cdecl, rw_00e6c000() -> u32 {
    unsafe {
        const SRC: u32 = 0xE9B9E0;
        const DST: u32 = 0x171BC08;
        let loaded = *global::<u64>(SRC);
        let ans: u64 = callee_cdecl!(
            1,
            u64,
            (loaded & 0xFFFF_FFFF) as u32,
            ((loaded >> 32) & 0xFFFF_FFFF) as u32
        );
        let v = core::hint::black_box(f64::from_bits(ans)) as f32;
        *global::<u32>(DST) = v.to_bits();
        0
    }
});

// Wrong version (k-f64; the lane's gap variant source was not kept):
// loads the neighbouring double instead of SRC. Must FAIL on the
// compared double argument.
export!(cdecl, mut_rw_00e6c000() -> u32 {
    unsafe {
        const DST: u32 = 0x171BC08;
        let loaded = *global::<u64>(0xE9B9E8);
        let ans: u64 = callee_cdecl!(
            1,
            u64,
            (loaded & 0xFFFF_FFFF) as u32,
            ((loaded >> 32) & 0xFFFF_FFFF) as u32
        );
        let v = core::hint::black_box(f64::from_bits(ans)) as f32;
        *global::<u32>(DST) = v.to_bits();
        0
    }
});
