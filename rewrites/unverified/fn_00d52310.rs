// original: 0x00D52310 collect_nearby_entities

/// Clears the shared enumeration counter, copies three coordinate words from
/// the camera, and asks the world helper to enumerate entities around them.
/// The helper receives a pointer to the three-word stack buffer, a callback,
/// the camera, a 0x100 range parameter and a count of five. Coordinate words
/// are copied bit for bit without floating-point arithmetic.
lf_checker_rt::export!(thiscall, rw_00d52310(camera: u32) -> () {
    const ENUMERATION_COUNTER_VA: u32 = 0x0172_0fcc;
    const FIRST_COORDINATE: u32 = 0x120;
    const SECOND_COORDINATE: u32 = 0x124;
    const THIRD_COORDINATE: u32 = 0x128;
    const CALLBACK_VA: u32 = 0x00d5_2380;
    const ENUMERATE_OBJECTS: u32 = 1;

    unsafe {
        lf_checker_rt::global::<u32>(ENUMERATION_COUNTER_VA).write_volatile(0);
        let coordinates = [
            (camera.wrapping_add(FIRST_COORDINATE) as *const u32).read_unaligned(),
            (camera.wrapping_add(SECOND_COORDINATE) as *const u32).read_unaligned(),
            (camera.wrapping_add(THIRD_COORDINATE) as *const u32).read_unaligned(),
        ];
        let coordinate_ptr = coordinates.as_ptr() as u32;
        let _ = lf_checker_rt::callee_cdecl!(
            ENUMERATE_OBJECTS, u32,
            coordinate_ptr, CALLBACK_VA, camera, 0x100, 5
        );
    }
});
