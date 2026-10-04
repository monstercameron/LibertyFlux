// original: 0x00ad53c0 audio_blend_table_lookup
// ---------------------------------------------------------------------------
// 0x00AD53C0: blended table lookup with edge fade and optional accumulation.
// ---------------------------------------------------------------------------
// The two integer arguments are validated against a window around two anchor
// globals: each must fall strictly inside (anchor, anchor + 200), else the
// result is +0.0. A fade factor is derived from the distance to the nearest
// window edge (1.0 in the interior, (dist + 1) * 0.1 within 9 of an edge).
// Both arguments are then hashed to remainders 0..99, which index a large
// float table (stride 100); an optional output pair accumulates faded table
// differences, an optional single output receives one faded entry, and the
// return value is the faded entry at the combined index.
//
// The fourth stack argument is never read.
export!(cdecl, rw_00ad53c0(a: i32, b: i32, pair: *mut u32, _unused: u32, single: *mut u32) -> f32 {
    unsafe {
        const WINDOW: i32 = 200;
        const CELLS: i32 = 100;
        const HASH_BIAS: i32 = 100_000;
        const FADE_LIMIT: i32 = 10;

        let anchor_a = *global::<i32>(0x1552c64);
        let lo_a = a.wrapping_sub(anchor_a);
        if lo_a <= 0 {
            return 0.0;
        }
        let hi_a = anchor_a.wrapping_sub(a).wrapping_add(WINDOW);
        if hi_a <= 0 {
            return 0.0;
        }
        let anchor_b = *global::<i32>(0x1552c68);
        let lo_b = b.wrapping_sub(anchor_b);
        if lo_b <= 0 {
            return 0.0;
        }
        let hi_b = anchor_b.wrapping_sub(b).wrapping_add(WINDOW);
        if hi_b <= 0 {
            return 0.0;
        }

        let full = f32::from_bits(*global::<u32>(0xfe88e8));
        let step = f32::from_bits(*global::<u32>(0xfe879c));
        let edge = lo_a.min(hi_a).min(lo_b).min(hi_b);
        let mut scale = full;
        if edge < FADE_LIMIT {
            scale = ((edge + 1) as f32) * step;
        }

        // Hash each argument to 0..99: halve (truncated), bias, remainder.
        let r1 = (a / 2).wrapping_add(HASH_BIAS) % CELLS;
        let r2 = (b / 2).wrapping_add(HASH_BIAS) % CELLS;

        let main_table = global::<f32>(0x1552c78);
        let aux_table = global::<f32>(0x155c8b8);

        if !pair.is_null() {
            let key_a = *global::<i32>(0x1552c6c);
            let key_b = *global::<i32>(0x1552c70);
            // Note the interleave: each remainder is saved before the next divide,
            // so row_b/row_d come from r1's pair and row_s/tail from r2's pair.
            let mut row_b = (r1 + 99) % CELLS;
            let mut row_d = (r1 + 101) % CELLS;
            let mut row_s = (r2 + 99) % CELLS;
            let tail = (r2 + 101) % CELLS;
            // Exact-index matches snap rows back to the hashed values.
            if r1 == key_a {
                row_b = r1;
            }
            if r2 == key_b {
                row_s = r2;
            }
            if r1 == key_a + 99 {
                row_d = r1;
            }
            if r2 == key_b + 99 {
                row_s = r2;
            }
            let base = r1 * CELLS;
            let first = *main_table.offset((base + row_s) as isize)
                - *main_table.offset((base + tail) as isize);
            let second = *main_table.offset((row_b * CELLS + r2) as isize)
                - *main_table.offset((row_d * CELLS + r2) as isize);
            let acc0 = f32::from_bits(*pair);
            let acc1 = f32::from_bits(*pair.add(1));
            *pair.add(1) = (first * scale + acc1).to_bits();
            *pair = (second * scale + acc0).to_bits();
        }

        let combined = r1 * CELLS + r2;
        if !single.is_null() {
            *single = (*aux_table.offset(combined as isize) * scale).to_bits();
        }
        *main_table.offset(combined as isize) * scale
    }
});
