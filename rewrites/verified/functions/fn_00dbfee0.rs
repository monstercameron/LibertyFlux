// original: 0x00dbfee0 farthest_validated (proposed)

/// Farthest-record query with per-candidate validation.
///
/// Same shape as its sibling `nearest_validated` (0x00DBFC40): the same
/// count call for `(a0, a1, a2, 0, a0m, TABLE)` with bit 1 of `[a0]` in the
/// low byte, the same SIGNED count comparison, the same squared-distance
/// order `d = (dy*dy + dx*dx) + dz*dz`, the same validation call
/// `(a3, record, VALIDATE_IMM, bit9)` answered in `al`, the same +50 mode
/// flag, and the same abort path (sets the abort flag byte, returns 0).
///
/// The acceptance test differs: starting from 0, a candidate is validated
/// only when strictly inside `LIMIT` (1e7) and strictly above the best so
/// far, so the result is the farthest validated row within range. Returns
/// 1 with the winning index and row written out, else 0 with `out_idx`
/// untouched.
///
/// Original: 0x00DBFEE0 (thiscall: object in ECX, eight stack words).
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

lf_checker_rt::export!(thiscall, rw_00dbfee0(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, point: u32, out_rec: u32, out_idx: u32, a7: u32) -> u8 {
    unsafe {
        const TABLE: u32 = 0x17a1ed0;
        const LIMIT_AT: u32 = 0xe75900;
        const FLAG_AT: u32 = 0x179fd48;
        const ABORT_AT: u32 = 0x17a337e;
        const VALIDATE_IMM: u32 = 0x1057810;
        const FIFTY_AT: u32 = 0xfe8b68;
        const COUNT_CALLEE: u32 = 1;
        const VALIDATE_CALLEE: u32 = 2;
        const STRIDE: u32 = 16;

        let table = lf_checker_rt::relocated(TABLE);
        let a0m = (a0 & 0xffff_ff00) | ((rd32(a0) >> 1) & 1);
        let count = lf_checker_rt::callee_cdecl!(COUNT_CALLEE, u32, a0, a1, a2, 0, a0m, table)
            as i32;
        let limit = rdf(lf_checker_rt::relocated(LIMIT_AT));
        let fifty = rdf(lf_checker_rt::relocated(FIFTY_AT));
        let imm = lf_checker_rt::relocated(VALIDATE_IMM);
        let bit9 = (rd32(rd32(this + 0xc18) + 0x14) >> 9) & 1;
        let mut best = 0.0f32;
        let mut besti: i32 = -1;
        if count > 0 {
            let mut i: i32 = 0;
            while i < count {
                let rec = table.wrapping_add((i as u32).wrapping_mul(STRIDE));
                let dx = fsub(rdf(point), rdf(rec));
                let dy = fsub(rdf(point + 4), rdf(rec + 4));
                let dz = fsub(rdf(point + 8), rdf(rec + 8));
                let d = fadd(fadd(fmul(dy, dy), fmul(dx, dx)), fmul(dz, dz));
                if d < limit && d > best {
                    let ans: u32 = lf_checker_rt::callee_stdcall!(
                        VALIDATE_CALLEE, u32, a3, rec, imm, bit9
                    );
                    if (ans as u8) != 0 {
                        let flag =
                            (lf_checker_rt::relocated(FLAG_AT) as *const u8).read();
                        best = if flag != 0 { fadd(d, fifty) } else { d };
                        besti = i;
                    } else if (a7 as u8) != 0 {
                        (lf_checker_rt::relocated(ABORT_AT) as *mut u8).write(1);
                        return 0;
                    }
                }
                i += 1;
            }
        }
        if besti == -1 {
            return 0;
        }
        wr32(out_idx, besti as u32);
        if out_rec != 0 {
            let src = table.wrapping_add((besti as u32).wrapping_mul(STRIDE));
            wr32(out_rec, rd32(src));
            wrf(out_rec + 4, rdf(src + 4));
            wrf(out_rec + 8, rdf(src + 8));
            wr32(out_rec + 12, rd32(src + 12));
        }
        1
    }
});
