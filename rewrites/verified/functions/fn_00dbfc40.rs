// original: 0x00dbfc40 nearest_validated (proposed)

/// Nearest-record query with per-candidate validation.
///
/// Scans the first `count` rows of the 16-byte-record table at `TABLE`
/// for the row nearest to `point`, as in `nearest_within_limit`, except
/// each row strictly closer than the best so far must also pass a
/// validation call. Returns 1 with the winning index in `out_idx` and the
/// winning row in `out_rec` (unless null), else 0.
///
/// `this` is an object whose `+0xc18` points at a descriptor word read at
/// `+0x14`; bit 9 of that word is passed to every validation call. `a0`
/// points at a flags word whose bit 1 the count callee sees in the low
/// byte of its fourth argument. The count call answers for
/// `(a0, a1, a2, 0, a0m, TABLE)`; the count is compared SIGNED. Each
/// validation call answers accept/reject in `al` for
/// `(a3, record, VALIDATE_IMM, bit9)`.
///
/// An accepted distance grows by 50 when the mode flag byte is set. A
/// rejected candidate either aborts the whole search (setting the abort
/// flag byte and returning 0, without writing `out_idx`) when `a7`'s low
/// byte is set, or is skipped. A scan with no accepted row returns 0 and
/// also leaves `out_idx` untouched.
///
/// Original: 0x00DBFC40 (thiscall: object in ECX, eight stack words).
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

lf_checker_rt::export!(thiscall, rw_00dbfc40(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, point: u32, out_rec: u32, out_idx: u32, a7: u32) -> u8 {
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
        let mut best = limit;
        let mut besti: i32 = -1;
        if count > 0 {
            let mut i: i32 = 0;
            while i < count {
                let rec = table.wrapping_add((i as u32).wrapping_mul(STRIDE));
                let dx = fsub(rdf(point), rdf(rec));
                let dy = fsub(rdf(point + 4), rdf(rec + 4));
                let dz = fsub(rdf(point + 8), rdf(rec + 8));
                let d = fadd(fadd(fmul(dy, dy), fmul(dx, dx)), fmul(dz, dz));
                if d < best {
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
