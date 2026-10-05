// original: 0x00a8a8c0 pool_scan_table_chain (proposed)

/// Scan the table chain for the first entry matching the mask.
///
/// `this` is unused; `arg` is the match mask. Follows the chain from the
/// shared head: each link's 16-bit kind at +0x12 selects the next table
/// row (0xFFFF ends the chain), and the row index is recovered by
/// dividing the row offset by 24. For a row whose 16-bit mask at +0xE
/// shares no bit with `arg`, the veto callee runs on the row index and,
/// unless it vetoes, the accept callee runs: a true accept returns 1. Any
/// other outcome returns 0 (low byte; the upper bytes are arithmetic
/// leftovers, so the proof compares `al`). The proof uses a one-row chain
/// cycling live and end links.
///
/// Original: 0x00A8A8C0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a8a8c0(this: u32, arg: u32) -> u32 {
    unsafe {
        const CALLEE_VETO: u32 = 1;
        const CALLEE_ACCEPT: u32 = 2;
        const CHAIN_HEAD: u32 = 0x103e8dc;
        const TABLE_BASE: u32 = 0x12fb3a8;
        const CHAIN_END: u32 = 0x103e8d8;
        const MANAGER: u32 = 0x103e8d0;
        const ROW_KIND: u32 = 0x12;
        const ROW_MASK: u32 = 0xe;
        const ROW_SIZE: u32 = 24;
        const CHAIN_LAST: u16 = 0xffff;
        let _ = this;
        let head =
            lf_checker_rt::global::<u32>(CHAIN_HEAD).read_unaligned();
        if head == 0 {
            return 0;
        }
        let base =
            lf_checker_rt::global::<u32>(TABLE_BASE).read_unaligned();
        let first_kind =
            ((head + ROW_KIND) as *const u16).read_unaligned();
        let mut row = if first_kind == CHAIN_LAST {
            0
        } else {
            base.wrapping_add(
                (first_kind as u32).wrapping_mul(ROW_SIZE),
            )
        };
        let end = lf_checker_rt::global::<u32>(CHAIN_END).read_unaligned();
        if row == end {
            return 0;
        }
        let manager = lf_checker_rt::relocated(MANAGER);
        loop {
            let index = row.wrapping_sub(base) / ROW_SIZE;
            let kind =
                ((row + ROW_KIND) as *const u16).read_unaligned();
            let next = if kind == CHAIN_LAST {
                0
            } else {
                base.wrapping_add(
                    (kind as u32).wrapping_mul(ROW_SIZE),
                )
            };
            let mask =
                ((row + ROW_MASK) as *const u16).read_unaligned() as u32;
            if arg & mask == 0 {
                let veto = lf_checker_rt::callee_thiscall!(
                    CALLEE_VETO,
                    u32,
                    manager,
                    index & 0xffff
                );
                if veto as u8 == 0 {
                    let accept = lf_checker_rt::callee_thiscall!(
                        CALLEE_ACCEPT,
                        u32,
                        manager,
                        index & 0xffff
                    );
                    if accept as u8 != 0 {
                        return 1;
                    }
                }
            }
            let end =
                lf_checker_rt::global::<u32>(CHAIN_END).read_unaligned();
            row = next;
            if next == end {
                return 0;
            }
        }
    }
});
