// original: 0x00e6be80 veh_scaled_int_store
/// Multiply a global integer by six into another global.
///
/// Loads `SRC` (0x01050AE0), multiplies by 6 with wrapping arithmetic
/// (the original does `(an instruction of the original)` then `(an instruction of the original)`),
/// stores the result into `DST` (0x01713A94) and returns it in EAX.
/// Takes no arguments.
///
/// Original: 0x00E6BE80, cdecl, no arguments.
export!(cdecl, rw_00e6be80() -> u32 {
    unsafe {
        const SRC: u32 = 0x1050AE0;
        const DST: u32 = 0x1713A94;
        const FACTOR: u32 = 6;
        let v = (*global::<u32>(SRC)).wrapping_mul(FACTOR);
        *global::<u32>(DST) = v;
        v
    }
});
