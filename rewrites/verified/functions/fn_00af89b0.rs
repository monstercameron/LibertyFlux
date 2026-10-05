// original: 0x00AF89B0 veh_bounds_store (proposed)

/// Normalise a coordinate box and append it to the live box table.
///
/// Takes two opposite corners `(x0, y0, z0)` and `(x1, y1, z1)` plus a table
/// selector `which` (0..3). Each axis pair is ordered low-first (an unordered
/// NaN pair keeps its incoming order, matching the original's ordered-compare
/// swap); a selector above 3, or a table that already holds 10 boxes, stores
/// nothing. Otherwise the six bounds land in the next 32-byte record of the
/// selected table, the two padding words are zero (the original copies two
/// words of its own uninitialised stack scratch here; the contract fills
/// scratch with zero so both sides observe zero), and the table count grows
/// by one. Nothing is returned.
///
/// Original: 0x00AF89B0 (cdecl, seven stack arguments).
lf_checker_rt::export!(cdecl, rw_00AF89B0(x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32, which: u32) -> u32 {
    unsafe {
        const COUNTS: u32 = 0x15FFBC4;
        const TABLE: u32 = 0x15FFC40;
        const TABLES: u32 = 4;
        const SLOTS: u32 = 10;
        const RECORD: u32 = 32;
        #[inline(always)]
        fn order(a: f32, b: f32) -> (f32, f32) {
            if a > b { (b, a) } else { (a, b) }
        }
        let (lox, hix) = order(x0, x1);
        let (loy, hiy) = order(y0, y1);
        let (loz, hiz) = order(z0, z1);
        if which >= TABLES {
            return 0;
        }
        let counts = lf_checker_rt::relocated(COUNTS);
        let n = ((counts + which * 4) as *const u32).read_unaligned();
        if n >= SLOTS {
            return 0;
        }
        let slot = lf_checker_rt::relocated(TABLE).wrapping_add((n + which * SLOTS) * RECORD);
        ((slot + 0x00) as *mut u32).write_unaligned(lox.to_bits());
        ((slot + 0x04) as *mut u32).write_unaligned(loy.to_bits());
        ((slot + 0x08) as *mut u32).write_unaligned(loz.to_bits());
        ((slot + 0x0C) as *mut u32).write_unaligned(0);
        ((slot + 0x10) as *mut u32).write_unaligned(hix.to_bits());
        ((slot + 0x14) as *mut u32).write_unaligned(hiy.to_bits());
        ((slot + 0x18) as *mut u32).write_unaligned(hiz.to_bits());
        ((slot + 0x1C) as *mut u32).write_unaligned(0);
        ((counts + which * 4) as *mut u32).write_unaligned(n + 1);
        0
    }
});
