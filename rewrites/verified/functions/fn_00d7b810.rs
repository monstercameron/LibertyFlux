// original: 0x00d7b810 ui_joint_angle_test
use lf_checker_rt::{export, global};

/// Angle test between three indexed polyline joints (original 0x00D7B810).
///
/// Each of the three packed arguments holds a table index in its low word and
/// a row selector in its high word. The function looks up a base pointer per
/// index in a global table, reads one 2D joint (a pair of i16 values scaled by
/// a global factor) out of each row, and measures the angle between the two
/// segments joint0->joint1 and joint1->joint2 via normalized dot and cross
/// products.
///
/// Returns 1 when the dot product exceeds `threshold`, otherwise stores a
/// below-threshold flag through `out_flag` and returns 2 or 4 from the sign of
/// the cross product (always storing the cross product through `out_cross`).
/// Returns 0 when any index is 0xFFFF, any table entry is null, or either
/// segment has zero length.
export!(cdecl, rw_d7b810(
    a1: u32,
    a2: u32,
    a3: u32,
    out_flag: *mut u8,
    out_cross: *mut f32,
    threshold: f32,
) -> u32 {
    unsafe {
        out_flag.write(0);
        let lo1 = a1 & 0xFFFF;
        if lo1 == 0xFFFF {
            return 0;
        }
        let lo2 = a2 & 0xFFFF;
        if lo2 == 0xFFFF {
            return 0;
        }
        let lo3 = a3 & 0xFFFF;
        if lo3 == 0xFFFF {
            return 0;
        }
        let table = global::<u32>(0x01178284);
        let base1 = table.add(lo1 as usize).read();
        if base1 == 0 {
            return 0;
        }
        let base2 = table.add(lo2 as usize).read();
        if base2 == 0 {
            return 0;
        }
        let base3 = table.add(lo3 as usize).read();
        if base3 == 0 {
            return 0;
        }
        let scale = global::<f32>(0x00FE87A4).read();
        let r0 = base1.wrapping_add((a1 >> 16) << 5);
        let r1 = base2.wrapping_add((a2 >> 16) << 5);
        let r2 = base3.wrapping_add((a3 >> 16) << 5);
        let x0 = (r0.wrapping_add(0x14) as *const i16).read_unaligned() as f32 * scale;
        let y0 = (r0.wrapping_add(0x16) as *const i16).read_unaligned() as f32 * scale;
        let x1 = (r1.wrapping_add(0x14) as *const i16).read_unaligned() as f32 * scale;
        let y1 = (r1.wrapping_add(0x16) as *const i16).read_unaligned() as f32 * scale;
        let x2 = (r2.wrapping_add(0x14) as *const i16).read_unaligned() as f32 * scale;
        let y2 = (r2.wrapping_add(0x16) as *const i16).read_unaligned() as f32 * scale;
        let d0 = y1 - y0;
        let d1 = x1 - x0;
        let d2 = y2 - y1;
        let d3 = x2 - x1;
        let len1 = (d0 * d0 + d1 * d1).sqrt();
        if len1 == 0.0 {
            return 0;
        }
        let inv1 = 1.0 / len1;
        let nx1 = inv1 * d1;
        let ny1 = inv1 * d0;
        let len2 = (d2 * d2 + d3 * d3).sqrt();
        if len2 == 0.0 {
            return 0;
        }
        let inv2 = 1.0 / len2;
        let a = inv2 * d2;
        let b = inv2 * d3;
        let dot = a * ny1 + b * nx1;
        let cross = a * nx1 - b * ny1;
        out_cross.write(cross);
        if dot > threshold {
            return 1;
        }
        let limit = global::<f32>(0x00FE8D74).read();
        out_flag.write(if limit > dot { 1 } else { 0 });
        if cross > 0.0 { 4 } else { 2 }
    }
});
