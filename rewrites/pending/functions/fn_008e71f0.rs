// original: 0x008e71f0 PlayerSet_FindLargestCluster
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

#[inline(always)]
unsafe fn rd_u32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read() }
}

#[inline(always)]
unsafe fn rd_u8(addr: u32) -> u8 {
    unsafe { (addr as *const u8).read() }
}

#[inline(always)]
unsafe fn rd_i16(addr: u32) -> i32 {
    unsafe { (addr as *const i16).read() as i32 }
}

#[inline(always)]
unsafe fn wr_u32(addr: u32, val: u32) {
    unsafe { (addr as *mut u32).write(val) }
}

#[inline(always)]
unsafe fn wr_u16(addr: u32, val: u16) {
    unsafe { (addr as *mut u16).write(val) }
}

#[inline(always)]
unsafe fn image_f32(file_va: u32) -> f32 {
    unsafe { global::<f32>(file_va).read() }
}

#[inline(always)]
unsafe fn image_u32(file_va: u32) -> u32 {
    unsafe { global::<u32>(file_va).read() }
}

/// Scan the 32 player slots for the cluster of mutually-near players with
/// the most members, and write its average position to `out_vec`.
///
/// `player` is the reference slot (also excluded from both scans). The outer
/// scan keeps slots whose position (or the matching row of the shared table
/// when its key falls in the live window) is outside range of the reference
/// point; each survivor seeds an inner scan over all slots except itself and
/// `player`, keeping slots near the seed. The biggest cluster wins; its
/// member positions are averaged. `flag` selects which relation callee
/// answers (nonzero: the one-argument form, zero: the two-argument form).
/// Returns 1 with four floats stored, or 0 when nothing qualified.
///
/// The fourth output word is NOT computed: the original propagates whatever
/// its stack slot held (zero under the checker's defined stack fill).
export!(stdcall, rw_008e71f0(player: u32, flag: u32, out_vec: u32) -> u32 {
    unsafe {
        let first: u32 = callee_cdecl!(1, u32, player);
        if first == 0 {
            return 0;
        }
        let second: u32 = callee_cdecl!(1, u32, player);
        if rd_u32(second.wrapping_add(0x598)) == 0 {
            return 0;
        }
        let third: u32 = callee_cdecl!(1, u32, player);
        let reference = rd_u32(third.wrapping_add(0x598));
        let limit = image_u32(0x011735B4);
        let ref_x = image_f32(0x010330E0);
        let ref_y = image_f32(0x010330E4);
        let ref_z = image_f32(0x010330E8);
        let outer_thresh = image_f32(0x00FE8C74);
        let inner_r = image_f32(0x010330D8);
        let one = image_f32(0x00FE88E8);
        let table = relocated(0x01177690);
        let table_end = relocated(0x01177A90);
        let two_arg = (flag as u8) == 0;
        let this_arg = || rd_u32(reference.wrapping_add(0x224));
        let in_window = |key: u32| key != 0 && key <= limit && key.wrapping_add(0xBB8) > limit;
        let mut best_count: i32 = 0;
        let mut best = [0.0f32, 0.0f32, 0.0f32, 0.0f32];
        let mut cursor1 = table;
        let mut i: u32 = 0;
        while cursor1 < table_end {
            let mut seed_ok = false;
            let mut acc = [0.0f32; 3];
            let mut seed_obj = 0u32;
            let a: u32 = callee_cdecl!(1, u32, i);
            if a != 0 {
                let b: u32 = callee_cdecl!(1, u32, i);
                if rd_u32(b.wrapping_add(0x598)) != 0 && i != player {
                    let c: u32 = callee_cdecl!(1, u32, i);
                    let obj = rd_u32(c.wrapping_add(0x598));
                    let pos = rd_u32(obj.wrapping_add(0x20));
                    let mut px = f32::from_bits(rd_u32(pos.wrapping_add(0x30)));
                    let mut py = f32::from_bits(rd_u32(pos.wrapping_add(0x34)));
                    let mut pz = f32::from_bits(rd_u32(pos.wrapping_add(0x38)));
                    seed_obj = obj;
                    let mut from_table = false;
                    if in_window(rd_u32(cursor1)) {
                        from_table = true;
                        px = f32::from_bits(rd_u32(cursor1.wrapping_sub(0x10)));
                        py = f32::from_bits(rd_u32(cursor1.wrapping_sub(0x0C)));
                        pz = f32::from_bits(rd_u32(cursor1.wrapping_sub(8)));
                    }
                    let dx = px - ref_x;
                    let dy = py - ref_y;
                    let dz = pz - ref_z;
                    let mut dist = dy * dy;
                    dist += dx * dx;
                    dist += dz * dz;
                    if !(outer_thresh > dist)
                        && (from_table || rd_u8(obj.wrapping_add(0x211)) == 0)
                    {
                        seed_ok = true;
                        acc = [px, py, pz];
                    }
                }
            }
            if seed_ok {
                let related: u32 = if two_arg {
                    callee_thiscall!(3, u32, this_arg(), seed_obj, 1)
                } else {
                    callee_thiscall!(2, u32, this_arg(), seed_obj)
                };
                if (related as u8) != 0 {
                    let mut count = 1u32;
                    let mut cursor2 = table;
                    let mut j: u32 = 0;
                    while cursor2 < table_end {
                        let mut member = false;
                        let mut other = 0u32;
                        let a2: u32 = callee_cdecl!(1, u32, j);
                        if a2 != 0 {
                            let b2: u32 = callee_cdecl!(1, u32, j);
                            if rd_u32(b2.wrapping_add(0x598)) != 0 && j != i && j != player {
                                let c2: u32 = callee_cdecl!(1, u32, j);
                                other = rd_u32(c2.wrapping_add(0x598));
                                if in_window(rd_u32(cursor2))
                                    || rd_u8(other.wrapping_add(0x211)) == 0
                                {
                                    let rel2: u32 = if two_arg {
                                        callee_thiscall!(3, u32, this_arg(), other, 1)
                                    } else {
                                        callee_thiscall!(2, u32, this_arg(), other)
                                    };
                                    member = (rel2 as u8) != 0;
                                }
                            }
                        }
                        if member {
                            let cpos = rd_u32(other.wrapping_add(0x20));
                            let tpos = rd_u32(seed_obj.wrapping_add(0x20));
                            let cx = f32::from_bits(rd_u32(cpos.wrapping_add(0x30)));
                            let cy = f32::from_bits(rd_u32(cpos.wrapping_add(0x34)));
                            let cz = f32::from_bits(rd_u32(cpos.wrapping_add(0x38)));
                            let ddx = cx - f32::from_bits(rd_u32(tpos.wrapping_add(0x30)));
                            let ddy = cy - f32::from_bits(rd_u32(tpos.wrapping_add(0x34)));
                            let ddz = cz - f32::from_bits(rd_u32(tpos.wrapping_add(0x38)));
                            let mut dd = ddy * ddy;
                            dd += ddx * ddx;
                            dd += ddz * ddz;
                            let thresh = inner_r * inner_r;
                            if thresh > dd {
                                acc[0] = cx + acc[0];
                                acc[1] = cy + acc[1];
                                acc[2] = cz + acc[2];
                                count = count.wrapping_add(1);
                            }
                        }
                        cursor2 = cursor2.wrapping_add(0x20);
                        j = j.wrapping_add(1);
                    }
                    if (count as i32) > best_count {
                        let inv = one / (count as f32);
                        best[0] = acc[0] * inv;
                        best[1] = acc[1] * inv;
                        best[2] = acc[2] * inv;
                        best_count = count as i32;
                    }
                }
            }
            cursor1 = cursor1.wrapping_add(0x20);
            i = i.wrapping_add(1);
        }
        if best_count <= 0 {
            return 0;
        }
        wr_u32(out_vec, best[0].to_bits());
        wr_u32(out_vec.wrapping_add(4), best[1].to_bits());
        wr_u32(out_vec.wrapping_add(8), best[2].to_bits());
        wr_u32(out_vec.wrapping_add(12), best[3].to_bits());
        1
    }
});
