// original: 0x00ad7c40 ui_grid_cell_blend
use lf_checker_rt::{callee_cdecl, export, global, relocated};

// Blend one cell of a 100x100 float grid and store the clamped result.
//
// The two integer arguments pick a cell row and column through a halved,
// biased, modulo-100 reduction; the two float arguments drive a linear
// blend against the cell's current value. The result passes through three
// clamp stages against shared limit words and is written back to the grid.
// A mode flag computed from shared state gates the whole update, and when
// the caller's flag byte is clear the inputs are also forwarded to a
// four-argument notifier. Returns the mode flag on the early path, the
// column quotient on the compute path, and the notifier's answer when it
// runs.
export!(cdecl, rw_00ad7c40(a: i32, b: i32, f1: f32, f2: f32, flag: u32) -> u32 {
    unsafe {
        ui_grid_cell_blend(a, b, f1, f2, flag)
    }
});

unsafe fn ui_grid_cell_blend(a: i32, b: i32, f1: f32, f2: f32, flag: u32) -> u32 {
    unsafe {
        // Mode flag: set unless the override word is clear while the two
        // state words agree and the stage word is not 0x12.
        let mode: u32 = if global::<u32>(0x011F7060).read() == 1 {
            1
        } else if global::<u32>(0x012088B4).read() != global::<u32>(0x00F1C040).read() {
            1
        } else if global::<u32>(0x01037720).read() == 0x12 {
            1
        } else {
            0
        };
        if mode != (flag & 0xFF) {
            return mode;
        }
        // Cell index: halve (truncate), bias, reduce mod 100, combine.
        let t1 = a / 2;
        let t1b = t1.wrapping_add(100_000);
        let r1 = t1b % 100;
        let t2 = b / 2;
        let t2b = t2.wrapping_add(100_000);
        let q2 = t2b / 100;
        let r2 = t2b % 100;
        let idx = r1.wrapping_mul(100).wrapping_add(r2);
        // Gate on the magnitude held in the companion table.
        let limit: f32 = global::<f32>(0x00FE8AB8).read();
        let mag_bits = table_word(0x01552C78, idx);
        let mag = f32::from_bits(mag_bits & 0x7FFF_FFFF);
        if !(limit > mag) {
            return q2 as u32;
        }
        // Blend: (1 - f2) * cell + f1 * f2.
        let one: f32 = global::<f32>(0x00FE88E8).read();
        let mut acc = one - f2;
        let cell = f32::from_bits(table_word(0x0155C8B8, idx));
        let prod = f1 * f2;
        acc = acc * cell;
        acc = acc + prod;
        // Clamp against the positive bound and the cell value.
        let bound: f32 = global::<f32>(0x0103F534).read();
        if !(acc > bound) {
            // within bound (or unordered): keep acc
        } else if !(bound > cell) {
            acc = cell;
        } else {
            acc = bound;
        }
        // Clamp against the negated bound.
        let neg = -bound;
        if neg > acc {
            acc = neg;
            if !(cell > acc) {
                acc = cell;
            }
        }
        // Clamp against the wide limits: below (or unordered with) the
        // low limit takes the low limit, above the high limit takes it.
        let wide_lo: f32 = global::<f32>(0x00E7E870).read();
        if !(acc > wide_lo) {
            acc = wide_lo;
        } else {
            let wide_hi: f32 = global::<f32>(0x00FE8B38).read();
            if !(wide_hi > acc) {
                acc = wide_hi;
            }
        }
        write_table_word(0x0155C8B8, idx, acc.to_bits());
        if (flag & 0xFF) != 0 {
            return q2 as u32;
        }
        callee_cdecl!(1, u32, a as u32, b as u32, f1.to_bits(), f2.to_bits())
    }
}

/// Read one word of a float table at a possibly-wild index, exactly the
/// addressing the original uses (wrapping byte offset, single load).
#[inline(always)]
unsafe fn table_word(table_va: u32, idx: i32) -> u32 {
    unsafe {
        let base = relocated(table_va);
        let addr = base.wrapping_add((idx as u32).wrapping_mul(4));
        (addr as *const u32).read()
    }
}

/// Store one word through the same addressing.
#[inline(always)]
unsafe fn write_table_word(table_va: u32, idx: i32, val: u32) {
    unsafe {
        let base = relocated(table_va);
        let addr = base.wrapping_add((idx as u32).wrapping_mul(4));
        (addr as *mut u32).write(val)
    }
}
