// original: 0x0061BD40 net_forward_grid_5

/// Forward a grid-cell operation with five arguments.
///
/// Computes the row address `a0*0x5580+[this]` and hands it with `a1` in
/// registers and `a2`, `a3`, `a4` on the stack to the shared cell
/// routine (which pops nothing; the caller cleans up). Returns the
/// routine's answer.
/// Original: 0x0061BD40 (thiscall, five stack words).
lf_checker_rt::export!(thiscall, rw_0061BD40(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const CELL_CALLEE: u32 = 1;
        const ROW_BYTES: u32 = 0x5580;
        let grid = (this as *const u32).read_unaligned();
        let row_at = a0.wrapping_mul(ROW_BYTES).wrapping_add(grid);
        lf_checker_rt::callee_fastcall!(CELL_CALLEE, u32, row_at, a1, a2, a3, a4)
    }
});
