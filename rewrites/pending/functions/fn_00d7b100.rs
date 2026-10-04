// original: 0x00d7b100 slope_between_grid_records
use lf_checker_rt::{export, global};

/// Slope between two records selected through a global row table.
///
/// The object carries two packed selectors at +0xDE0/+0xDE4 (low word: row
/// index into the table at 0x1178284, high word: record index within the
/// row, 32 bytes each). A 0xFFFF row index or a null row yields 0.0.
/// Otherwise two coordinates are scaled by 0.125, their differences give a
/// distance, the third coordinate is scaled by 0.015625, and the result is
/// the scaled third-coordinate difference divided by the distance.
export!(cdecl, rw_d7b100(obj: u32) -> f32 {
    unsafe {
        let pair0 = *((obj as *const u8).add(0xDE0) as *const u32);
        let pair1 = *((obj as *const u8).add(0xDE4) as *const u32);
        if pair0 & 0xFFFF == 0xFFFF || pair1 & 0xFFFF == 0xFFFF {
            return 0.0;
        }
        let table = global::<u32>(0x1178284);
        let row0 = *table.add((pair0 & 0xFFFF) as usize);
        if row0 == 0 {
            return 0.0;
        }
        let row1 = *table.add((pair1 & 0xFFFF) as usize);
        if row1 == 0 {
            return 0.0;
        }
        let k1 = *global::<f32>(0xFE87A4);
        let k2 = *global::<f32>(0xFE8720);
        let rec0 = (row0 as *const u8).add(((pair0 >> 16) << 5) as usize);
        let rec1 = (row1 as *const u8).add(((pair1 >> 16) << 5) as usize);
        let a0 = f32::from(*((rec0 as *const u8).add(0x14) as *const i16));
        let a1 = f32::from(*((rec0 as *const u8).add(0x16) as *const i16));
        let a2 = f32::from(*((rec0 as *const u8).add(0x18) as *const i16));
        let b0 = f32::from(*((rec1 as *const u8).add(0x14) as *const i16));
        let b1 = f32::from(*((rec1 as *const u8).add(0x16) as *const i16));
        let b2 = f32::from(*((rec1 as *const u8).add(0x18) as *const i16));
        let a1s = a1 * k1;
        let a0s = a0 * k1;
        let b1s = b1 * k1;
        let b0s = b0 * k1;
        let dy = a1s - b1s;
        let dx = a0s - b0s;
        let dist = (dy * dy + dx * dx).sqrt();
        let b2s = b2 * k2;
        let a2s = a2 * k2;
        (b2s - a2s) / dist
    }
});
