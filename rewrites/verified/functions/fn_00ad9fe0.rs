// original: 0x00ad9fe0 ui_proximity_flag_sweep (proposed)

/// Flag entries whose lookup points sit far apart, then spread the flag to
/// overlapping neighbours until nothing changes.
///
/// State (all globals): a signed count at `COUNT`, `count` entries of 16
/// bytes at `ENTRIES` (`+0`: first table index, `+6`: second table index,
/// both signed 16-bit; `+8`: 16-bit flags, bit `FAR` is the flag this
/// function maintains), and a table at `TABLE` of (x, y) signed-16-bit pairs
/// laid out 8 bytes apart (x at `index*8+0`, y at `index*8+2`).
///
/// Phase 1 clears or sets `FAR` on every entry: the flag is set when the two
/// table points differ by `LIMIT` (150) or more in x or in y, cleared
/// otherwise. Phase 2 repeats a sweep until a full pass sets nothing: an
/// unflagged entry gains the flag when its box (first point to second point)
/// overlaps the box of any flagged entry, comparing x and y ends with signed
/// `<=`. A non-positive count runs neither phase. The original reloads the
/// count from memory inside the loops; it never changes, so it is read once.
///
/// Original: 0x00ad9fe0 (cdecl, no arguments, no return value, no calls).
lf_checker_rt::export!(cdecl, rw_00ad9fe0() -> u32 {
    unsafe {
        const COUNT: u32 = 0x0155_0eac;
        const ENTRIES: u32 = 0x0155_0eb4;
        const TABLE: u32 = 0x0158_e860;
        const STRIDE: u32 = 16;
        const FAR: u16 = 0x0008;
        const LIMIT: i32 = 0x96;

        #[inline(always)]
        unsafe fn rd16(a: u32) -> i32 {
            unsafe { (a as *const i16).read_unaligned() as i32 }
        }
        /// Table point coordinate: `which` 0 for x, 2 for y.
        #[inline(always)]
        unsafe fn coord(table: u32, idx: i32, which: u32) -> i32 {
            unsafe { rd16(table.wrapping_add((idx.wrapping_mul(8)) as u32).wrapping_add(which)) }
        }

        let entries = lf_checker_rt::relocated(ENTRIES);
        let table = lf_checker_rt::relocated(TABLE);
        let n = (lf_checker_rt::global::<i32>(COUNT) as *const i32).read_unaligned();

        if n > 0 {
            // Phase 1: set or clear FAR from the point separation.
            let mut e = entries;
            for _ in 0..n {
                let a = rd16(e);
                let b = rd16(e.wrapping_add(6));
                let dx = (coord(table, a, 0) - coord(table, b, 0)).abs();
                let flags = e.wrapping_add(8) as *mut u16;
                if dx >= LIMIT {
                    flags.write_unaligned(flags.read_unaligned() | FAR);
                } else {
                    let dy = (coord(table, a, 2) - coord(table, b, 2)).abs();
                    if dy >= LIMIT {
                        flags.write_unaligned(flags.read_unaligned() | FAR);
                    } else {
                        flags.write_unaligned(flags.read_unaligned() & !FAR);
                    }
                }
                e = e.wrapping_add(STRIDE);
            }
            // Phase 2: spread FAR across overlapping boxes to a fixpoint.
            loop {
                let mut changed = false;
                let mut j = 0i32;
                while j < n {
                    let ej = entries.wrapping_add((j as u32).wrapping_mul(STRIDE));
                    if (ej.wrapping_add(8) as *const u16).read_unaligned() & FAR == 0 {
                        let aj = rd16(ej);
                        let bj = rd16(ej.wrapping_add(6));
                        let x_lo = coord(table, aj, 0);
                        let x_hi = coord(table, bj, 0);
                        let y_lo = coord(table, aj, 2);
                        let y_hi = coord(table, bj, 2);
                        let mut i = 0i32;
                        while i < n {
                            if i != j {
                                let ei = entries.wrapping_add((i as u32).wrapping_mul(STRIDE));
                                if (ei.wrapping_add(8) as *const u16).read_unaligned() & FAR != 0 {
                                    let ai = rd16(ei);
                                    let bi = rd16(ei.wrapping_add(6));
                                    if x_hi >= coord(table, ai, 0)
                                        && x_lo <= coord(table, bi, 0)
                                        && y_hi >= coord(table, ai, 2)
                                        && y_lo <= coord(table, bi, 2)
                                    {
                                        let f = ej.wrapping_add(8) as *mut u16;
                                        f.write_unaligned(f.read_unaligned() | FAR);
                                        changed = true;
                                    }
                                }
                            }
                            i += 1;
                        }
                    }
                    j += 1;
                }
                if !changed {
                    break;
                }
            }
        }
        0
    }
});
