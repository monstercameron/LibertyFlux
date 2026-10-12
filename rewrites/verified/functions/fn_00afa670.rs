// original: 0x00AFA670 vehicle_population_get_spawn_multiplier
/// Select the single-precision multiplier from the signed population state.
/// Negative values return exactly 1.0; zero and positive values return the
/// stored 2.0 multiplier. The float is returned through x87 ST0.
lf_checker_rt::export!(cdecl, rw_00afa670() -> f32 {
    const POPULATION_STATE: u32 = 0x0160031C;
    const SPAWN_MULTIPLIER: u32 = 0x00FE8A24;

    unsafe {
        if lf_checker_rt::global::<i32>(POPULATION_STATE).read_volatile() < 0 {
            1.0
        } else {
            lf_checker_rt::global::<f32>(SPAWN_MULTIPLIER).read_volatile()
        }
    }
});
