// original: 0x00e5fe90 broadcast_timing_factor_3
/// Copy the shared timing factor to three consumer slots.
///
/// A bit-exact copy; the value is a float but no arithmetic is done.
export!(cdecl, rw_00e5fe90() -> u32 {
    unsafe {
        /// Shared source value.
        const SOURCE: u32 = 0x017AD148;
        /// Consumer slots receiving the copy.
        const DESTS: [u32; 3] = [0x01BB3900, 0x01BB3904, 0x01BB3908];
        let v: f32 = *global::<f32>(SOURCE);
        for d in DESTS {
            *global::<f32>(d) = v;
        }
        0
    }
});
