// original: 0x008e8590 TableScan_FindBestEntry
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

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
unsafe fn image_f32(file_va: u32) -> f32 {
    unsafe { global::<f32>(file_va).read() }
}

#[inline(always)]
unsafe fn image_u32(file_va: u32) -> u32 {
    unsafe { global::<u32>(file_va).read() }
}

/// Scan a table of 32-byte entries for the best match under a weighted
/// Manhattan distance with per-entry filters.
///
/// `handle` owns one entry array per `index` (base at `+0x804`, count at
/// `+0xB04`, both indexed by `index*4`). Each entry whose filter bytes match
/// the `use_*`/`want_*` selectors is scored against `point` (three floats);
/// the score may be adjusted by callee 1 and callee 2 (both return a float),
/// and the winner (below `*best`, optionally in `want_area` per callee 3)
/// updates `*best` and `*out_ptr` with `(entry_index << 16) | index`.
/// Returns `handle` in all cases.
///
/// Float comparison follows the original's `comiss`+`jbe` shape: a new
/// score replaces the best only when the best is strictly greater, so an
/// unordered (NaN) comparison keeps the old best.
export!(thiscall, rw_008e8590(
    handle: u32,
    out_ptr: u32,
    index: u32,
    point: u32,
    best: u32,
    use_flag_a: u32,
    use_flag_b: u32,
    bit_sel: u32,
    want_kind7: u32,
    want_kind8: u32,
    use_adjust: u32,
    w0: u32,
    w1: u32,
    dist_zw: u32,
    want_area: u32,
    use_flag_c: u32,
) -> u32 {
    unsafe {
        let slot = index.wrapping_mul(4);
        if rd_u32(handle.wrapping_add(slot).wrapping_add(0x804)) == 0 {
            return handle;
        }
        let count = rd_u32(handle.wrapping_add(slot).wrapping_add(0xB04)) as i32;
        if count <= 0 {
            return handle;
        }
        let abs_mask = image_u32(0x00FE8F80);
        let xy_scale = image_f32(0x00FE87A4);
        let z_scale = image_f32(0x00FE8720);
        let dist_scale = image_f32(0x00FE87E8);
        let px = f32::from_bits(rd_u32(point));
        let py = f32::from_bits(rd_u32(point.wrapping_add(4)));
        let pz = f32::from_bits(rd_u32(point.wrapping_add(8)));
        let zw = f32::from_bits(dist_zw);
        let mut entry_off: u32 = 0;
        let mut i: i32 = 0;
        loop {
            let base = rd_u32(handle.wrapping_add(slot).wrapping_add(0x804));
            let entry = base.wrapping_add(entry_off);
            let mut accept = true;
            if (use_flag_a as u8) != 0 && rd_u8(entry.wrapping_add(0x1E)) & 0x80 != 0 {
                accept = false;
            }
            if accept && (use_flag_b as u8) != 0 && rd_u8(entry.wrapping_add(0x1F)) & 8 != 0 {
                accept = false;
            }
            let flag = rd_u8(entry.wrapping_add(0x1F));
            if accept && ((flag >> 1) & 1) != (bit_sel as u8) {
                accept = false;
            }
            let kind = rd_u8(entry.wrapping_add(0x1C)) >> 4;
            if accept && u32::from(kind == 7) != (want_kind7 as u8) as u32 {
                accept = false;
            }
            if accept && u32::from(kind == 8) != (want_kind8 as u8) as u32 {
                accept = false;
            }
            if accept && flag & 0x80 != 0 && (use_flag_c as u8) != 0 {
                accept = false;
            }
            if accept {
                let sx = rd_i16(entry.wrapping_add(0x14)) as f32 * xy_scale;
                let sy = rd_i16(entry.wrapping_add(0x16)) as f32 * xy_scale;
                let sz = rd_i16(entry.wrapping_add(0x18)) as f32 * z_scale;
                let dx = f32::from_bits((sx - px).to_bits() & abs_mask);
                let dy = f32::from_bits((sy - py).to_bits() & abs_mask);
                let dz = f32::from_bits((sz - pz).to_bits() & abs_mask);
                let mut score = (dy + dx + dz * zw) * dist_scale;
                if (use_adjust as u8) != 0 {
                    let v: f32 = callee_thiscall!(1, f32, handle, entry, w0, w1);
                    let bias = image_f32(0x00FE88E8);
                    let k = image_f32(0x00FE8B68);
                    score -= (v - bias) * k;
                }
                let limit = f32::from_bits(rd_u32(best));
                if limit > score {
                    let v2: f32 = callee_thiscall!(2, f32, handle, entry, point);
                    let k2 = image_f32(0x00FE87D0);
                    let total = v2 * k2 + score;
                    let limit2 = f32::from_bits(rd_u32(best));
                    if limit2 > total {
                        let mut area_ok = true;
                        if (want_area as i32) >= 0 {
                            let scaled = [sx, sy, sz];
                            let area: u32 =
                                callee_cdecl!(3, u32, scaled.as_ptr() as u32);
                            if want_area != area {
                                area_ok = false;
                            }
                        }
                        if area_ok {
                            wr_u32(best, total.to_bits());
                            let combined = ((i as u32) << 16) | index;
                            wr_u32(out_ptr, combined);
                        }
                    }
                }
            }
            entry_off = entry_off.wrapping_add(0x20);
            i = i.wrapping_add(1);
            if i >= count {
                break;
            }
        }
        handle
    }
});
