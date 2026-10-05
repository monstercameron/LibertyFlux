// original: 0x00A8D770 pool_notify_nodes (proposed)

/// Notify pool nodes from the iterator and the row lists.
///
/// When `this+0x73` is clear and `this+0x75` is set there is nothing to
/// do. Otherwise the row lists are always swept: each row's node chain is
/// walked and every live item whose word `+0x44` is not -1 and whose
/// flags `+0x28` select bit `0x100` is notified through its function
/// table slot `+0x44`. When both bytes are set the iterator sweep runs
/// first (cursor reset to row 0 / cell -1, every yielded non-null node
/// with word `+0x44` not -1 notified), then the row sweep follows.
///
/// Original: thiscall, no stack words, no return value. Two callee
/// shapes: the cell iterator (thiscall, one frame out-slot argument) and
/// the table slot (thiscall through the object, no stack words),
/// intercepted by a planted stub address.
lf_checker_rt::export!(thiscall, rw_00A8D770(this: u32) -> u32 {
    unsafe {
        const EN_A: u32 = 0x73;
        const EN_B: u32 = 0x75;
        const ROW_TABLE_OFF: u32 = 0xe4;
        const ROW_COUNT_OFF: u32 = 0xe8;
        const CUR_ROW_OFF: u32 = 0xfc;
        const CUR_CELL_OFF: u32 = 0x100;
        const ROW_STRIDE: u32 = 160;
        const LIST_OFF: u32 = 8;
        const ITEM_OFF: u32 = 0x34;
        const WORD_OFF: u32 = 0x44;
        const FLAGS_OFF: u32 = 0x28;
        const FLAGS_MASK: u32 = 0x3c0;
        const FLAGS_WANT: u32 = 0x100;
        const NOTIFY_SLOT: u32 = 0x44;
        const ITERATOR: u32 = 1;
        // The notify slot holds the planted stub for callee 2.
        #[inline(always)]
        unsafe fn notify(obj: u32) {
            // Load-and-call through the object exactly like the original;
            // both sides land on the planted stub for callee NOTIFY.
            unsafe {
                let slot = ((((obj as *const u32).read_unaligned()) + NOTIFY_SLOT)
                    as *const u32)
                    .read_unaligned();
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(slot as usize);
                let _ = f(obj);
            }
        }
        let a = ((this + EN_A) as *const u8).read();
        let b = ((this + EN_B) as *const u8).read();
        if a == 0 {
            if b != 0 {
                return 0;
            }
        } else if b != 0 {
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
                if node != 0 {
                    let w = ((node + WORD_OFF) as *const u16).read_unaligned();
                    if w != 0xffff {
                        notify(node);
                    }
                }
            }
        }
        let rows = ((this + ROW_COUNT_OFF) as *const u16).read_unaligned() as u32;
        if rows == 0 {
            return 0;
        }
        let table = ((this + ROW_TABLE_OFF) as *const u32).read_unaligned();
        let mut row: u32 = 0;
        while (row as i32) < rows as i32 {
            let mut link = (table
                .wrapping_add(row.wrapping_mul(ROW_STRIDE))
                .wrapping_add(LIST_OFF) as *const u32)
                .read_unaligned();
            while link != 0 {
                let item = (link as *const u32).read_unaligned();
                link = ((link + 4) as *const u32).read_unaligned();
                let inner = ((item + ITEM_OFF) as *const u32).read_unaligned();
                if inner == 0 {
                    continue;
                }
                if ((inner + WORD_OFF) as *const u16).read_unaligned() == 0xffff {
                    continue;
                }
                if ((inner + FLAGS_OFF) as *const u32).read_unaligned() & FLAGS_MASK
                    != FLAGS_WANT
                {
                    continue;
                }
                notify(inner);
            }
            row = row.wrapping_add(1);
        }
        0
    }
});
