// original: 0x00BA61D0 NativeImpl_FORCE_SPAWN_SCENARIO_PEDS_IN_AREA_2

lf_checker_rt::export!(cdecl, rw_00ba61d0(
    vector_x: f32,
    vector_y: f32,
    vector_z: f32,
    fourth_value: f32,
    fifth_value: f32,
) -> u32 {
    const SPAWN_SCENARIO_HELPER: u32 = 1;
    const SPAWN_MODE: u32 = 1;
    let area = [vector_x, vector_y, vector_z];

    let _ignored = lf_checker_rt::callee_cdecl!(
        SPAWN_SCENARIO_HELPER,
        u32,
        area.as_ptr() as u32,
        SPAWN_MODE,
        fourth_value.to_bits(),
        fifth_value.to_bits(),
    );
    0
});
