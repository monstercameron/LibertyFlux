// original: 0x00e62d90 refresh_ratio_116bfe8
/// Refresh the stored ratio for the first scale slot.
export!(cdecl, rw_00e62d90() -> () {
    unsafe {
        let a = *global::<f32>(0x01031910);
        let b = *global::<f32>(0x01031914);
        *global::<f32>(0x0116bfe8) = a / b;
    }
});
