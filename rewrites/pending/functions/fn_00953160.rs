// original: 0x00953160 take_scale_and_clear
/// Take the global scale factor, clearing the slot back to zero, and return
/// the taken value.
export!(cdecl, rw_00953160() -> f64 {
    unsafe {
        let slot = global::<u32>(0x011f7054);
        let bits = *slot;
        *slot = 0;
        f32::from_bits(bits) as f64
    }
});
