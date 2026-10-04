// original: 0x00b034e0 cull_bank_against_rows (proposed)

/// Test every item of a bank against eight rows of three floats, reporting
/// whether any slot holds no passing item.
///
/// `this` points to an object whose first word is the bank table. `bank`
/// selects a bank (`BANK_STRIDE` bytes each); the dword at `COUNT_OFF` past
/// the bank base is the slot count. The first stack word is not read. `pa`
/// and `pc` point at two 3-float corners; `pdi`, when non-null, points at a
/// 15-float (60-byte) transform.
/// Rows start as the eight corners of the box spanned by the two corners:
/// row 0 is (a0, a1, a2), row 1 (a0, a1, c2), row 2 (a0, c1, a2), row 3
/// (a0, c1, c2), row 4 (c0, c1, c2), row 5 (c0, c1, a2), row 6 (c0, a1, c2),
/// row 7 (c0, a1, a2). With a transform, each row becomes an affine mix of
/// the corners and the transform entries, in the exact operation order below.
/// For each slot (flag at `FLAG_OFF`, item count at the slot word, items of
/// four floats ending at the slot word): a non-zero flag skips the slot; a
/// zero count returns 1 at once; otherwise each item scores
/// row.y*v1 + row.x*v0 + row.z*v2 - v3 against every row, and an item whose
/// eight scores are all strictly above zero moves on to the next slot. A
/// slot whose items all fail to do that returns 1. All slots passed, or a
/// non-positive count, returns 0. NaN scores fail like any other non-positive
/// value; a zero count returns before the corners are touched.
///
/// Original: 0x00b034e0 (thiscall, this in ecx, five stack words).
lf_checker_rt::export!(thiscall, rw_00b034e0(this: u32, _unused: u32, bank: u32, pa: u32, pc: u32, pdi: u32) -> u32 {
    unsafe {
        const BANK_STRIDE: u32 = 0x5580;
        const COUNT_OFF: u32 = 0x4000;
        const SLOT_FIRST: u32 = 0xec;
        const SLOT_STRIDE: u32 = 0x100;
        const FLAG_BACK: u32 = 0x0c;
        const ITEMS_BACK: u32 = 0xec;
        const ITEM_STRIDE: u32 = 0x10;
        const N_ROWS: u32 = 8;
        const COOKIE_CALLEE: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn cookie() {
            unsafe { lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,) };
        }

        let base = rd32(this);
        let sbase = bank.wrapping_mul(BANK_STRIDE).wrapping_add(base);
        let count = rd32(sbase.wrapping_add(COUNT_OFF));
        if count == 0 {
            cookie();
            return 0;
        }
        let (a0, a1, a2) = (rdf(pa), rdf(pa + 4), rdf(pa + 8));
        let (c0, c1, c2) = (rdf(pc), rdf(pc + 4), rdf(pc + 8));
        let mut rows = [
            [a0, a1, a2],
            [a0, a1, c2],
            [a0, c1, a2],
            [a0, c1, c2],
            [c0, c1, c2],
            [c0, c1, a2],
            [c0, a1, c2],
            [c0, a1, a2],
        ];
        if pdi != 0 {
            let mut e = [0.0f32; 15];
            for i in 0..15 {
                e[i] = rdf(pdi + i as u32 * 4);
            }
            // Shared partial products, each the value the original kept.
            let t58 = mul(e[8], a2);
            let t5c = mul(e[4], a1);
            let t4c = mul(e[5], a1);
            let t2c = mul(e[9], a2);
            let t50 = mul(e[6], a1);
            let t54 = mul(e[10], a2);
            let t60 = mul(e[8], c2);
            let t68 = mul(e[9], c2);
            let t64 = mul(e[4], c1);
            let t20b = mul(e[5], c1);
            let t30b = mul(e[0], c0);
            let t20c = mul(e[1], c0);
            let t10b = mul(e[2], c0);
            let t6c1 = mul(e[6], c1);
            let t10c2 = mul(e[10], c2);
            rows[0] = [
                add(add(add(mul(e[0], a0), t5c), t58), e[12]),
                add(add(add(mul(e[1], a0), t4c), t2c), e[13]),
                add(add(add(mul(e[2], a0), t50), t54), e[14]),
            ];
            rows[1] = [
                add(add(add(mul(e[0], a0), t5c), t60), e[12]),
                add(add(add(mul(e[1], a0), t4c), t68), e[13]),
                add(add(add(mul(e[2], a0), t50), t10c2), e[14]),
            ];
            rows[2] = [
                add(add(add(mul(e[4], c1), mul(e[0], a0)), t58), e[12]),
                add(add(add(mul(e[5], c1), mul(e[1], a0)), t2c), e[13]),
                add(add(add(t6c1, mul(e[2], a0)), t54), e[14]),
            ];
            rows[3] = [
                add(add(add(t64, mul(e[0], a0)), t60), e[12]),
                add(add(add(t20b, mul(e[1], a0)), t68), e[13]),
                add(add(add(t6c1, mul(e[2], a0)), t10c2), e[14]),
            ];
            rows[4] = [
                add(add(add(t30b, t64), t60), e[12]),
                add(add(add(t20c, t20b), t68), e[13]),
                add(add(add(t10b, t6c1), t10c2), e[14]),
            ];
            rows[5] = [
                add(add(add(t30b, t64), t58), e[12]),
                add(add(add(t20c, t20b), t2c), e[13]),
                add(add(add(t10b, t6c1), t54), e[14]),
            ];
            rows[6] = [
                add(add(add(t30b, t5c), t60), e[12]),
                add(add(add(t20c, t4c), t68), e[13]),
                add(add(add(t10b, t50), t10c2), e[14]),
            ];
            rows[7] = [
                add(add(add(t30b, t5c), t58), e[12]),
                add(add(add(t20c, t4c), t2c), e[13]),
                add(add(add(t10b, t50), t54), e[14]),
            ];
        }
        if (count as i32) <= 0 {
            cookie();
            return 0;
        }
        let mut slot = sbase.wrapping_add(SLOT_FIRST);
        for _ in 0..count {
            if rd32(slot.wrapping_sub(FLAG_BACK)) != 0 {
                slot = slot.wrapping_add(SLOT_STRIDE);
                continue;
            }
            let n = rd32(slot) as i32;
            if n <= 0 {
                if n == 0 {
                    cookie();
                    return 1;
                }
                slot = slot.wrapping_add(SLOT_STRIDE);
                continue;
            }
            let mut item = slot.wrapping_sub(ITEMS_BACK);
            let mut all_fail = true;
            for _ in 0..n {
                let v0 = rdf(item);
                let v1 = rdf(item + 4);
                let v2 = rdf(item + 8);
                let v3 = rdf(item + 12);
                let mut pass = true;
                for k in 0..N_ROWS as usize {
                    let r = rows[k];
                    let t = sub(add(add(mul(r[1], v1), mul(r[0], v0)), mul(r[2], v2)), v3);
                    if !(t > 0.0) {
                        pass = false;
                        break;
                    }
                }
                if pass {
                    all_fail = false;
                    break;
                }
                item = item.wrapping_add(ITEM_STRIDE);
            }
            if all_fail {
                cookie();
                return 1;
            }
            slot = slot.wrapping_add(SLOT_STRIDE);
        }
        cookie();
        0
    }
});
