// original: 0x008E6E20 NativeImpl_LOAD_ALL_PATH_NODES (symbols)

/// Test whether every path node in a quantised rectangle is loaded.
///
/// The four stack floats are the rectangle corners; each is quantised by
/// the indexer (callee 1, thiscall on `obj`) in the order second, first,
/// fourth, third, giving `r1` (row stride base), `r2` (last row), `r3`
/// (last column) and `r4` (first row). All four bound comparisons are
/// signed: when `r4 > r2` the rectangle is empty and the result is true
/// without reading any cell; otherwise every cell of rows `r4..=r2` and
/// columns `0..=(r3 - r1)` is read at `obj` plus the row term
/// `(r1 * 8 + GRID_BIAS + row) * 4` plus `column * COL_STRIDE`, and the
/// result is true only when every cell is non-zero. A row with `r1 > r3`
/// contributes no cells. Returns 1 or 0 in the low byte.
///
/// Original: 0x008E6E20 (thiscall, four float stack arguments).
lf_checker_rt::export!(thiscall, rw_008E6E20(obj: u32, f0: u32, f1: u32, f2: u32, f3: u32) -> u32 {
    unsafe {
        /// Bias added to the row term before scaling.
        const GRID_BIAS: u32 = 0x201;
        /// Row stride multiplier applied to the first index.
        const ROW_MUL: u32 = 8;
        /// Column stride in bytes.
        const COL_STRIDE: u32 = 0x20;
        /// Indexer callee id.
        const INDEX: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let r1: u32 = lf_checker_rt::callee_thiscall!(INDEX, u32, obj, f2);
        let r2: u32 = lf_checker_rt::callee_thiscall!(INDEX, u32, obj, f1);
        let r3: u32 = lf_checker_rt::callee_thiscall!(INDEX, u32, obj, f3);
        let r4: u32 = lf_checker_rt::callee_thiscall!(INDEX, u32, obj, f0);
        if (r4 as i32) > (r2 as i32) {
            return 1;
        }
        let mut row = r4 as i32;
        while (row as i32) <= (r2 as i32) {
            if (r1 as i32) <= (r3 as i32) {
                let base = obj.wrapping_add(
                    (r1.wrapping_mul(ROW_MUL))
                        .wrapping_add(GRID_BIAS)
                        .wrapping_add(row as u32)
                        .wrapping_mul(4),
                );
                let mut col = r1 as i32;
                let mut cell = base;
                loop {
                    if rd32(cell) == 0 {
                        return 0;
                    }
                    col = col.wrapping_add(1);
                    if col > (r3 as i32) {
                        break;
                    }
                    cell = cell.wrapping_add(COL_STRIDE);
                }
            }
            row = row.wrapping_add(1);
        }
        1
    }
});
