// original: 0x00dbfb30 nearest_within_limit (proposed)

/// Nearest-record query over the fixed point table.
///
/// Scans the first `count` rows of the 16-byte-record table at `TABLE`
/// (word 0: x float, also copied as an id; words 1-2: y, z; word 3: tag)
/// for the row nearest to `point` (three floats), among rows strictly
/// closer than `LIMIT` (1e7). Writes the winning row's 16 bytes to
/// `out_rec` unless it is null, writes the winning index to `out_idx`,
/// and returns the winning squared distance (or `LIMIT` when nothing
/// qualified) in ST0.
///
/// `a0` points at a flags word. The prologue rewrites the low bytes of the
/// incoming `a0`/`a6` slots as scratch: when `a6`'s low byte is set and bit
/// 12 of the flags word is set, the count callee sees (1, 0) there,
/// otherwise (0, bit 1 of the flags word). The count callee then answers
/// the row count for `(a0, a1, a2, a0m, a6m, TABLE)`.
///
/// The count is compared SIGNED (`test`/`jle`, `cmp`/`jl`): a negative
/// count skips the scan. Distances use the original's SSE order:
/// `d = (dy*dy + dx*dx) + dz*dz`. A candidate replaces the best only when
/// strictly closer (`comiss`/`jbe`), so NaN distances never win. Index -1
/// (no candidate) still copies the slot just before the table when
/// `out_rec` is set.
///
/// Original: 0x00DBFB30 (stdcall, seven stack words; no register inputs).
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

lf_checker_rt::export!(stdcall, rw_00dbfb30(a0: u32, a1: u32, a2: u32, point: u32, out_rec: u32, out_idx: u32, a6: u32) -> f32 {
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
        let mut best = limit;
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
                if d < best {
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
