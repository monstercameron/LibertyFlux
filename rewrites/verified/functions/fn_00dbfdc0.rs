// original: 0x00dbfdc0 farthest_within_limit (proposed)

/// Farthest-record query over the fixed point table.
///
/// Same shape as its sibling `nearest_within_limit` (0x00DBFB30): the same
/// prologue (flag byte plus bit 12 / bit 1 of the flags word select the
/// low bytes the count callee sees), the same count call for
/// `(a0, a1, a2, a0m, a6m, TABLE)`, the same squared-distance order
/// `d = (dy*dy + dx*dx) + dz*dz`, the same epilogue (winning row to
/// `out_rec` unless null, index to `out_idx`, best distance in ST0).
///
/// The acceptance test differs: starting from 0, a candidate wins only
/// when strictly inside `LIMIT` (1e7) and strictly above the best so far,
/// so the result is the farthest row within range. NaN distances never
/// win. The count is compared SIGNED (`test`/`jle`, `cmp`/`jl`).
///
/// Original: 0x00DBFDC0 (stdcall, seven stack words; no register inputs).
#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn rdf(a: u32) -> f32 {
    unsafe { f32::from_bits(rd32(a)) }
}

#[inline(always)]
unsafe fn wr32(a: u32, v: u32) {
    unsafe { (a as *mut u32).write_unaligned(v) }
}

#[inline(always)]
unsafe fn wrf(a: u32, v: f32) {
    unsafe { wr32(a, v.to_bits()) }
}

#[inline(always)]
fn fsub(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}

#[inline(always)]
fn fmul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

#[inline(always)]
fn fadd(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

lf_checker_rt::export!(stdcall, rw_00dbfdc0(a0: u32, a1: u32, a2: u32, point: u32, out_rec: u32, out_idx: u32, a6: u32) -> f32 {
    unsafe {
        const TABLE: u32 = 0x17a1ed0;
        const LIMIT_AT: u32 = 0xe75900;
        const COUNT_CALLEE: u32 = 1;
        const STRIDE: u32 = 16;

        let table = lf_checker_rt::relocated(TABLE);
        let flags = rd32(a0);
        let (lo0, lo6) = if (a6 as u8) != 0 && (flags >> 12) & 1 == 1 {
            (1u32, 0u32)
        } else {
            (0u32, (flags >> 1) & 1)
        };
        let a0m = (a0 & 0xffff_ff00) | lo0;
        let a6m = (a6 & 0xffff_ff00) | lo6;
        let count =
            lf_checker_rt::callee_cdecl!(COUNT_CALLEE, u32, a0, a1, a2, a0m, a6m, table) as i32;
        let limit = rdf(lf_checker_rt::relocated(LIMIT_AT));
        let mut best = 0.0f32;
        let mut besti: i32 = -1;
        if count > 0 {
            let px = rdf(point);
            let py = rdf(point + 4);
            let pz = rdf(point + 8);
            let mut i: i32 = 0;
            while i < count {
                let rec = table.wrapping_add((i as u32).wrapping_mul(STRIDE));
                let dx = fsub(px, rdf(rec));
                let dy = fsub(py, rdf(rec + 4));
                let dz = fsub(pz, rdf(rec + 8));
                let d = fadd(fadd(fmul(dy, dy), fmul(dx, dx)), fmul(dz, dz));
                if d < limit && d > best {
                    best = d;
                    besti = i;
                }
                i += 1;
            }
        }
        if out_rec != 0 {
            let src = table.wrapping_add((besti as u32).wrapping_mul(STRIDE));
            wr32(out_rec, rd32(src));
            wrf(out_rec + 4, rdf(src + 4));
            wrf(out_rec + 8, rdf(src + 8));
            wr32(out_rec + 12, rd32(src + 12));
        }
        wr32(out_idx, besti as u32);
        best
    }
});
