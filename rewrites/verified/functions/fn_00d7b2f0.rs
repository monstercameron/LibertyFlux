// original: 0x00d7b2f0 find_best_cell_match
use lf_checker_rt::{callee_cdecl, callee_stdcall, export, global};

/// Best match over all cells, records and sub-entries.
///
/// Scores every candidate of an 8x8 cell window with a scripted scorer,
/// keeping the lowest score. The first slot receives the winning (record,
/// cell) pair and the second the winning sub-entry; both stay -1 when
/// nothing qualifies.
export!(cdecl, rw_d7b2f0(obj_a: u32, out1: u32, out2: u32) -> u32 {
    unsafe {
        let obj_b = *((obj_a as *const u8).add(0x20) as *const u32);
        let p0 = f32::from_bits(*((obj_b as *const u8).add(0x30) as *const u32));
        let p1 = f32::from_bits(*((obj_b as *const u8).add(0x34) as *const u32));
        let bound_a = callee_stdcall!(1, u32, p0.to_bits());
        let bound_b = callee_stdcall!(2, u32, p1.to_bits());
        let scale_a = callee_stdcall!(3, f32, bound_a);
        let diff0 = p0 - scale_a;
        let scale_b = callee_stdcall!(4, f32, bound_b);
        let diff1 = p1 - scale_b;
        let low_t = *global::<f32>(0xFE8BF4);
        let high_t = *global::<f32>(0xEEC928);
        let ba = bound_a as i32;
        let bb = bound_b as i32;
        let mut i_lo = if low_t > diff0 { ba.wrapping_sub(1) } else { ba };
        let mut j_lo = if low_t > diff1 { bb.wrapping_sub(1) } else { bb };
        let mut i_hi = if diff0 > high_t { ba.wrapping_add(1) } else { ba };
        let mut j_hi = if diff1 > high_t { bb.wrapping_add(1) } else { bb };
        if i_lo < 0 {
            i_lo = 0;
        }
        if j_lo < 0 {
            j_lo = 0;
        }
        if i_hi > 7 {
            i_hi = 7;
        }
        if j_hi > 7 {
            j_hi = 7;
        }
        let k1 = *global::<f32>(0xFE87A4);
        let k2 = *global::<f32>(0xFE8720);
        let gate = *global::<f32>(0xEE3EE8);
        let three = *global::<f32>(0xFE8A94);
        let five = *global::<f32>(0xFE8AD8);
        let one = *global::<f32>(0xFE88E8);
        let qx = f32::from_bits(*((obj_b as *const u8).add(0x30) as *const u32));
        let qy = f32::from_bits(*((obj_b as *const u8).add(0x34) as *const u32));
        let qz = f32::from_bits(*((obj_b as *const u8).add(0x38) as *const u32));
        let vx = f32::from_bits(*((obj_b as *const u8).add(0x10) as *const u32));
        let vy = f32::from_bits(*((obj_b as *const u8).add(0x14) as *const u32));
        let vz = f32::from_bits(*((obj_b as *const u8).add(0x18) as *const u32));
        let mut best = *global::<f32>(0xE9BD14);
        let mut found = 0xFFFFFFFFu32;
        let mut best_ent = 0xFFFFFFFFu32;
        let table = global::<u32>(0x1178284);
        if i_lo <= i_hi {
            let mut i = i_lo;
            loop {
                if j_lo <= j_hi {
                    let mut rowptr = table.add((i + j_lo * 8) as usize);
                    let mut cell = i + j_lo * 8;
                    let mut left = j_hi - j_lo + 1;
                    loop {
                        let row = *rowptr;
                        if row != 0 {
                            let count = *((rowptr as *const u8).add(0x300)
                                as *const i32);
                            if count > 0 {
                                let mut k = 0;
                                let mut off: usize = 0;
                                while k < count {
                                    let rec =
                                        (row as *const u8).add(off);
                                    let ox = f32::from(
                                        *((rec as *const u8).add(0x14)
                                            as *const i16),
                                    ) * k1;
                                    let oy = f32::from(
                                        *((rec as *const u8).add(0x16)
                                            as *const i16),
                                    ) * k1;
                                    let oz = f32::from(
                                        *((rec as *const u8).add(0x18)
                                            as *const i16),
                                    ) * k2;
                                    let gdx = ox - qx;
                                    let gdy = oy - qy;
                                    let gdz = oz - qz;
                                    let gd2 = (gdx * gdx + gdy * gdy) + gdz * gdz;
                                    if gd2 < gate {
                                        let n = (*((rec as *const u8)
                                            .add(0x1E))
                                            & 0xF)
                                            as i32;
                                        if n > 0 {
                                            let sw = *((rec as *const u8)
                                                .add(0x12)
                                                as *const i16)
                                                as i32;
                                            let sub = *((rowptr as *const u8)
                                                .add(0x100)
                                                as *const u32);
                                            let mut m = 0;
                                            while m < n {
                                                let idx = sw + m;
                                                let ent = *((sub as *const u8)
                                                    .add((idx * 8) as usize)
                                                    as *const u32);
                                                let row2 = *table.add(
                                                    (ent & 0xFFFF) as usize,
                                                );
                                                if row2 != 0 {
                                                    let r2 = (row2
                                                        as *const u8)
                                                        .add(
                                                            (((ent >> 16)
                                                                << 5)
                                                                as usize),
                                                        );
                                                    let r0 = f32::from(
                                                        *((r2 as *const u8)
                                                            .add(0x14)
                                                            as *const i16),
                                                    ) * k1;
                                                    let r1 = f32::from(
                                                        *((r2 as *const u8)
                                                            .add(0x16)
                                                            as *const i16),
                                                    ) * k1;
                                                    let r2v = f32::from(
                                                        *((r2 as *const u8)
                                                            .add(0x18)
                                                            as *const i16),
                                                    ) * k2;
                                                    let mut outer_t =
                                                        [ox, oy, oz * three];
                                                    let mut cand_t =
                                                        [r0, r1, r2v * three];
                                                    let mut anchor_t =
                                                        [qx, qy, qz * three];
                                                    let f: f32 =
                                                        callee_cdecl!(
                                                            5,
                                                            f32,
                                                            outer_t.as_mut_ptr()
                                                                as u32,
                                                            cand_t.as_mut_ptr()
                                                                as u32,
                                                            anchor_t.as_mut_ptr()
                                                                as u32
                                                        );
                                                    let ddx = r0 - ox;
                                                    let ddy = r1 - oy;
                                                    let ddz = r2v - oz;
                                                    let db2 = (ddy * ddy
                                                        + ddx * ddx)
                                                        + ddz * ddz;
                                                    let inv =
                                                        if db2 == 0.0 {
                                                            0.0
                                                        } else {
                                                            one / db2.sqrt()
                                                        };
                                                    let ndx = ddx * inv;
                                                    let ndy = ddy * inv;
                                                    let ndz = ddz * inv;
                                                    let dot = (vy * ndy
                                                        + vx * ndx)
                                                        + vz * ndz;
                                                    let score =
                                                        (one - dot) * five + f;
                                                    if score < best {
                                                        best = score;
                                                        found = ((k as u32)
                                                            << 16)
                                                            | (cell as u32);
                                                        best_ent = ent;
                                                    }
                                                }
                                                m += 1;
                                            }
                                        }
                                    }
                                    k += 1;
                                    off += 0x20;
                                }
                            }
                        }
                        rowptr = rowptr.add(8);
                        cell += 8;
                        left -= 1;
                        if left == 0 {
                            break;
                        }
                    }
                }
                i += 1;
                if i > i_hi {
                    break;
                }
            }
        }
        *(out1 as *mut u32) = found;
        *(out2 as *mut u32) = best_ent;
        out2
    }
});
