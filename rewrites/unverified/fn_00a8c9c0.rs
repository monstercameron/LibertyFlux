// original: 0x00A8C9C0 count_matching_query_rows

/// Count valid query candidates while checking each candidate against a
/// 0.1-unit absolute-distance limit. A nonpositive signed count, a disabled
/// query object, a missing query result, a zero ready byte, or a distance over
/// the limit returns false immediately. The function returns true only after
/// processing at least one candidate without any such failure.
///
/// `this` is the query object; byte `0x76` enables the scan. The signed count
/// comes from the count helper. Rows have four cells, selected by the cell
/// helper; cell flags at `+0x28` must mask to `0x100`, and byte `+0x22b` must
/// be nonzero. The query result stores an f32 at `+0` and a ready byte at `+4`.
/// Its sign is masked before comparing to the exact f32 limit. Only AL is a
/// defined return value. Thiscall takes no stack arguments.
lf_checker_rt::export!(thiscall, rw_00a8c9c0(this: u32) -> u32 {
    unsafe {
        const CALLEE_COUNT: u32 = 1;
        const CALLEE_ROW_ENABLED: u32 = 2;
        const CALLEE_CELL: u32 = 3;
        const CALLEE_QUERY: u32 = 4;
        const ROW_ENABLED_FLAG: u32 = 0x76;
        const CELL_FLAGS: u32 = 0x28;
        const CELL_KIND: u32 = 0x22b;
        const QUERY_VALUE: u32 = 0;
        const QUERY_READY: u32 = 4;
        const CELL_FLAG_MASK: u32 = 0x3c0;
        const REQUIRED_CELL_FLAGS: u32 = 0x100;
        const ABSOLUTE_MASK: u32 = 0x7fff_ffff;
        const QUERY_LIMIT_BITS: u32 = 0x3dcc_cccd;
        const QUERY_CONTEXT: u32 = 0x0163_346c;

        let count = lf_checker_rt::callee_thiscall!(CALLEE_COUNT, u32, this, 0);
        let enabled = ((this + ROW_ENABLED_FLAG) as *const u8).read_unaligned();
        if enabled == 0 || (count as i32) <= 0 {
            return 0;
        }

        let limit = f32::from_bits(QUERY_LIMIT_BITS);
        let mut candidates = 0u32;
        let mut row = 0u32;
        while (row as i32) < (count as i32) {
            let row_enabled =
                lf_checker_rt::callee_thiscall!(CALLEE_ROW_ENABLED, u32, this, 0, row);
            if row_enabled as u8 == 0 {
                let mut column = 0u32;
                while column < 4 {
                    let cell = lf_checker_rt::callee_thiscall!(
                        CALLEE_CELL,
                        u32,
                        this,
                        0,
                        row,
                        column
                    );
                    if cell != 0 {
                        let flags = ((cell + CELL_FLAGS) as *const u32).read_unaligned();
                        let kind = ((cell + CELL_KIND) as *const u8).read_unaligned();
                        if flags & CELL_FLAG_MASK == REQUIRED_CELL_FLAGS && kind != 0 {
                            candidates = candidates.wrapping_add(1);
                            let query = lf_checker_rt::callee_thiscall!(
                                CALLEE_QUERY,
                                u32,
                                lf_checker_rt::relocated(QUERY_CONTEXT),
                                cell
                            );
                            if query == 0 {
                                return 0;
                            }
                            let ready = ((query + QUERY_READY) as *const u8).read_unaligned();
                            if ready == 0 {
                                return 0;
                            }
                            let value_bits =
                                ((query + QUERY_VALUE) as *const u32).read_unaligned();
                            let absolute = f32::from_bits(value_bits & ABSOLUTE_MASK);
                            if absolute > limit {
                                return 0;
                            }
                        }
                    }
                    column += 1;
                }
            }
            row += 1;
        }
        u32::from(candidates != 0)
    }
});
