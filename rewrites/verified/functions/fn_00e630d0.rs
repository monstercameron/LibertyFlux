// original: 0x00e630d0 refresh_ratio_117968c
/// Refresh the stored ratio for the thirteenth scale slot.
export!(cdecl, rw_00e630d0() -> () {
    unsafe {
        let a = *global::<f32>(0x010330c0);
        let b = *global::<f32>(0x010330c4);
        *global::<f32>(0x0117968c) = a / b;
    }
});
