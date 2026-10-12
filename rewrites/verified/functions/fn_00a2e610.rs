// original: 0x00A2E610 weather_select_forecast_table
/// Selects one of two forecast tables from the signed mode word in global
/// state. The returned table address is relocated with the loaded image.
lf_checker_rt::export!(cdecl, rw_00A2E610() -> u32 {
    unsafe {
        const FORECAST_MODE: u32 = 0x011D_6FD4;
        const MODE_TWO_TABLE: u32 = 0x0103_C980;
        const OTHER_TABLE: u32 = 0x0103_C900;
        let mode = lf_checker_rt::global::<i32>(FORECAST_MODE).read_volatile();
        let table = if mode == 2 { MODE_TWO_TABLE } else { OTHER_TABLE };
        lf_checker_rt::relocated(table)
    }
});
