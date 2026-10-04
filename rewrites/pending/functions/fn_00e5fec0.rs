// original: 0x00e5fec0 copy_timing_factor_1
/// Copy the shared timing factor to its consumer slot.
///
/// A bit-exact copy; the value is a float but no arithmetic is done.
export!(cdecl, rw_00e5fec0() -> u32 {
    unsafe {
        /// Shared source value.
        const SOURCE: u32 = 0x017AD148;
        /// Consumer slot receiving the copy.
        const DEST: u32 = 0x01BB38FC;
        *global::<f32>(DEST) = *global::<f32>(SOURCE);
        0
    }
});
