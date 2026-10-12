// original: 0x00D87810 camera_update_distance_and_vector

/// Computes the camera-to-target X and Y deltas in the original operand
/// order, asks the scalar helper for their distance, stores those exact
/// returned float bits at camera offset 0x1edc, then forwards the camera and
/// both deltas to the vector helper. The function returns that helper's EAX.
lf_checker_rt::export!(cdecl, rw_00d87810(camera: u32) -> u32 {
    const TARGET_SLOT: u32 = 0x20;
    const CAMERA_X: u32 = 0xe50;
    const CAMERA_Y: u32 = 0xe4c;
    const TARGET_X: u32 = 0x34;
    const TARGET_Y: u32 = 0x30;
    const DISTANCE_OUTPUT: u32 = 0x1edc;
    const DISTANCE_HELPER: u32 = 1;
    const VECTOR_HELPER: u32 = 2;

    let target = unsafe { (camera.wrapping_add(TARGET_SLOT) as *const u32).read_unaligned() };
    let camera_x = f32::from_bits(unsafe {
        (camera.wrapping_add(CAMERA_X) as *const u32).read_volatile()
    });
    let target_x = f32::from_bits(unsafe {
        (target.wrapping_add(TARGET_X) as *const u32).read_volatile()
    });
    let camera_y = f32::from_bits(unsafe {
        (camera.wrapping_add(CAMERA_Y) as *const u32).read_volatile()
    });
    let target_y = f32::from_bits(unsafe {
        (target.wrapping_add(TARGET_Y) as *const u32).read_volatile()
    });
    let ordered_subtract = |left: f32, right: f32| {
        core::hint::black_box(left) - core::hint::black_box(right)
    };
    let delta_x = ordered_subtract(camera_x, target_x);
    let delta_y = ordered_subtract(camera_y, target_y);
    let distance = lf_checker_rt::callee_cdecl!(
        DISTANCE_HELPER, f32, delta_x.to_bits(), delta_y.to_bits()
    );
    unsafe {
        (camera.wrapping_add(DISTANCE_OUTPUT) as *mut u32)
            .write_volatile(distance.to_bits());
    }
    lf_checker_rt::callee_cdecl!(
        VECTOR_HELPER, u32, camera, delta_x.to_bits(), delta_y.to_bits()
    )
});
