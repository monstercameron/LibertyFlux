// original: 0x00ae2440 ui_grid48_plot_global
/// Plot a point into the shared 48-wide grid.
///
/// Forwards the three arguments unchanged to the 48-wide grid plotter with
/// the shared grid object. Returns the plotter's answer.
export!(cdecl, rw_00ae2440(a: u32, b: u32, c: u32) -> u32 {
    unsafe {
        let grid = relocated(0x15C1610);
        callee_thiscall!(1, u32, grid, a, b, c)
    }
});
