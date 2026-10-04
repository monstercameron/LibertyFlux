// original: 0x0092b6c0 cam_matrix_from_basis (proposed)
//! Camera basis-to-matrix builder (proposed name `cam_matrix_from_basis`).
//!
//! Calling convention: cdecl, four pointer arguments, no return value.
//!
//! Arguments and layout read:
//! - `a0`: object pointer; reads three floats at +0x160/+0x164/+0x168 (direction A).
//! - `a1`: pointer to three floats at +0x0/+0x4/+0x8 (direction B).
//! - `a2`: basis table; reads fifteen floats at +0x40..+0x78 (rows consumed in order).
//! - `a3`: output pointer; writes sixteen words +0x0..+0x3C, a row-major 4x4 float
//!   matrix whose last column is fixed (0, 0, 0, 1) and whose other twelve words are
//!   computed. Reads no globals and makes no calls.
//!
//! Algorithm: negate A; form cross products of A and B; normalize the first cross
//! product by 1/sqrt(len-squared); accumulate dot products of the normalized axes
//! against the `a2` rows; scale the accumulated row by 0.25; negate; store. The
//! remaining rows repeat the accumulate/scale/negate/store pattern.
//!
//! Edge cases: when the first cross product has length exactly +0.0 or -0.0 the
//! scale is 0 (no division by zero); a NaN length takes the normalize path, so NaN
//! propagates into the outputs. All float operations are bit-exact, including NaN
//! sign and payload, which is why operand order is pinned with `black_box` helpers
//! and this crate is built at opt-level 0 (the O3 backend commutes commutative
//! float operands, changing which NaN payload survives; verified by A/B build).
lf_checker_rt::export!(cdecl, rw_0092b6c0(a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
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
                let mut st = [0.0f32; 0x1b];
        let _ = &mut st;
        let mut t001: u32 = 0;
        let mut t002: f32 = 0.0;
        let mut t003: f32 = 0.0;
        let mut t004: f32 = 0.0;
        let mut t005: f32 = 0.0;
        let mut t006: u32 = 0;
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
        let mut t034: u32 = 0;
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
        let mut t073: f32 = 0.0;
        let mut t074: f32 = 0.0;
        let mut t075: f32 = 0.0;
        let mut t076: f32 = 0.0;
        let mut t077: f32 = 0.0;
        let mut t078: f32 = 0.0;
        let mut t079: f32 = 0.0;
        let mut t080: f32 = 0.0;
        let mut t081: f32 = 0.0;
        let mut t082: f32 = 0.0;
        let mut t083: f32 = 0.0;
        let mut t084: f32 = 0.0;
        let mut t085: f32 = 0.0;
        let mut t086: f32 = 0.0;
        let mut t087: f32 = 0.0;
        let mut t088: f32 = 0.0;
        let mut t089: f32 = 0.0;
        let mut t090: f32 = 0.0;
        let mut t091: f32 = 0.0;
        let mut t092: f32 = 0.0;
        let mut t093: f32 = 0.0;
        let mut t094: f32 = 0.0;
        let mut t095: f32 = 0.0;
        let mut t096: f32 = 0.0;
        let mut t097: f32 = 0.0;
        let mut t098: f32 = 0.0;
        let mut t099: f32 = 0.0;
        let mut t100: f32 = 0.0;
        let mut t101: f32 = 0.0;
        let mut t102: f32 = 0.0;
        let mut t103: f32 = 0.0;
        let mut t104: f32 = 0.0;
        let mut t105: f32 = 0.0;
        let mut t106: f32 = 0.0;
        let mut t107: f32 = 0.0;
        let mut t108: f32 = 0.0;
        let mut t109: f32 = 0.0;
        let mut t110: f32 = 0.0;
        let mut t111: f32 = 0.0;
        let mut t112: f32 = 0.0;
        let mut t113: f32 = 0.0;
        let mut t114: f32 = 0.0;
        let mut t115: f32 = 0.0;
        let mut t116: f32 = 0.0;
        let mut t117: f32 = 0.0;
        let mut t118: f32 = 0.0;
        let mut t119: f32 = 0.0;
        let mut t120: f32 = 0.0;
        let mut t121: f32 = 0.0;
        let mut t122: f32 = 0.0;
        let mut t123: f32 = 0.0;
        let mut t124: f32 = 0.0;
        let mut t125: f32 = 0.0;
        let mut t126: f32 = 0.0;
        let mut t127: f32 = 0.0;
        let mut t128: f32 = 0.0;
        let mut t129: f32 = 0.0;
        let mut t130: f32 = 0.0;
        let mut t131: f32 = 0.0;
        let mut t132: f32 = 0.0;
        let mut t133: f32 = 0.0;
        let mut t134: f32 = 0.0;
        let mut t135: f32 = 0.0;
        let mut t136: f32 = 0.0;
        let mut t137: f32 = 0.0;
        let mut t138: f32 = 0.0;
        let mut t139: f32 = 0.0;
        let mut t140: f32 = 0.0;
        let mut t141: f32 = 0.0;
        let mut t142: f32 = 0.0;
        let mut t143: f32 = 0.0;
        let mut t144: f32 = 0.0;
        let mut t145: f32 = 0.0;
        let mut t146: f32 = 0.0;
        let mut t147: u32 = 0;
        let mut t148: f32 = 0.0;
        let mut t149: f32 = 0.0;
        let mut t150: f32 = 0.0;
        let mut t151: f32 = 0.0;
        let mut t152: f32 = 0.0;
        let mut t153: f32 = 0.0;
        let mut t154: f32 = 0.0;
        let mut t155: f32 = 0.0;
        let mut t156: f32 = 0.0;
        let mut t157: f32 = 0.0;
        let mut t158: f32 = 0.0;
        let mut t159: f32 = 0.0;
        let mut t160: f32 = 0.0;
        let mut t161: f32 = 0.0;
        let mut t162: f32 = 0.0;
        let mut t163: f32 = 0.0;
        let mut t164: f32 = 0.0;
        let mut t165: f32 = 0.0;
        let mut t166: f32 = 0.0;
        let mut t167: f32 = 0.0;
        let mut t168: f32 = 0.0;
        let mut t169: f32 = 0.0;
        let mut t170: f32 = 0.0;
        t001 = a0;
        t002 = f32::from_bits(0x80000000);
        t003 = rdf(t001.wrapping_add(0x160));
        t004 = rdf(t001.wrapping_add(0x164));
        t005 = rdf(t001.wrapping_add(0x168));
        t006 = a1;
        t007 = fneg(t005);
        t008 = rdf(t006.wrapping_add(0x8));
        t009 = rdf(t006.wrapping_add(0x4));
        t010 = rdf(t006.wrapping_add(0x0));
        t011 = fneg(t003);
        t012 = fneg(t004);
        t013 = fmul(t008, t012);
        t014 = fmul(t009, t007);
        st[0x2] = t008;
        t015 = fmul(t008, t011);
        t016 = fsub(t014, t013);
        t017 = fmul(t010, t007);
        st[0x4] = t010;
        t018 = fmul(t010, t012);
        t019 = fsub(t015, t017);
        t020 = fmul(t009, t011);
        t021 = fmul(t016, t016);
        t022 = fsub(t018, t020);
        t023 = 0.0;
        st[0x0] = t009;
        t024 = fmul(t019, t019);
        t025 = fadd(t021, t024);
        t026 = fmul(t022, t022);
        t027 = fadd(t025, t026);
        if t027 != t023 {
            t028 = 0.0;
            t029 = fsqrt(t027);
            t030 = rdf(reloc + 0xfe88e8);
            t031 = fdiv(t030, t029);
        }
        t032 = st[0x0];
        t033 = fmul(t031, t019);
        t034 = a2;
        t035 = fmul(t031, t022);
        t036 = st[0x2];
        t037 = fmul(t031, t016);
        t038 = st[0x4];
        t039 = fmul(t032, t035);
        t040 = fmul(t036, t033);
        t041 = fmul(t038, t035);
        t042 = fsub(t040, t039);
        t043 = fmul(t036, t037);
        st[0xc] = t035;
        t044 = st[0x0];
        t045 = fsub(t041, t043);
        t046 = fmul(t038, t033);
        t047 = fmul(t044, t037);
        st[0x6] = t037;
        t048 = st[0x2];
        t049 = fsub(t047, t046);
        t050 = f32::from_bits(0x80000000);
        t051 = fneg(t038);
        t052 = fneg(t044);
        t053 = fneg(t048);
        st[0x4] = t051;
        st[0x0] = t052;
        t054 = rdf(t034.wrapping_add(0x40));
        st[0x2] = t053;
        t055 = rdf(t034.wrapping_add(0x44));
        t056 = fmul(t055, t045);
        t057 = fmul(t054, t042);
        st[0xb] = t033;
        t058 = rdf(t034.wrapping_add(0x48));
        t059 = fadd(t056, t057);
        t060 = fmul(t058, t049);
        st[0x10] = t049;
        t061 = fmul(t055, st[0xb]);
        t062 = fadd(t059, t060);
        t063 = fmul(t054, st[0x6]);
        st[0xf] = t045;
        st[0x16] = t062;
        t064 = st[0x0];
        t065 = fadd(t061, t063);
        t066 = fmul(t058, st[0xc]);
        t067 = fmul(t064, t055);
        t068 = rdf(t034.wrapping_add(0x54));
        t069 = fadd(t065, t066);
        t070 = st[0x4];
        t071 = fmul(t070, t054);
        t072 = rdf(t034.wrapping_add(0x50));
        t073 = fmul(t068, st[0xf]);
        t074 = fadd(t067, t071);
        t075 = st[0x2];
        t076 = fmul(t075, t058);
        t077 = rdf(t034.wrapping_add(0x58));
        st[0x1a] = t042;
        t078 = fadd(t074, t076);
        t079 = fmul(t072, t042);
        t080 = fadd(t078, rdf(reloc + 0xfe88e8));
        t081 = fmul(t068, st[0xb]);
        t082 = fadd(t073, t079);
        t083 = fmul(t077, st[0x10]);
        t084 = fadd(t082, t083);
        st[0x12] = t084;
        t085 = st[0x0];
        t086 = fmul(t072, st[0x6]);
        t087 = fmul(t085, t068);
        t088 = fadd(t081, t086);
        t089 = fmul(t077, st[0xc]);
        t090 = rdf(t034.wrapping_add(0x64));
        t091 = fadd(t088, t089);
        t092 = st[0x4];
        t093 = fmul(t092, t072);
        t094 = rdf(t034.wrapping_add(0x60));
        t095 = fadd(t091, t069);
        t096 = fadd(t087, t093);
        t097 = st[0x2];
        t098 = fmul(t097, t077);
        t099 = rdf(t034.wrapping_add(0x68));
        t100 = fadd(t096, t098);
        t101 = st[0x12];
        t102 = fadd(t101, st[0x16]);
        t103 = fmul(t090, st[0xb]);
        t104 = fadd(t100, rdf(reloc + 0xfe88e8));
        st[0x12] = t102;
        t105 = fmul(t094, st[0x1a]);
        t106 = fadd(t104, t080);
        t107 = fmul(t090, st[0xf]);
        t108 = fadd(t107, t105);
        t109 = fmul(t099, st[0x10]);
        t110 = fadd(t108, t109);
        t111 = fmul(t094, st[0x6]);
        st[0x16] = t110;
        t112 = st[0x0];
        t113 = fadd(t103, t111);
        t114 = fmul(t099, st[0xc]);
        t115 = fmul(t112, t090);
        t116 = fadd(t113, t114);
        t117 = st[0x4];
        t118 = fmul(t117, t094);
        t119 = rdf(t034.wrapping_add(0x70));
        t120 = rdf(t034.wrapping_add(0x74));
        t121 = fadd(t115, t118);
        t122 = st[0x2];
        t123 = fmul(t122, t099);
        t124 = rdf(t034.wrapping_add(0x78));
        t125 = fadd(t116, t095);
        t126 = fadd(t121, t123);
        t127 = st[0x16];
        t128 = fadd(t127, st[0x12]);
        t129 = fmul(t120, st[0xb]);
        t130 = fadd(t126, rdf(reloc + 0xfe88e8));
        st[0x16] = t128;
        t131 = fmul(t119, st[0x1a]);
        t132 = fadd(t130, t106);
        t133 = fmul(t120, st[0xf]);
        t134 = fadd(t133, t131);
        t135 = fmul(t124, st[0x10]);
        t136 = fadd(t134, t135);
        t137 = fmul(t119, st[0x6]);
        st[0x12] = t136;
        t138 = st[0x0];
        t139 = fadd(t129, t137);
        t140 = fmul(t124, st[0xc]);
        t141 = fmul(t138, t120);
        t142 = fadd(t139, t140);
        t143 = st[0x4];
        t144 = fmul(t143, t119);
        t145 = st[0x2];
        t146 = fadd(t141, t144);
        t147 = a3;
        t148 = st[0x12];
        t149 = fadd(t148, st[0x16]);
        t150 = fmul(t145, t124);
        t151 = fadd(t142, t125);
        wr32(t147.wrapping_add(0xc), 0x0);
        t152 = fadd(t146, t150);
        t153 = rdf(reloc + 0xfe87e4);
        t154 = fmul(t149, t153);
        t155 = fmul(t151, t153);
        t156 = fadd(t152, rdf(reloc + 0xfe88e8));
        t157 = fadd(t156, t132);
        t158 = fmul(t157, t153);
        t159 = f32::from_bits(0x80000000);
        t160 = fneg(t154);
        t161 = fneg(t155);
        t162 = fneg(t158);
        t163 = st[0x1a];
        wrf(t147.wrapping_add(0x0), t163);
        t164 = st[0x6];
        wrf(t147.wrapping_add(0x4), t164);
        t165 = st[0x4];
        wrf(t147.wrapping_add(0x8), t165);
        t166 = st[0xf];
        wrf(t147.wrapping_add(0x10), t166);
        t167 = st[0xb];
        wrf(t147.wrapping_add(0x14), t167);
        t168 = st[0x0];
        wrf(t147.wrapping_add(0x18), t168);
        t169 = st[0x10];
        wr32(t147.wrapping_add(0x1c), 0x0);
        wrf(t147.wrapping_add(0x20), t169);
        t170 = st[0xc];
        wrf(t147.wrapping_add(0x24), t170);
        wrf(t147.wrapping_add(0x28), t145);
        wr32(t147.wrapping_add(0x2c), 0x0);
        wrf(t147.wrapping_add(0x30), t160);
        wrf(t147.wrapping_add(0x34), t161);
        wrf(t147.wrapping_add(0x38), t162);
        wr32(t147.wrapping_add(0x3c), 0x3f800000);
        0
    }
});
