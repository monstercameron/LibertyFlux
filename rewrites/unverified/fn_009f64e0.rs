// original: 0x009F64E0 Stat_AddFloat

/// Add a positive amount to a statistic and record the new total.
///
/// `stat_id` selects the counter, `amount_bits` is the added float. Counters
/// at or below `MAX_FLOAT_STAT` live in a float table, the next
/// `INT_STAT_COUNT` ids live in a signed-int table (converted to float);
/// other ids skip the record step. A gate callee must accept the id, and a
/// second callee decides whether the sum is clamped to a global maximum
/// before a store callee records it. All comparisons use ordered float
/// semantics: a NaN amount returns immediately, and a NaN sum is clamped.
///
/// Two ids chain further work: `CHAIN_STAT` re-runs the whole step for
/// `DERIVED_STAT` when the gate accepts it, and `DERIVED_STAT` runs a rating
/// block. That block resolves an object through a global index (index -1
/// faults on a null object, like the original), scales a signed 16-bit field
/// down by the amount times a global rate, clamps it into [0, a global max],
/// converts it back with truncation (out-of-range or NaN becomes the
/// indefinite `0x80000000`, low 16 bits stored), then calls two object
/// routines: one nullary routine returning a float, and one taking the
/// amount, or the adjusted rating when it is not above the amount (which also
/// zeroes the stored field).
///
/// The clamp callee's second argument is not the sum: the sum is spilled four
/// bytes past the argument slot, so the callee reads the caller's entry `esi`
/// still sitting in its register-save slot. The rewrite takes that value as
/// an explicit third parameter fed with the same per-trial value.
///
/// Original: cdecl, two stack words (id, float bits), no meaningful return.
lf_checker_rt::export!(cdecl, rw_009F64E0(stat_id: u32, amount_bits: u32, entry_esi: u32) -> u32 {
    unsafe {
        const MAX_FLOAT_STAT: u32 = 0xfc;
        const INT_STAT_BASE: u32 = 0xfd;
        const INT_STAT_COUNT: u32 = 0x18b;
        const CHAIN_STAT: u32 = 0x1b9;
        const DERIVED_STAT: u32 = 0x2ab;
        const FLOAT_STATS: u32 = 0x12b75b0;
        const INT_STATS: u32 = 0x12b79c8;
        const STAT_MAXIMUM: u32 = 0xfe8c58;
        const DERATE_FACTOR: u32 = 0xfe8830;
        const RATING_MAXIMUM: u32 = 0xfe8b58;
        const OBJECT_INDEX: u32 = 0x1036f14;
        const OBJECT_TABLE: u32 = 0x11a8808;
        const OBJ_RATING: u32 = 0x550;
        const OBJ_BASELINE: u32 = 0x556;
        const OBJ_WORKER: u32 = 0x598;
        const VT_SLOT_MEASURE: u32 = 0xfc;
        const VT_SLOT_APPLY: u32 = 0xf8;
        const GATE_CALLEE: u32 = 0;
        const CLAMP_CALLEE: u32 = 1;
        const STORE_CALLEE: u32 = 2;

        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        /// Truncating float-to-int with `cvttss2si` semantics: NaN and
        /// out-of-range values yield `0x80000000` instead of saturating.
        fn cvttss2si(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                i32::MIN
            } else {
                x as i32
            }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        /// One record step for `id`: look up the current value, add the
        /// amount, maybe clamp, and store. Ids outside both tables do nothing.
        /// `entry_esi` is the caller's register-save slot, which the clamp
        /// callee reads as its second argument.
        #[inline(always)]
        unsafe fn record(id: u32, amount: f32, float_stats: u32, int_stats: u32, entry_esi: u32) {
            unsafe {
                let current = if id <= MAX_FLOAT_STAT {
                    f32::from_bits(rd32(float_stats + id * 4))
                } else if id.wrapping_sub(INT_STAT_BASE) <= INT_STAT_COUNT {
                    ((int_stats + (id - INT_STAT_BASE) * 4) as *const i32).read_unaligned() as f32
                } else {
                    return;
                };
                let mut total = add(current, amount);
                let clamped =
                    lf_checker_rt::callee_cdecl!(CLAMP_CALLEE, u32, id, entry_esi) != 0;
                if clamped {
                    let max = rdf(lf_checker_rt::relocated(STAT_MAXIMUM));
                    if !(max > total) {
                        total = max;
                    }
                }
                lf_checker_rt::callee_cdecl!(STORE_CALLEE, u32, id, total.to_bits());
            }
        }

        let amount = f32::from_bits(amount_bits);
        let float_stats = lf_checker_rt::relocated(FLOAT_STATS);
        let int_stats = lf_checker_rt::relocated(INT_STATS);

        let mut id = stat_id;
        if lf_checker_rt::callee_cdecl!(GATE_CALLEE, u32, id) == 0 {
            return 0;
        }
        if !(amount > 0.0) {
            return 0;
        }
        record(id, amount, float_stats, int_stats, entry_esi);
        if id == CHAIN_STAT {
            id = DERIVED_STAT;
            if lf_checker_rt::callee_cdecl!(GATE_CALLEE, u32, id) == 0 {
                return 0;
            }
            if !(amount > 0.0) {
                return 0;
            }
            record(id, amount, float_stats, int_stats, entry_esi);
        } else if id != DERIVED_STAT {
            return 0;
        }

        // Rating block for DERIVED_STAT (reached directly or via the chain).
        let index = rd32(lf_checker_rt::relocated(OBJECT_INDEX));
        let obj = if index == 0xffff_ffff {
            0
        } else {
            rd32(lf_checker_rt::relocated(OBJECT_TABLE) + index.wrapping_mul(4))
        };
        let rating = ((obj + OBJ_RATING) as *const i16).read_unaligned() as i32;
        let worker = rd32(obj + OBJ_WORKER);
        let rate = rdf(lf_checker_rt::relocated(DERATE_FACTOR));
        let residual = sub(rating as f32, mul(amount, rate));
        let max_rating = rdf(lf_checker_rt::relocated(RATING_MAXIMUM));
        let limited = if 0.0 > residual {
            0.0
        } else if max_rating > residual {
            residual
        } else {
            max_rating
        };
        let baseline = ((obj + OBJ_BASELINE) as *const u16).read_unaligned() as u32;
        ((obj + OBJ_RATING) as *mut u16).write_unaligned(cvttss2si(limited) as u16);
        let vtable = rd32(worker);
        let measure: extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute(rd32(vtable + VT_SLOT_MEASURE) as usize);
        let measured = measure(worker);
        let adjusted = sub(baseline as f32, measured);
        let mut applied = amount;
        if !(amount < adjusted) {
            ((obj + OBJ_RATING) as *mut u16).write_unaligned(0);
            applied = adjusted;
        }
        let apply: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable + VT_SLOT_APPLY) as usize);
        apply(worker, applied.to_bits());
        0
    }
});
