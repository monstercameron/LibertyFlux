// original: 0x0092b370 cam_state_refresh (proposed)
//! Camera state refresh from the selected record (proposed name `cam_state_refresh`).
//!
//! Calling convention: cdecl, no arguments, no return value. All state lives in
//! globals: an index word, an array of 0x110-byte records selected by the index,
//! and shared output words.
//!
//! Layout: record base plus index times 0x110; reads two float triples at record
//! +0x40/+0x44/+0x48 and +0x50/+0x54/+0x58 and a position triple at +0x70/+0x74/+0x78;
//! compares the position triple against the shared triple word for word.
//!
//! Algorithm: if all three position words equal the shared triple (plain `==`, so a
//! NaN on either side counts as different), return with no writes. Otherwise form
//! pairwise sums and differences of the two triples, normalize each by
//! 1/sqrt(len-squared) with exact-zero length mapping to scale 0, and write the
//! normalized basis (eight words), two scaled scalars, and a copy of the position
//! triple. Also copies one uninitialized stack word to three globals; the checker
//! pins that word with `stack_fill` 0 and the rewrite emits the same constant.
//!
//! Edge cases: early exit writes nothing; NaN anywhere in the compared triples runs
//! the body; zero-length sums or differences yield zero scales (never infinities
//! from this path). Float order is pinned with `black_box` helpers; built at
//! opt-level 0 for the same NaN-payload codegen reason as the rest of this lane.
lf_checker_rt::export!(cdecl, rw_0092b370() -> u32 {
    unsafe {
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
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
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
        fn fdiv(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn fneg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ 0x80000000)
        }
        #[inline(always)]
        fn fabs(a: f32) -> f32 {
            f32::from_bits(a.to_bits() & 0x7fffffff)
        }
        #[inline(always)]
        fn fsqrt(a: f32) -> f32 {
            core::hint::black_box(a).sqrt()
        }
        #[inline(always)]
        fn slotaddr(st: &[f32], off: u32) -> u32 {
            unsafe { (st.as_ptr().add((off / 4) as usize)) as u32 }
        }
        const STACK_FILL: u32 = 0;
        let reloc: u32 = lf_checker_rt::xbase().wrapping_sub(0x400000);
                let mut st = [0.0f32; 0xc];
        let _ = &mut st;
        let mut t001: u32 = 0;
        let mut t002: f32 = 0.0;
        let mut t003: u32 = 0;
        let mut t004: u32 = 0;
        let mut t005: f32 = 0.0;
        let mut t006: f32 = 0.0;
        let mut t007: f32 = 0.0;
        let mut t008: f32 = 0.0;
        let mut t009: f32 = 0.0;
        let mut t010: f32 = 0.0;
        let mut t011: f32 = 0.0;
        let mut t012: f32 = 0.0;
        let mut t013: f32 = 0.0;
        let mut t014: f32 = 0.0;
        let mut t015: f32 = 0.0;
        let mut t016: f32 = 0.0;
        let mut t017: f32 = 0.0;
        let mut t018: f32 = 0.0;
        let mut t019: f32 = 0.0;
        let mut t020: f32 = 0.0;
        let mut t021: f32 = 0.0;
        let mut t022: f32 = 0.0;
        let mut t023: f32 = 0.0;
        let mut t024: f32 = 0.0;
        let mut t025: f32 = 0.0;
        let mut t026: f32 = 0.0;
        let mut t027: f32 = 0.0;
        let mut t028: f32 = 0.0;
        let mut t029: f32 = 0.0;
        let mut t030: f32 = 0.0;
        let mut t031: f32 = 0.0;
        let mut t032: f32 = 0.0;
        let mut t033: f32 = 0.0;
        let mut t034: f32 = 0.0;
        let mut t035: f32 = 0.0;
        let mut t036: f32 = 0.0;
        let mut t037: f32 = 0.0;
        let mut t038: f32 = 0.0;
        let mut t039: f32 = 0.0;
        let mut t040: f32 = 0.0;
        let mut t041: f32 = 0.0;
        let mut t042: f32 = 0.0;
        let mut t043: f32 = 0.0;
        let mut t044: f32 = 0.0;
        let mut t045: f32 = 0.0;
        let mut t046: f32 = 0.0;
        let mut t047: f32 = 0.0;
        let mut t048: f32 = 0.0;
        let mut t049: f32 = 0.0;
        let mut t050: f32 = 0.0;
        let mut t051: f32 = 0.0;
        let mut t052: f32 = 0.0;
        let mut t053: f32 = 0.0;
        let mut t054: f32 = 0.0;
        let mut t055: f32 = 0.0;
        let mut t056: f32 = 0.0;
        let mut t057: f32 = 0.0;
        let mut t058: f32 = 0.0;
        let mut t059: f32 = 0.0;
        let mut t060: f32 = 0.0;
        let mut t061: f32 = 0.0;
        let mut t062: f32 = 0.0;
        let mut t063: f32 = 0.0;
        let mut t064: f32 = 0.0;
        let mut t065: f32 = 0.0;
        let mut t066: f32 = 0.0;
        let mut t067: f32 = 0.0;
        let mut t068: f32 = 0.0;
        let mut t069: f32 = 0.0;
        let mut t070: f32 = 0.0;
        let mut t071: f32 = 0.0;
        let mut t072: f32 = 0.0;
        t001 = rd32(reloc + 0x1174790);
        t002 = rdf(reloc + 0x11a1bf0);
        t003 = t001.wrapping_mul(0x110);
        t004 = t003.wrapping_add(reloc).wrapping_add(0x11a2150);
        t005 = rdf(t004.wrapping_add(0x70));
        t006 = rdf(t004.wrapping_add(0x74));
        t007 = rdf(t004.wrapping_add(0x78));
        t008 = rdf(reloc + 0x11a1bf4);
        t009 = rdf(reloc + 0x11a1bf8);
        if (t002 != t005) || (t008 != t006) || (t009 != t007) {
            t010 = f32::from_bits(STACK_FILL);
            t011 = rdf(t004.wrapping_add(0x44));
            t012 = rdf(t004.wrapping_add(0x48));
            t013 = rdf(t004.wrapping_add(0x50));
            t014 = rdf(t004.wrapping_add(0x40));
            wrf(reloc + 0x11a1bfc, t010);
            t015 = fadd(t011, rdf(t004.wrapping_add(0x54)));
            wrf(reloc + 0x11a1bf0, t005);
            t016 = fadd(t013, t014);
            t017 = fsub(t014, t013);
            st[0x3] = t015;
            t018 = fadd(t012, rdf(t004.wrapping_add(0x58)));
            st[0x4] = t016;
            t019 = st[0x4];
            wrf(reloc + 0x11a1bf8, t007);
            t020 = fsub(t011, rdf(t004.wrapping_add(0x54)));
            st[0x0] = t018;
            t021 = f32::from_bits(STACK_FILL);
            t022 = st[0x0];
            wrf(reloc + 0x11a213c, t021);
            t023 = f32::from_bits(STACK_FILL);
            wrf(reloc + 0x11a214c, t023);
            t024 = st[0x3];
            t025 = fmul(t024, t024);
            t026 = fmul(t019, t019);
            t027 = fsub(t012, rdf(t004.wrapping_add(0x58)));
            wrf(reloc + 0x11a1bf4, t006);
            t028 = fadd(t025, t026);
            t029 = fmul(t022, t022);
            t030 = fadd(t028, t029);
            st[0x4] = t030;
            t031 = st[0x4];
            t032 = 0.0;
            if t031 == t032 {
            } else {
                t033 = fsqrt(t031);
                t034 = rdf(reloc + 0xfe88e8);
                t035 = fdiv(t034, t033);
            }
            t036 = st[0x3];
            t037 = fmul(t036, t035);
            t038 = fmul(t019, t035);
            st[0x3] = t037;
            t039 = st[0x0];
            t040 = fmul(t039, t035);
            t041 = fmul(t020, t020);
            st[0x0] = t040;
            t042 = fmul(t017, t017);
            t043 = fadd(t041, t042);
            t044 = fmul(t027, t027);
            t045 = fadd(t043, t044);
            if t045 != t032 {
                t046 = fsqrt(t045);
                t047 = rdf(reloc + 0xfe88e8);
                t048 = fdiv(t047, t046);
            }
            t049 = rdf(reloc + 0xfe8d94);
            t050 = st[0x0];
            t051 = fmul(t038, t049);
            t052 = fmul(t050, t049);
            t053 = fmul(t017, t048);
            t054 = fmul(t020, t048);
            t055 = fmul(t027, t048);
            t056 = st[0x3];
            t057 = fmul(t056, t049);
            t058 = fmul(t053, t049);
            t059 = fmul(t054, t049);
            t060 = fmul(t055, t049);
            st[0x4] = t051;
            st[0x0] = t052;
            wrf(reloc + 0x11a2130, t051);
            t061 = rdf(t004.wrapping_add(0x70));
            wrf(reloc + 0x11a2138, t052);
            t062 = fmul(t061, st[0x4]);
            t063 = fmul(t006, t057);
            t064 = fmul(t006, t059);
            wrf(reloc + 0x11a2134, t057);
            t065 = rdf(t004.wrapping_add(0x78));
            t066 = fmul(t061, t058);
            t067 = fadd(t063, t062);
            t068 = fmul(t065, st[0x0]);
            t069 = fadd(t064, t066);
            t070 = fmul(t065, t060);
            t071 = fadd(t067, t068);
            wrf(reloc + 0x11a2140, t058);
            t072 = fadd(t069, t070);
            wrf(reloc + 0x11a2144, t059);
            wrf(reloc + 0x11a2148, t060);
            wrf(reloc + 0x11a1b78, t071);
            wrf(reloc + 0x11a1b7c, t072);
        }
        0
    }
});
