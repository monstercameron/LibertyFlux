//! Differential cases, part 8 (second lane): row tables.
//!
//! Each case plants the row blocks, their cell arrays and the cursor
//! words, runs the rewrite and the lifted method on the same inputs, and
//! compares answers and every effect. Each method has a deliberately
//! wrong lift that must be caught. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_pooldiff::rewrites::*;
    use lf_world::pools::{Row, RowTable};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{Rng, addr, lock, put_u32};

    /// Row-block stride of the 32-bit row tables.
    const STRIDE: usize = 160;

    /// Plants the rows and the object holding the table pointer, the
    /// enable byte and the count/cursor words. Returns (object, lift).
    /// Boxes are leaked so their addresses stay valid.
    fn plant_rows(counts: &[usize], fill: &mut Rng) -> (u32, RowTable) {
        let mut rows = Vec::with_capacity(counts.len());
        let mut obj = vec![0u8; 0x104];
        fill.bytes(&mut obj);
        let mut blocks = vec![0u8; counts.len() * STRIDE];
        for (r, &n) in counts.iter().enumerate() {
            let mut cells = Vec::with_capacity(n);
            for _ in 0..n {
                cells.push(fill.u32());
            }
            let mut arr = vec![0u8; n * 4];
            for (i, c) in cells.iter().enumerate() {
                put_u32(&mut arr, i * 4, *c);
            }
            let arr_box = arr.into_boxed_slice();
            let arr_addr = if n == 0 { 0 } else { addr(&arr_box[0]) };
            // Empty rows still need a stable (unused) address: the
            // iterator never reads through it, but the block word must
            // hold something deterministic.
            let arr_addr = if n == 0 {
                let z = Box::leak(Box::new(0u32));
                addr(z)
            } else {
                arr_addr
            };
            put_u32(&mut blocks, r * STRIDE, arr_addr);
            blocks[r * STRIDE + 4..r * STRIDE + 6].copy_from_slice(&(n as u16).to_le_bytes());
            rows.push(Row { cells });
            if n != 0 {
                std::mem::forget(arr_box);
            }
        }
        let blocks_box = blocks.into_boxed_slice();
        // An empty table has no blocks; the rewrite returns before
        // dereferencing, so a dummy address will do.
        let blocks_addr = if blocks_box.is_empty() {
            addr(Box::leak(Box::new(0u32)))
        } else {
            addr(&blocks_box[0])
        };
        put_u32(&mut obj, 0xe4, blocks_addr);
        // The row count is the table length.
        obj[0xe8..0xe8 + 2].copy_from_slice(&(counts.len() as u16).to_le_bytes());
        let obj_box = obj.into_boxed_slice();
        let this = addr(&obj_box[0]);
        std::mem::forget(blocks_box);
        std::mem::forget(obj_box);
        (this, RowTable { rows })
    }

    unsafe fn write_byte(this: u32, off: u32, v: u8) {
        unsafe {
            ((this + off) as *mut u8).write(v);
        }
    }

    unsafe fn write_cursor(this: u32, row: i32, cell: u32) {
        unsafe {
            ((this + 0xfc) as *mut i32).write_unaligned(row);
            ((this + 0x100) as *mut u32).write_unaligned(cell);
        }
    }

    unsafe fn read_cursor(this: u32) -> (i32, u32) {
        unsafe {
            (
                ((this + 0xfc) as *const i32).read_unaligned(),
                ((this + 0x100) as *const u32).read_unaligned(),
            )
        }
    }

    #[test]
    fn row_count_guarded_matches() {
        let _guard = lock();
        let mut rng = Rng(0x70C0);
        let mut cases = 0;
        let mut caught = 0;
        for _ in 0..8 {
            let nrows = 1 + (rng.below(4) as usize);
            let mut counts = Vec::new();
            for _ in 0..nrows {
                counts.push(rng.below(5) as usize);
            }
            let (this, table) = plant_rows(&counts, &mut rng);
            for row in 0..nrows {
                for &enabled in &[true, false] {
                    unsafe { write_byte(this, 0x73, u8::from(enabled)) };
                    let got = unsafe { fn_00A8EA70::rw_00A8EA70(this, row as u32) };
                    let want = table.count_guarded(enabled, row as u32);
                    assert_eq!(got, u32::from(want), "row {row} enabled={enabled}");
                    // Wrong lift: ignores the enable byte.
                    if table.count_guarded(true, row as u32) != want {
                        caught += 1;
                    }
                    cases += 1;
                }
            }
        }
        assert!(cases > 8, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong guard never caught ({cases} cases)");
    }

    #[test]
    fn row_cell_matches() {
        let _guard = lock();
        let mut rng = Rng(0x70CE);
        let mut cases = 0;
        let mut caught = 0;
        for _ in 0..8 {
            let nrows = 1 + (rng.below(4) as usize);
            let mut counts = Vec::new();
            for _ in 0..nrows {
                counts.push(1 + (rng.below(4) as usize));
            }
            let (this, table) = plant_rows(&counts, &mut rng);
            for (r, &n) in counts.iter().enumerate() {
                for c in 0..n {
                    let got = unsafe { fn_00A8EAF0::rw_00A8EAF0(this, r as u32, c as u32) };
                    assert_eq!(got, table.cell(r as u32, c as u32), "cell ({r}, {c})");
                    // Wrong lift: the next cell over (wraps within the row).
                    let w = table.cell(r as u32, ((c + 1) % n) as u32);
                    if w != got {
                        caught += 1;
                    }
                    cases += 1;
                }
            }
        }
        assert!(cases > 8, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong cell never caught ({cases} cases)");
    }

    /// Wrong cursor: advances the cell but never the row, ending the
    /// walk at the row's limit instead of stepping over.
    struct WrongCursor {
        row: i32,
        cell: u32,
    }

    /// One wrong step from the walker's start state.
    fn wrong_step(table: &RowTable, w: &mut WrongCursor, out: &mut u32) -> bool {
        let rows = table.len() as i32;
        if w.row >= rows {
            return false;
        }
        let cell = w.cell.wrapping_add(1);
        w.cell = cell;
        let cells = &table.rows[w.row as usize].cells;
        if (cell as i32) < cells.len() as i32 {
            *out = cells[cell as usize];
            true
        } else {
            false
        }
    }

    /// Walks the whole table from (0, -1) on all three sides (rewrite,
    /// lift, wrong lift), comparing every step's answer, output and
    /// cursor. Returns (steps, caught).
    fn walk_all(this: u32, table: &RowTable) -> (u32, u32) {
        let mut steps = 0;
        let mut caught = 0;
        let mut cursor = table.cursor(0, u32::MAX);
        let mut wrong = WrongCursor {
            row: 0,
            cell: u32::MAX,
        };
        unsafe { write_cursor(this, 0, u32::MAX) };
        loop {
            let out_box = Box::leak(Box::new(0xA5A5_A5A5u32));
            let out_addr = addr(out_box);
            let mut lout = 0xA5A5_A5A5u32;
            let mut wout = 0xA5A5_A5A5u32;
            let got = unsafe { fn_00A8E9B0::rw_00A8E9B0(this, out_addr) };
            let want = cursor.step(&mut lout);
            let wans = wrong_step(table, &mut wrong, &mut wout);
            assert_eq!(got, u32::from(want), "step {steps} answer");
            let (row, cell) = unsafe { read_cursor(this) };
            assert_eq!(row, cursor.row, "step {steps} row");
            assert_eq!(cell, cursor.cell, "step {steps} cell");
            let rob = unsafe { (out_addr as *const u32).read_unaligned() };
            assert_eq!(rob, lout, "step {steps} out");
            if wans != want || (wans && wout != rob) {
                caught += 1;
            }
            steps += 1;
            if !want {
                assert_eq!(rob, 0xA5A5_A5A5, "miss leaves the output alone");
                break;
            }
            assert!(steps < 100, "walk diverged");
        }
        (steps, caught)
    }

    #[test]
    fn cell_iterator_matches() {
        let _guard = lock();
        let mut rng = Rng(0x71E0);
        let mut cases = 0;
        let mut caught = 0;
        // Targeted shapes: empty table, empty rows, singletons, ragged.
        let shapes: Vec<Vec<usize>> = vec![
            vec![],
            vec![0],
            vec![1],
            vec![0, 0, 0],
            vec![2, 0, 1],
            vec![1, 1, 1, 1],
            vec![3, 2],
        ];
        for counts in &shapes {
            let (this, table) = plant_rows(counts, &mut rng);
            let (steps, c) = walk_all(this, &table);
            cases += steps;
            caught += c;
        }
        for _ in 0..6 {
            let nrows = rng.below(5) as usize;
            let mut counts = Vec::new();
            for _ in 0..nrows {
                counts.push(rng.below(4) as usize);
            }
            let (this, table) = plant_rows(&counts, &mut rng);
            let (steps, c) = walk_all(this, &table);
            cases += steps;
            caught += c;
        }
        // Mid-walk starts: (row, last cell) must advance, (last, last-1)
        // must end.
        {
            let (this, table) = plant_rows(&[2, 3], &mut rng);
            let mut cursor = table.cursor(0, 1);
            unsafe { write_cursor(this, 0, 1) };
            let out_box = Box::leak(Box::new(0u32));
            let out_addr = addr(out_box);
            let mut lout = 0u32;
            let got = unsafe { fn_00A8E9B0::rw_00A8E9B0(this, out_addr) };
            let want = cursor.step(&mut lout);
            assert_eq!((got, cursor.row, cursor.cell), (1, 1, 0));
            assert!(want);
            let rob = unsafe { (out_addr as *const u32).read_unaligned() };
            assert_eq!((rob, lout), (table.cell(1, 0), table.cell(1, 0)));
            cases += 1;
        }
        assert!(cases > 8, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong iterator never caught ({cases} cases)");
    }
}
