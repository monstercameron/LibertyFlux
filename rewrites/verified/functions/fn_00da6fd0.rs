// original: 0x00DA6FD0 planar_distance_gate

/// Compute the signed planar displacement between this task and the
/// pointed-to entity, then ask the shared distance helper to classify it.
/// The inner entity pointer is stored at outer offset `+0x20`; coordinates
/// at `+0x30` and `+0x34` are subtracted from this task in that order.
/// The two f32 results are forwarded as exact stack words to a cdecl helper.
lf_checker_rt::export!(thiscall, rw_00DA6FD0(this: u32, outer: u32) -> u32 {
    unsafe {
        const INNER_ENTITY: u32 = 0x20;
        const X_COORDINATE: u32 = 0x30;
        const Y_COORDINATE: u32 = 0x34;
        const DISTANCE_HELPER: u32 = 1;

        let inner = (outer.wrapping_add(INNER_ENTITY) as *const u32).read_unaligned();
        let target_x_bits = (inner.wrapping_add(X_COORDINATE) as *const u32).read_unaligned();
        let self_x_bits = (this.wrapping_add(X_COORDINATE) as *const u32).read_unaligned();
        let target_y_bits = (inner.wrapping_add(Y_COORDINATE) as *const u32).read_unaligned();
        let self_y_bits = (this.wrapping_add(Y_COORDINATE) as *const u32).read_unaligned();

        let delta_x = (core::hint::black_box(f32::from_bits(target_x_bits))
            - core::hint::black_box(f32::from_bits(self_x_bits))).to_bits();
        let delta_y = (core::hint::black_box(f32::from_bits(target_y_bits))
            - core::hint::black_box(f32::from_bits(self_y_bits))).to_bits();
        lf_checker_rt::callee_cdecl!(DISTANCE_HELPER, u32, delta_x, delta_y)
    }
});
