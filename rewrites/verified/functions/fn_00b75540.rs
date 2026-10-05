// original: 0x00b75540 table_alloc_first_clear (proposed)

/// Scan a fixed table of 44-byte records for the first record whose status
/// byte (offset 0x29) does not have bit 0x10 set, then hand that record to
/// the per-record handler and return its index, or -1 when nothing qualifies.
///
/// Arguments (cdecl, 27 stack words): `a0` and `a1` are floats (bit patterns
/// in u32); `a2`..`a24` are opaque words forwarded to the handler; the low
/// byte of `a22` selects the scan range; the low bytes of `a25` and `a26`
/// gate the handler call through a predicate callee.
///
/// Behaviour in order:
/// 1. Early-out: when `|a0 - 360| < 5` and `|a1 + 118| < 5` (both strict,
///    NaN-safe: an unordered comparison counts as not-small on that side),
///    return -1 without touching anything else.
/// 2. Predicate gate: when both gate bytes are zero, skip the predicate
///    callee. Otherwise call it (no arguments); when its low byte is zero
///    continue only if the `a25` byte is set, else return -1; when nonzero
///    continue only if the `a26` byte is set, else return -1.
/// 3. Scan records 25..1100 when the `a22` low byte is zero, else records
///    0..25, stopping at the first record whose status byte lacks bit 0x10.
///    When every scanned record has the bit, return -1.
/// 4. Call the handler (thiscall: ECX = record address) with 25 stack words:
///    `a0`..`a21`, `a23`, `a24`, then `a22` last. Increment the allocation
///    counter word just past the table and return the record index.
///
/// The table base (0x1670D00 file address) and the counter (0x167CA10) are
/// reached through the checker's relocation helper; the float constants are
/// the exact single-precision values the original compares against. Float
/// order is pinned with `black_box` on both operands of each operation.
///
/// Original: 0x00B75540 (cdecl, 27 stack words).
lf_checker_rt::export!(cdecl, rw_00b75540(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, a9: u32, a10: u32, a11: u32, a12: u32, a13: u32, a14: u32, a15: u32, a16: u32, a17: u32, a18: u32, a19: u32, a20: u32, a21: u32, a22: u32, a23: u32, a24: u32, a25: u32, a26: u32) -> u32 {
    unsafe {
        const FABS_MASK: u32 = 0x7FFF_FFFF;
        const EPS: f32 = 5.0;
        const REF_A: f32 = 360.0;
        const REF_B: f32 = 118.0;
        const TABLE_BASE: u32 = 0x0167_0D00;
        const RECORD_LEN: u32 = 44;
        const STATUS_OFF: u32 = 0x29;
        const BUSY_BIT: u8 = 0x10;
        const SMALL_END: u32 = 25;
        const BIG_START: u32 = 25;
        const BIG_END: u32 = 1100;
        const COUNTER: u32 = 0x0167_CA10;
        const PRED_CALLEE: u32 = 0;
        const HANDLER_CALLEE: u32 = 1;
        const NONE: u32 = 0xFFFF_FFFF;

        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fabs_bits(v: f32) -> f32 {
            f32::from_bits(v.to_bits() & FABS_MASK)
        }

        // 1. Early-out gate on the two floats.
        let v0 = fabs_bits(fsub(f32::from_bits(a0), REF_A));
        if EPS > v0 {
            let v1 = fabs_bits(fadd(f32::from_bits(a1), REF_B));
            if EPS > v1 {
                return NONE;
            }
        }

        // 2. Predicate gate on the two flag bytes.
        let hi_set = (a25 & 0xFF) != 0;
        let lo_set = (a26 & 0xFF) != 0;
        if hi_set || lo_set {
            let r: u32 = lf_checker_rt::callee_cdecl!(PRED_CALLEE, u32,);
            if (r & 0xFF) == 0 {
                if !hi_set {
                    return NONE;
                }
            } else if !lo_set {
                return NONE;
            }
        }

        // 3. Scan for the first record without the busy bit.
        let table = lf_checker_rt::relocated(TABLE_BASE);
        let (mut i, end) = if (a22 & 0xFF) != 0 {
            (0u32, SMALL_END)
        } else {
            (BIG_START, BIG_END)
        };
        while i < end {
            let flag = ((table + STATUS_OFF + i.wrapping_mul(RECORD_LEN)) as *const u8).read();
            if flag & BUSY_BIT == 0 {
                break;
            }
            i += 1;
        }
        if i >= end {
            return NONE;
        }

        // 4. Hand the record to the handler, bump the counter, return index.
        let rec = table + i.wrapping_mul(RECORD_LEN);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            HANDLER_CALLEE, u32, rec,
            a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13,
            a14, a15, a16, a17, a18, a19, a20, a21, a23, a24, a22
        );
        let ctr = lf_checker_rt::relocated(COUNTER) as *mut u32;
        ctr.write_unaligned(ctr.read_unaligned().wrapping_add(1));
        i
    }
});
