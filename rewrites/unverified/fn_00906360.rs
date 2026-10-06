// original: 0x00906360 input_refresh_all (proposed)
/// Refresh every cell of the dimension-by-dimension grid.
///
/// Calls the cell refresher callee with `(y, x)` for each `y` below the
/// static dimension and each `x` below it (both signed bounds; nothing
/// happens when the dimension is not positive). Returns the dimension.
/// Cdecl with no arguments.
export!(cdecl, rw_00906360() -> u32 {
    unsafe {
        /// Static grid dimension (file VA).
        const DIM: u32 = 0x010344E4;
        const CELL_ID: u32 = 1;
        let n = (global::<u32>(DIM)).read_unaligned() as i32;
        if n > 0 {
            let mut y: i32 = 0;
            while y < n {
                let mut x: i32 = 0;
                while x < n {
                    let _: u32 = callee_cdecl!(CELL_ID, u32, y as u32, x as u32);
                    x += 1;
                }
                y += 1;
            }
        }
        n as u32
    }
});
