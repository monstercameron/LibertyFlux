// original: 0x008ED550 fill_node_rect_global (proposed)

/// Fill a rectangle of the global node grid after quantising a float span.
///
/// `pair` points to two floats (low, high); `half` is a half-width. The
/// indexer (callee 1, thiscall on `obj`) quantises `low - half`, `low +
/// half`, `high - half` and `high + half` (each subtraction/addition in
/// that operand order). The four answers are clamped, all comparisons
/// signed: `r1` below 0 becomes 0 (first row), `r2` above 7 becomes 7
/// (last row), `r3` below 0 becomes 0 (first column), `r4` above 7 becomes
/// 7 (last column). When the first row is above the last row (signed)
/// nothing is filled. Otherwise every cell of rows `r1..=r2` and columns
/// `r3..=r4` of the 8-wide global grid at `GRID` is set to 1; a row with
/// `r3 > r4` (signed) contributes no cells. Returns the clamped `r4`.
///
/// The original keeps the clamped first row and last column in its own
/// incoming argument slots; the contract switches the stack check off and
/// observes those values through the return value, the filled grid bytes
/// and the call arguments instead.
///
/// Original: 0x008ED550 (thiscall, two stack arguments).
lf_checker_rt::export!(thiscall, rw_008ED550(obj: u32, pair: u32, half: u32) -> u32 {
    unsafe {
        /// Global node grid (8 columns, rows 8 bytes apart).
        const GRID: u32 = 0x1176DB8;
        /// Highest valid row/column index.
        const MAX_IDX: u32 = 7;
        /// Row stride in bytes.
        const ROW_STRIDE: u32 = 8;
        /// Fill byte.
        const FILL: u8 = 1;
        /// Indexer callee id.
        const INDEX: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let lo = f32::from_bits(rd32(pair));
        let hi = f32::from_bits(rd32(pair.wrapping_add(4)));
        let h = f32::from_bits(half);
        let r1: u32 =
            lf_checker_rt::callee_thiscall!(INDEX, u32, obj, fsub(lo, h).to_bits());
        let r2: u32 =
            lf_checker_rt::callee_thiscall!(INDEX, u32, obj, fadd(lo, h).to_bits());
        let r1c = if (r1 as i32) < 0 { 0 } else { r1 };
        let r3: u32 =
            lf_checker_rt::callee_thiscall!(INDEX, u32, obj, fsub(hi, h).to_bits());
        let r4: u32 =
            lf_checker_rt::callee_thiscall!(INDEX, u32, obj, fadd(hi, h).to_bits());
        let r2c = if (r2 as i32) > (MAX_IDX as i32) {
            MAX_IDX
        } else {
            r2
        };
        let r3c = if (r3 as i32) < 0 { 0 } else { r3 };
        let r4c = if (r4 as i32) > (MAX_IDX as i32) {
            MAX_IDX
        } else {
            r4
        };
        if (r1c as i32) > (r2c as i32) {
            return r4c;
        }
        let grid = lf_checker_rt::relocated(GRID);
        let mut row = r1c as i32;
        while row <= (r2c as i32) {
            if (r3c as i32) <= (r4c as i32) {
                let n = (r4c as i32 - r3c as i32 + 1) as u32;
                let mut p = grid
                    .wrapping_add((row as u32).wrapping_mul(ROW_STRIDE))
                    .wrapping_add(r3c);
                let mut k: u32 = 0;
                while k < n {
                    wr8(p, FILL);
                    p = p.wrapping_add(1);
                    k = k.wrapping_add(1);
                }
            }
            row = row.wrapping_add(1);
        }
        r4c
    }
});
