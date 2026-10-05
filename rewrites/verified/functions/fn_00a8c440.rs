// original: 0x00A8C440 pool_sweep_matching_into_list (proposed)

/// Sweep the pool twice, prepending every matching cell to `*head`.
///
/// Does nothing when the enable byte at `this+0x73` is clear. First sweep:
/// the cell iterator runs (cursor reset to row 0 / cell -1) and every
/// yielded non-null node whose flags `+0x28` select exactly bit `0x40` is
/// wrapped by the allocator and prepended. Second sweep: the count helper
/// (with 0) bounds the rows (non-positive counts skip); for each row and
/// each of four columns the cell helper runs and non-null cells are
/// wrapped and prepended the same way. The allocator's object is the
/// global pointer; a null wrapper faults storing the link.
///
/// Original: thiscall, one stack word (list-head pointer), no return
/// value. Four callees: iterator (thiscall, one frame out-slot), count
/// (thiscall one arg), cell (thiscall three args), allocator (thiscall
/// no args).
lf_checker_rt::export!(thiscall, rw_00A8C440(this: u32, head: u32) -> u32 {
    unsafe {
        const EN: u32 = 0x73;
        const CUR_ROW_OFF: u32 = 0xfc;
        const CUR_CELL_OFF: u32 = 0x100;
        const FLAGS_OFF: u32 = 0x28;
        const FLAGS_MASK: u32 = 0x3c0;
        const FLAGS_WANT: u32 = 0x40;
        const ALLOC_GLOBAL: u32 = 0x12b4164;
        const ITERATOR: u32 = 1;
        const COUNT_HELPER: u32 = 2;
        const CELL_HELPER: u32 = 3;
        const ALLOC: u32 = 4;
        #[inline(always)]
        unsafe fn prepend(head: u32, cell: u32) {
            unsafe {
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
        }
        if ((this + EN) as *const u8).read() == 0 {
            return 0;
        }
        ((this + CUR_ROW_OFF) as *mut u32).write_unaligned(0);
        ((this + CUR_CELL_OFF) as *mut u32).write_unaligned(0xffffffff);
        let mut out: u32 = 0;
        loop {
            let more: u32 = lf_checker_rt::callee_thiscall!(
                ITERATOR,
                u32,
                this,
                &mut out as *mut u32 as u32
            );
            if more as u8 == 0 {
                break;
            }
            let node = out;
            if node == 0 {
                continue;
            }
            if ((node + FLAGS_OFF) as *const u32).read_unaligned() & FLAGS_MASK
                != FLAGS_WANT
            {
                continue;
            }
            prepend(head, node);
        }
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
                    prepend(head, cell);
                }
                col = col.wrapping_add(1);
            }
            row = row.wrapping_add(1);
        }
        0
    }
});
