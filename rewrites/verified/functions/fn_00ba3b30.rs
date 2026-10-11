// original: 0x00BA3B30 NativeImpl_ADD_SCENARIO_BLOCKING_AREA

lf_checker_rt::export!(cdecl, rw_00ba3b30(
    first_x: f32,
    first_y: f32,
    first_z: f32,
    second_x: f32,
    second_y: f32,
    second_z: f32,
) -> u32 {
    const SCENARIO_AREA_HELPER: u32 = 1;

    let first_vector = [first_x, first_y, first_z];
    let second_vector = [second_x, second_y, second_z];

    unsafe {
        let _ignored = lf_checker_rt::callee_cdecl!(
            SCENARIO_AREA_HELPER,
            u32,
            first_vector.as_ptr() as u32,
            second_vector.as_ptr() as u32,
        );
    }
    0
});
