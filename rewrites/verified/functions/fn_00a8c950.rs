// original: 0x00A8C950 pool_matching_cells_collect (proposed)

/// Prepend every matching cell of every row to the list at `*head`.
///
/// The count helper (with 0) gives the row count; non-positive counts do
/// nothing. For each row and each of four columns the cell helper runs;
/// non-null cells are wrapped by the allocator (whose object is the global
/// pointer) and prepended: the wrapper's first word is the cell, its second
/// the previous head. A null wrapper faults storing the link.
///
/// Original: thiscall, one stack word (list-head pointer), no return
/// value. Three callees: count (thiscall one arg), cell (thiscall three
/// args), allocator (thiscall no args).
lf_checker_rt::export!(thiscall, rw_00A8C950(this: u32, head: u32) -> u32 {
    unsafe {
        const ALLOC_GLOBAL: u32 = 0x12b4164;
        const COUNT_HELPER: u32 = 1;
        const CELL_HELPER: u32 = 2;
        const ALLOC: u32 = 3;
        let n: u32 = lf_checker_rt::callee_thiscall!(COUNT_HELPER, u32, this, 0);
        if (n as i32) <= 0 {
            return 0;
        }
        let mut row: u32 = 0;
        while (row as i32) < n as i32 {
            let mut col: u32 = 0;
            while col < 4 {
                let cell: u32 =
                    lf_checker_rt::callee_thiscall!(CELL_HELPER, u32, this, 0, row, col);
                if cell != 0 {
                    let scope = (lf_checker_rt::global::<u32>(ALLOC_GLOBAL) as *const u32)
                        .read_unaligned();
                    let wrap: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, scope);
                    if wrap != 0 {
                        (wrap as *mut u32).write_unaligned(cell);
                    }
                    let prev = (head as *const u32).read_unaligned();
                    ((wrap + 4) as *mut u32).write_unaligned(prev);
                    (head as *mut u32).write_unaligned(wrap);
                }
                col = col.wrapping_add(1);
            }
            row = row.wrapping_add(1);
        }
        0
    }
});
