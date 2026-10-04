// original: 0x0092ae00 cam_pose_update (proposed)
lf_checker_rt::export!(cdecl, rw_0092ae00(a0: u32) -> u32 {
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
                let mut st = [0.0f32; 0x5d];
        let _ = &mut st;
        let mut t001: u32 = 0;
        let mut t002: u32 = 0;
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
        let mut t123: u32 = 0;
        let mut t124: u32 = 0;
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
        let mut t136: u32 = 0;
        let mut t137: f32 = 0.0;
        let mut t138: u32 = 0;
        let mut t139: u32 = 0;
        let mut t140: u32 = 0;
        let mut t141: u32 = 0;
        let mut t142: u32 = 0;
        let mut t143: u32 = 0;
        t001 = rd32(reloc + 0x1174790);
        t002 = t001.wrapping_mul(0x110);
        t003 = a0;
        t004 = t002.wrapping_add(reloc).wrapping_add(0x11a2150);
        t005 = rdf(t003.wrapping_add(0x64));
        t006 = rdf(t004.wrapping_add(0xd0));
        t007 = rdf(t004.wrapping_add(0xd4));
        t008 = rdf(t004.wrapping_add(0xd8));
        st[0x2] = t005;
        t009 = rdf(t003.wrapping_add(0x70));
        st[0x10] = t009;
        t010 = rdf(t003.wrapping_add(0x74));
        st[0x17] = t010;
        t011 = rdf(t003.wrapping_add(0x78));
        st[0x14] = t011;
        t012 = f32::from_bits(0x80000000);
        t013 = fneg(t006);
        t014 = fneg(t007);
        t015 = fneg(t008);
        t016 = f32::from_bits(STACK_FILL);
        st[0x33] = t016;
        t017 = fabs(t015);
        st[0xc] = t013;
        st[0x1] = t014;
        st[0x8] = t015;
        t018 = 0.0;
        if t017 >= rdf(reloc + 0xfe88dc) {
            t019 = f32::from_bits(STACK_FILL);
            t020 = rdf(reloc + 0xfe88e8);
            st[0x2b] = t019;
        } else {
            t021 = fmul(t015, t018);
            t022 = fsub(t013, t021);
            t023 = fmul(t014, t018);
            t024 = fmul(t013, t018);
            t025 = fsub(t021, t014);
            t026 = fsub(t023, t024);
        }
        t027 = fmul(t026, t014);
        t028 = fmul(t022, t015);
        t029 = fmul(t025, t015);
        t030 = fsub(t027, t028);
        t031 = fmul(t026, st[0xc]);
        t032 = fmul(t022, st[0xc]);
        t033 = fsub(t029, t031);
        t034 = fmul(t025, st[0x1]);
        st[0x4] = t030;
        t035 = fsub(t032, t034);
        t036 = fmul(t022, t022);
        st[0x0] = t036;
        t037 = st[0x0];
        t038 = fmul(t025, t025);
        t039 = fadd(t037, t038);
        t040 = fmul(t026, t026);
        t041 = fadd(t039, t040);
        st[0x0] = t041;
        if t041 == t018 {
        } else {
            t042 = fsqrt(t041);
            t043 = rdf(reloc + 0xfe88e8);
            t044 = fdiv(t043, t042);
        }
        t045 = fmul(t033, t033);
        t046 = fmul(t025, t044);
        t047 = fmul(t022, t044);
        t048 = fmul(t026, t044);
        t049 = st[0x4];
        st[0x0] = t045;
        t050 = fmul(t049, t049);
        t051 = st[0x0];
        st[0x28] = t046;
        t052 = fadd(t051, t050);
        t053 = fmul(t035, t035);
        st[0x29] = t047;
        st[0x2a] = t048;
        t054 = fadd(t052, t053);
        st[0x0] = t054;
        if t054 == t018 {
        } else {
            t055 = fsqrt(t018);
            t056 = rdf(reloc + 0xfe88e8);
            t057 = fdiv(t056, t055);
            st[0xa] = t057;
        }
        t058 = st[0x4];
        t059 = fmul(t058, t057);
        t060 = fmul(t035, t057);
        t061 = fmul(t033, t057);
        st[0x4] = t059;
        st[0x2c] = t059;
        t062 = st[0xc];
        st[0xa] = t060;
        st[0x2e] = t060;
        st[0x0] = t061;
        st[0x2d] = t061;
        t063 = st[0x1];
        t064 = fmul(t062, t062);
        t065 = st[0x8];
        t066 = fmul(t063, t063);
        t067 = fadd(t066, t064);
        t068 = fmul(t065, t065);
        t069 = fadd(t067, t068);
        if t069 != t018 {
            t070 = 0.0;
            t071 = fsqrt(t069);
            t072 = rdf(reloc + 0xfe88e8);
            t073 = fdiv(t072, t071);
        }
        t074 = fmul(t063, t073);
        t075 = fmul(t065, t073);
        t076 = rdf(t004.wrapping_add(0x14));
        t077 = fmul(t073, st[0xc]);
        t078 = st[0x2];
        t079 = fmul(t078, t074);
        st[0x1] = t074;
        st[0x31] = t074;
        t080 = rdf(t003.wrapping_add(0x60));
        t081 = fmul(t080, t077);
        st[0x8] = t075;
        st[0x32] = t075;
        t082 = fadd(t079, t081);
        t083 = rdf(t003.wrapping_add(0x68));
        t084 = fmul(t083, t075);
        st[0x1c] = t047;
        t085 = fadd(t082, t084);
        t086 = st[0x1];
        st[0x18] = t046;
        st[0x20] = t048;
        st[0x30] = t077;
        t087 = fneg(t085);
        t088 = fmul(t087, t076);
        t089 = fmul(t076, rdf(reloc + 0xfe8830));
        t090 = fmul(t088, rdf(t004.wrapping_add(0x18)));
        t091 = fadd(t090, t089);
        wrf(t004.wrapping_add(0x10), t091);
        st[0x2] = t091;
        t092 = st[0x2];
        t093 = fmul(t077, t091);
        t094 = fmul(t086, t091);
        t095 = st[0x8];
        t096 = fmul(t092, t095);
        t097 = fadd(t093, st[0x10]);
        t098 = fadd(t094, st[0x17]);
        t099 = fadd(t096, st[0x14]);
        wrf(t004.wrapping_add(0x40), t046);
        wrf(t004.wrapping_add(0x44), t047);
        wrf(t004.wrapping_add(0x48), t048);
        st[0x34] = t097;
        st[0x2] = t099;
        st[0x36] = t099;
        t100 = f32::from_bits(STACK_FILL);
        st[0x37] = t100;
        t101 = st[0x4];
        wrf(t004.wrapping_add(0x50), t101);
        t102 = st[0x0];
        wrf(t004.wrapping_add(0x54), t102);
        t103 = st[0xa];
        wrf(t004.wrapping_add(0x58), t103);
        t104 = st[0x1];
        wrf(t004.wrapping_add(0x64), t104);
        t105 = st[0x2];
        wrf(t004.wrapping_add(0x68), t095);
        wrf(t004.wrapping_add(0x60), t077);
        wrf(t004.wrapping_add(0x78), t105);
        t106 = fmul(t097, t046);
        t107 = f32::from_bits(0x80000000);
        t108 = fmul(t098, t047);
        t109 = st[0x2];
        st[0x35] = t098;
        t110 = fadd(t108, t106);
        t111 = fmul(t109, t048);
        t112 = st[0x0];
        wrf(t004.wrapping_add(0x70), t097);
        t113 = fadd(t111, t110);
        t114 = st[0x4];
        st[0x19] = t114;
        wrf(t004.wrapping_add(0x74), t098);
        t115 = fneg(t113);
        st[0x24] = t115;
        t116 = fmul(t097, t114);
        t117 = fmul(t098, t112);
        t118 = fadd(t117, t116);
        st[0x1d] = t112;
        t119 = st[0xa];
        t120 = fmul(t109, t119);
        t121 = fmul(t097, t077);
        t122 = fadd(t120, t118);
        t123 = slotaddr(&st, 0x60);
        t124 = slotaddr(&st, 0xf0);
        t125 = fneg(t122);
        st[0x25] = t125;
        t126 = st[0x1];
        t127 = fmul(t098, t126);
        st[0x1e] = t126;
        t128 = st[0x8];
        t129 = fadd(t127, t121);
        t130 = fmul(t109, t128);
        st[0x21] = t119;
        st[0x1a] = t077;
        t131 = fadd(t130, t129);
        st[0x22] = t128;
        t132 = fneg(t131);
        st[0x26] = t132;
        lf_checker_rt::callee_cdecl!(1, u32, t124, t123);
        t133 = rdf(reloc + 0xfe8e30);
        { st[0x18] = f32::from_bits(0x3f800000); st[0x19] = f32::from_bits(0x00000000); st[0x1a] = f32::from_bits(0x00000000); st[0x1b] = f32::from_bits(0x00000000); }
        t134 = rdf(reloc + 0xfe8e40);
        { st[0x1c] = f32::from_bits(0x00000000); st[0x1d] = f32::from_bits(0x3f800000); st[0x1e] = f32::from_bits(0x00000000); st[0x1f] = f32::from_bits(0x00000000); }
        t135 = rdf(reloc + 0xe86390);
        t136 = slotaddr(&st, 0xf0);
        { st[0x20] = f32::from_bits(0x00000000); st[0x21] = f32::from_bits(0x00000000); st[0x22] = f32::from_bits(0x3f800000); st[0x23] = f32::from_bits(0x00000000); }
        t137 = rdf(reloc + 0xfe8f00);
        t138 = slotaddr(&st, 0x60);
        t139 = slotaddr(&st, 0x130);
        { st[0x24] = f32::from_bits(0x00000000); st[0x25] = f32::from_bits(0x00000000); st[0x26] = f32::from_bits(0x00000000); st[0x27] = f32::from_bits(0x3f800000); }
        lf_checker_rt::callee_thiscall!(2, u32, t139, t138, t136);
        t140 = slotaddr(&st, 0x130);
        t141 = t004.wrapping_add(0x80);
        lf_checker_rt::callee_thiscall!(3, u32, t141, t140);
        t142 = slotaddr(&st, 0xc0);
        t143 = slotaddr(&st, 0xd0);
        lf_checker_rt::callee_cdecl!(4, u32, t143, t142, t003, 1);
        // ---- inlined 0x0092b370 (native call5; verified separately) ----
        let mut st2 = [0.0f32; 12];
        let mut u001: u32 = 0;
        let mut u002: f32 = 0.0;
        let mut u003: u32 = 0;
        let mut u004: u32 = 0;
        let mut u005: f32 = 0.0;
        let mut u006: f32 = 0.0;
        let mut u007: f32 = 0.0;
        let mut u008: f32 = 0.0;
        let mut u009: f32 = 0.0;
        let mut u010: f32 = 0.0;
        let mut u011: f32 = 0.0;
        let mut u012: f32 = 0.0;
        let mut u013: f32 = 0.0;
        let mut u014: f32 = 0.0;
        let mut u015: f32 = 0.0;
        let mut u016: f32 = 0.0;
        let mut u017: f32 = 0.0;
        let mut u018: f32 = 0.0;
        let mut u019: f32 = 0.0;
        let mut u020: f32 = 0.0;
        let mut u021: f32 = 0.0;
        let mut u022: f32 = 0.0;
        let mut u023: f32 = 0.0;
        let mut u024: f32 = 0.0;
        let mut u025: f32 = 0.0;
        let mut u026: f32 = 0.0;
        let mut u027: f32 = 0.0;
        let mut u028: f32 = 0.0;
        let mut u029: f32 = 0.0;
        let mut u030: f32 = 0.0;
        let mut u031: f32 = 0.0;
        let mut u032: f32 = 0.0;
        let mut u033: f32 = 0.0;
        let mut u034: f32 = 0.0;
        let mut u035: f32 = 0.0;
        let mut u036: f32 = 0.0;
        let mut u037: f32 = 0.0;
        let mut u038: f32 = 0.0;
        let mut u039: f32 = 0.0;
        let mut u040: f32 = 0.0;
        let mut u041: f32 = 0.0;
        let mut u042: f32 = 0.0;
        let mut u043: f32 = 0.0;
        let mut u044: f32 = 0.0;
        let mut u045: f32 = 0.0;
        let mut u046: f32 = 0.0;
        let mut u047: f32 = 0.0;
        let mut u048: f32 = 0.0;
        let mut u049: f32 = 0.0;
        let mut u050: f32 = 0.0;
        let mut u051: f32 = 0.0;
        let mut u052: f32 = 0.0;
        let mut u053: f32 = 0.0;
        let mut u054: f32 = 0.0;
        let mut u055: f32 = 0.0;
        let mut u056: f32 = 0.0;
        let mut u057: f32 = 0.0;
        let mut u058: f32 = 0.0;
        let mut u059: f32 = 0.0;
        let mut u060: f32 = 0.0;
        let mut u061: f32 = 0.0;
        let mut u062: f32 = 0.0;
        let mut u063: f32 = 0.0;
        let mut u064: f32 = 0.0;
        let mut u065: f32 = 0.0;
        let mut u066: f32 = 0.0;
        let mut u067: f32 = 0.0;
        let mut u068: f32 = 0.0;
        let mut u069: f32 = 0.0;
        let mut u070: f32 = 0.0;
        let mut u071: f32 = 0.0;
        let mut u072: f32 = 0.0;
        u001 = rd32(reloc + 0x1174790);
        u002 = rdf(reloc + 0x11a1bf0);
        u003 = u001.wrapping_mul(0x110);
        u004 = u003.wrapping_add(reloc).wrapping_add(0x11a2150);
        u005 = rdf(u004.wrapping_add(0x70));
        u006 = rdf(u004.wrapping_add(0x74));
        u007 = rdf(u004.wrapping_add(0x78));
        u008 = rdf(reloc + 0x11a1bf4);
        u009 = rdf(reloc + 0x11a1bf8);
        if (u002 != u005) || (u008 != u006) || (u009 != u007) {
            u010 = f32::from_bits(STACK_FILL);
            u011 = rdf(u004.wrapping_add(0x44));
            u012 = rdf(u004.wrapping_add(0x48));
            u013 = rdf(u004.wrapping_add(0x50));
            u014 = rdf(u004.wrapping_add(0x40));
            wrf(reloc + 0x11a1bfc, u010);
            u015 = fadd(u011, rdf(u004.wrapping_add(0x54)));
            wrf(reloc + 0x11a1bf0, u005);
            u016 = fadd(u013, u014);
            u017 = fsub(u014, u013);
            st2[0x3] = u015;
            u018 = fadd(u012, rdf(u004.wrapping_add(0x58)));
            st2[0x4] = u016;
            u019 = st2[0x4];
            wrf(reloc + 0x11a1bf8, u007);
            u020 = fsub(u011, rdf(u004.wrapping_add(0x54)));
            st2[0x0] = u018;
            u021 = f32::from_bits(STACK_FILL);
            u022 = st2[0x0];
            wrf(reloc + 0x11a213c, u021);
            u023 = f32::from_bits(STACK_FILL);
            wrf(reloc + 0x11a214c, u023);
            u024 = st2[0x3];
            u025 = fmul(u024, u024);
            u026 = fmul(u019, u019);
            u027 = fsub(u012, rdf(u004.wrapping_add(0x58)));
            wrf(reloc + 0x11a1bf4, u006);
            u028 = fadd(u025, u026);
            u029 = fmul(u022, u022);
            u030 = fadd(u028, u029);
            st2[0x4] = u030;
            u031 = st2[0x4];
            u032 = 0.0;
            if u031 == u032 {
            } else {
                u033 = fsqrt(u031);
                u034 = rdf(reloc + 0xfe88e8);
                u035 = fdiv(u034, u033);
            }
            u036 = st2[0x3];
            u037 = fmul(u036, u035);
            u038 = fmul(u019, u035);
            st2[0x3] = u037;
            u039 = st2[0x0];
            u040 = fmul(u039, u035);
            u041 = fmul(u020, u020);
            st2[0x0] = u040;
            u042 = fmul(u017, u017);
            u043 = fadd(u041, u042);
            u044 = fmul(u027, u027);
            u045 = fadd(u043, u044);
            if u045 != u032 {
                u046 = fsqrt(u045);
                u047 = rdf(reloc + 0xfe88e8);
                u048 = fdiv(u047, u046);
            }
            u049 = rdf(reloc + 0xfe8d94);
            u050 = st2[0x0];
            u051 = fmul(u038, u049);
            u052 = fmul(u050, u049);
            u053 = fmul(u017, u048);
            u054 = fmul(u020, u048);
            u055 = fmul(u027, u048);
            u056 = st2[0x3];
            u057 = fmul(u056, u049);
            u058 = fmul(u053, u049);
            u059 = fmul(u054, u049);
            u060 = fmul(u055, u049);
            st2[0x4] = u051;
            st2[0x0] = u052;
            wrf(reloc + 0x11a2130, u051);
            u061 = rdf(u004.wrapping_add(0x70));
            wrf(reloc + 0x11a2138, u052);
            u062 = fmul(u061, st2[0x4]);
            u063 = fmul(u006, u057);
            u064 = fmul(u006, u059);
            wrf(reloc + 0x11a2134, u057);
            u065 = rdf(u004.wrapping_add(0x78));
            u066 = fmul(u061, u058);
            u067 = fadd(u063, u062);
            u068 = fmul(u065, st2[0x0]);
            u069 = fadd(u064, u066);
            u070 = fmul(u065, u060);
            u071 = fadd(u067, u068);
            wrf(reloc + 0x11a2140, u058);
            u072 = fadd(u069, u070);
            wrf(reloc + 0x11a2144, u059);
            wrf(reloc + 0x11a2148, u060);
            wrf(reloc + 0x11a1b78, u071);
            wrf(reloc + 0x11a1b7c, u072);
        }
        // ---- end inline ----
        0
    }
});
