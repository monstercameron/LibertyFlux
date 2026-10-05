// original: 0x0096ACF0 DEAFENING_STRENGTH_TO_ATTENUATION (merged name, low confidence)

/// Initialise the attenuation voice bank: zero and default several hundred
/// fields, build sine/cosine direction tables, and wire the sub-voices.
///
/// `this` is the bank object (about 12.8 KiB, fields up to `+0x3210`). The
/// function takes no stack arguments and returns the fill callee's answer.
/// It makes 43 calls to 11 callees, always in the same order: six
/// sub-voice initialisers, gain/filter setups, sine/cosine table fills,
/// a 64-slot randomised slot table, a buffer release and reallocation,
/// and the stack-guard check.
///
/// Layout built, in order: a 16-word global preset block (four of its
/// words copy one uninitialised stack word, which the proof fixes to zero
/// through the contract's defined stack fill); per-voice defaults through
/// the one-argument sub-voice callee; a 24-entry sine/cosine pair table;
/// some hundred scalar defaults; a 4-by-(8+4) table of normalised
/// direction triples (each triple is scaled by the table high value over
/// its length, or zeroed when its squared length is exactly zero, with
/// the two inner loops adding the squares in opposite orders); zeroed
/// scratch grids; per-slot gain/filter setups through the four-argument
/// callee (rates come from globals times one shared factor); a repeat
/// fill; the 64 randomised slots; and the reallocated mix buffer.
///
/// The late buffer calls use deferred stack cleanup (one `(an instruction of the original)`
/// covers five pushed words), and the guard check preserves all registers.
/// Float operation order is the original's throughout.
///
/// Original: 0x0096ACF0 (thiscall, ECX = bank, no stack words).
lf_checker_rt::export!(thiscall, rw_0096ACF0(this: u32) -> u32 {
    unsafe {
        const F_ONE: u32 = 0x3F800000;
        const F_NEG_ONE: u32 = 0xBF800000;
        const C_SUBINIT: u32 = 1;
        const C_GAIN4: u32 = 2;
        const C_ZERO6: u32 = 3;
        const C_SIN: u32 = 4;
        const C_COS: u32 = 5;
        const C_RNG: u32 = 6;
        const C_DEL: u32 = 7;
        const C_NEW: u32 = 8;
        const C_FILL: u32 = 9;
        const C_F32P: u32 = 10;
        const C_COOKIE: u32 = 11;
        const G_SIN_STEP1: u32 = 0x00fe8aec;
        const G_R2_STEP2: u32 = 0x00fe8768;
        const G_R4_STEP2: u32 = 0x00fe87a4;
        const G_NORM_HI: u32 = 0x00fe88e8;
        const G_RATE: u32 = 0x00fe86b4;
        const G_A00: u32 = 0x01037a00;
        const G_A18: u32 = 0x01037a18;
        const G_A30: u32 = 0x01037a30;
        const G_A38: u32 = 0x01037a38;
        const G_A3C: u32 = 0x01037a3c;
        const G_A44: u32 = 0x01037a44;
        const G_A48: u32 = 0x01037a48;
        const G_A5C: u32 = 0x01037a5c;
        const G_ARR: u32 = 0x01030808;
        const G_ID: u32 = 0x011735b4;
        const G_H: u32 = 0x017acc5c;
        const K3: [u32; 4] = [0x3f800000, 0x3ed3f7cf, 0xbed3f7cf, 0xbf800000];
        const K4: [u32; 4] = [0x41300000, 0x4019999a, 0xc019999a, 0xc1300000];

        #[inline(always)]
        unsafe fn r32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn w32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn w16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn w8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wf(a: u32, v: f32) {
            unsafe { w32(a, v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn rg32(addr: u32) -> u32 {
            unsafe { *lf_checker_rt::global::<u32>(addr) }
        }
        #[inline(always)]
        unsafe fn rgf(addr: u32) -> f32 {
            unsafe { *lf_checker_rt::global::<f32>(addr) }
        }
        #[inline(always)]
        unsafe fn wg32(addr: u32, v: u32) {
            unsafe { *lf_checker_rt::global::<u32>(addr) = v }
        }
        #[inline(always)]
        fn ra(file_va: u32) -> u32 {
            lf_checker_rt::relocated(file_va)
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
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        /// Call the vector-register sine/cosine callees (the stub loads the
        /// argument from the pushed word and answers in both xmm0 and eax).
        #[inline(always)]
        fn shape(id: u32, x: f32) -> f32 {
            let bits: u32 = lf_checker_rt::callee_cdecl!(id, u32, x.to_bits());
            f32::from_bits(bits)
        }

        // ---- global preset block (one stack word is uninitialised scratch;
        // the contract's zero stack fill makes it read 0.0 on both sides) ----
    // ---- R0: 16 field stores ----
    wg32(0x121F5E0, 0x0);
    wg32(0x121F5E4, F_ONE);
    wg32(0x121F5E8, 0x0);
    wg32(0x121F5F0, F_ONE);
    wg32(0x121F5F4, 0x0);
    wg32(0x121F5F8, 0x0);
    wg32(0x121F600, 0x0);
    wg32(0x121F604, F_NEG_ONE);
    wg32(0x121F608, 0x0);
    wg32(0x121F610, F_NEG_ONE);
    wg32(0x121F614, 0x0);
    wg32(0x121F618, 0x0);

        wg32(0x121F5EC, 0);
        wg32(0x121F5FC, 0);
        wg32(0x121F60C, 0);
        wg32(0x121F61C, 0);
        lf_checker_rt::callee_thiscall!(C_SUBINIT, u32, this + 0x1240, ra(0x00E8B5CC));

        // ---- first voices ----
    // ---- R1: 10 field stores ----
    w32(this + 0x232C, F_ONE);
    w16(this + 0x1268, 0x0);
    w32(this + 0x1270, 0x0);
    w32(this + 0x126C, 0x0);
    w32(this + 0x1278, 0x0);
    w32(this + 0x1274, 0x0);
    w32(this + 0x1280, 0x0);
    w32(this + 0x127C, 0x0);
    w32(this + 0x2FA4, 0x0);
    w32(this + 0x2FC4, 0x0);

        lf_checker_rt::callee_thiscall!(C_SUBINIT, u32, this + 0x1284, ra(0x00E8B5E0));
        lf_checker_rt::callee_thiscall!(C_SUBINIT, u32, this + 0x12ac, ra(0x00E8B604));
        lf_checker_rt::callee_thiscall!(C_SUBINIT, u32, this + 0x12d4, ra(0x00E8B624));
        lf_checker_rt::callee_thiscall!(C_SUBINIT, u32, this + 0x12fc, ra(0x00E8B644));
        lf_checker_rt::callee_thiscall!(C_SUBINIT, u32, this + 0x1324, ra(0x00E8B668));
        lf_checker_rt::callee_thiscall!(C_SUBINIT, u32, ra(0x01218530), ra(0x00E8B678));
        lf_checker_rt::callee_thiscall!(C_GAIN4, u32, this + 0x2fa8, 0x3b3b3ee7u32, 0x3b3b3ee7u32, 0u32, F_ONE);
        lf_checker_rt::callee_thiscall!(C_GAIN4, u32, this + 0x2fc8, 0x3b3b3ee7u32, 0x3b3b3ee7u32, 0u32, F_ONE);
        lf_checker_rt::callee_thiscall!(C_ZERO6, u32, this);

        // ---- 24-entry sine/cosine pair table ----
    // ---- R2: 3 field stores ----
    w32(this + 0x1350, 0x0);
    w32(this + 0x134C, 0x0);
    w8(this + 0x1354, 0x0);

        {
            let s1 = rgf(G_SIN_STEP1);
            let s2 = rgf(G_R2_STEP2);
            let mut row = this;
            let mut tab = this + 0x1394;
            let mut i = 0u32;
            while i < 0x18 {
                let t = mul(mul(i as f32, s1), s2);
                let s = shape(C_SIN, t);
                let c = shape(C_COS, t);
                wf(row, s);
                wf(row + 4, c);
                w32(row + 8, 0);
                w32(tab, 0);
                w32(tab + 4, 0);
                w32(tab - 4, 0);
                i += 1;
                row += 0x10;
                tab += 0x10;
            }
        }

    // ---- R3: 100 field stores ----
    w32(this + 0x1690, F_NEG_ONE);
    w32(this + 0x1694, F_NEG_ONE);
    w32(this + 0x1698, F_NEG_ONE);
    w32(this + 0x169C, F_NEG_ONE);
    w32(this + 0x16A0, F_NEG_ONE);
    w32(this + 0x16A4, F_NEG_ONE);
    w32(this + 0x16A8, F_NEG_ONE);
    w32(this + 0x16AC, F_NEG_ONE);
    w32(this + 0x16B0, F_NEG_ONE);
    w32(this + 0x16B4, F_NEG_ONE);
    w32(this + 0x16B8, F_NEG_ONE);
    w32(this + 0x16BC, F_NEG_ONE);
    w32(this + 0x1510, 0x0);
    w32(this + 0x1518, 0x0);
    w32(this + 0x1514, 0x0);
    w32(this + 0x1C80, 0x0);
    w32(this + 0x1C60, 0x0);
    w32(this + 0x1CD0, 0x0);
    w32(this + 0x1CD8, 0x0);
    w32(this + 0x1CD4, 0x0);
    w32(this + 0x1D50, F_ONE);
    w32(this + 0x1D58, F_ONE);
    w32(this + 0x1D54, F_ONE);
    w32(this + 0x1528, 0x0);
    w32(this + 0x1524, 0x0);
    w32(this + 0x1520, 0x0);
    w32(this + 0x1C84, 0x0);
    w32(this + 0x1C64, 0x0);
    w32(this + 0x1CE8, 0x0);
    w32(this + 0x1CE4, 0x0);
    w32(this + 0x1CE0, 0x0);
    w32(this + 0x1D68, F_ONE);
    w32(this + 0x1D64, F_ONE);
    w32(this + 0x1D60, F_ONE);
    w32(this + 0x1538, 0x0);
    w32(this + 0x1534, 0x0);
    w32(this + 0x1530, 0x0);
    w32(this + 0x1C88, 0x0);
    w32(this + 0x1C68, 0x0);
    w32(this + 0x1CF8, 0x0);
    w32(this + 0x1CF4, 0x0);
    w32(this + 0x1CF0, 0x0);
    w32(this + 0x1D78, F_ONE);
    w32(this + 0x1D74, F_ONE);
    w32(this + 0x1D70, F_ONE);
    w32(this + 0x1548, 0x0);
    w32(this + 0x1544, 0x0);
    w32(this + 0x1540, 0x0);
    w32(this + 0x1C8C, 0x0);
    w32(this + 0x1C6C, 0x0);
    w32(this + 0x1D08, 0x0);
    w32(this + 0x1D04, 0x0);
    w32(this + 0x1D00, 0x0);
    w32(this + 0x1D88, F_ONE);
    w32(this + 0x1D84, F_ONE);
    w32(this + 0x1D80, F_ONE);
    w32(this + 0x1558, 0x0);
    w32(this + 0x1554, 0x0);
    w32(this + 0x1550, 0x0);
    w32(this + 0x1C90, 0x0);
    w32(this + 0x1C70, 0x0);
    w32(this + 0x1D18, 0x0);
    w32(this + 0x1D14, 0x0);
    w32(this + 0x1D10, 0x0);
    w32(this + 0x1D98, F_ONE);
    w32(this + 0x1D94, F_ONE);
    w32(this + 0x1D90, F_ONE);
    w32(this + 0x1568, 0x0);
    w32(this + 0x1564, 0x0);
    w32(this + 0x1560, 0x0);
    w32(this + 0x1C94, 0x0);
    w32(this + 0x1C74, 0x0);
    w32(this + 0x1D28, 0x0);
    w32(this + 0x1D24, 0x0);
    w32(this + 0x1D20, 0x0);
    w32(this + 0x1DA8, F_ONE);
    w32(this + 0x1DA4, F_ONE);
    w32(this + 0x1DA0, F_ONE);
    w32(this + 0x1578, 0x0);
    w32(this + 0x1574, 0x0);
    w32(this + 0x1570, 0x0);
    w32(this + 0x1C98, 0x0);
    w32(this + 0x1C78, 0x0);
    w32(this + 0x1D38, 0x0);
    w32(this + 0x1D34, 0x0);
    w32(this + 0x1D30, 0x0);
    w32(this + 0x1DB8, F_ONE);
    w32(this + 0x1DB4, F_ONE);
    w32(this + 0x1DB0, F_ONE);
    w32(this + 0x1588, 0x0);
    w32(this + 0x1584, 0x0);
    w32(this + 0x1580, 0x0);
    w32(this + 0x1C9C, 0x0);
    w32(this + 0x1C7C, 0x0);
    w32(this + 0x1D48, 0x0);
    w32(this + 0x1D44, 0x0);
    w32(this + 0x1D40, 0x0);
    w32(this + 0x1DC8, F_ONE);
    w32(this + 0x1DC4, F_ONE);
    w32(this + 0x1DC0, F_ONE);


        // ---- 4 x (8 + 4) normalised direction triples ----
        {
            let s1 = rgf(G_SIN_STEP1);
            let s2 = rgf(G_R4_STEP2);
            let hi = rgf(G_NORM_HI);
            let mut base = this + 0x384;
            let mut outer = 0u32;
            while outer < 4 {
                let mut cur = base - 0x200;
                let mut i = 0u32;
                while i < 8 {
                    let t = mul(mul(i as f32, s1), s2);
                    let mut s = shape(C_SIN, t);
                    let mut c = shape(C_COS, t);
                    let mut k = f32::from_bits(K3[outer as usize]);
                    // Squared length added as (c*c + s*s) + k*k here.
                    let n = add(add(mul(c, c), mul(s, s)), mul(k, k));
                    let f = if n != 0.0 { div(hi, n.sqrt()) } else { 0.0 };
                    s = mul(s, f);
                    c = mul(c, f);
                    k = mul(k, f);
                    wf(cur - 4, s);
                    wf(cur, c);
                    wf(cur + 4, k);
                    i += 1;
                    cur += 0x10;
                }
                cur = base;
                i = 0;
                while i < 4 {
                    let t = mul(mul(i as f32, s1), s2);
                    let mut s = shape(C_SIN, t);
                    let mut c = shape(C_COS, t);
                    let mut k = f32::from_bits(K4[outer as usize]);
                    // Opposite order here: (s*s + c*c) + k*k.
                    let n = add(add(mul(s, s), mul(c, c)), mul(k, k));
                    let f = if n != 0.0 { div(hi, n.sqrt()) } else { 0.0 };
                    s = mul(s, f);
                    c = mul(c, f);
                    k = mul(k, f);
                    wf(cur - 4, s);
                    wf(cur, c);
                    wf(cur + 4, k);
                    i += 1;
                    cur += 0x10;
                }
                base += 0x80;
                outer += 1;
            }
        }

        // ---- zero grid plus 32 gain/filter slots ----
        {
            let mut a = this + 0x1840;
            let mut n = 0u32;
            while n < 0x18 {
                w32(a - 0x60, 0);
                w32(a, 0);
                w32(a + 0x60, 0);
                w32(a + 0xc0, 0);
                a += 4;
                n += 1;
            }
            let t = mul(rgf(G_A5C), rgf(G_RATE));
            let mut s3 = this + 0x5b4;
            let mut e = this + 0xa30;
            let mut f = this + 0x19e0;
            let mut o = 0u32;
            while o < 4 {
                let mut k = 0u32;
                while k < 8 {
                    w32(f - 0x80, 0);
                    w32(f, 0);
                    w32(f + 0x80, 0);
                    w32(f + 0x100, 0);
                    w32(s3 + 4, 0);
                    w32(s3, 0);
                    w32(s3 - 4, 0);
                    w32(s3 + 0x204, 0);
                    w32(s3 + 0x200, 0);
                    w32(s3 + 0x1fc, 0);
                    w32(f - 0x1030, 0);
                    lf_checker_rt::callee_thiscall!(C_GAIN4, u32, e, t.to_bits(), t.to_bits(), 0u32, F_ONE);
                    e += 0x1c;
                    s3 += 0x10;
                    f += 4;
                    k += 1;
                }
                o += 1;
            }
        }

        // ---- zero grid plus 4 gain/filter slots ----
        {
            let mut a = this + 0x1ba0;
            let mut o = 0u32;
            while o < 4 {
                let mut k = 0u32;
                while k < 4 {
                    w32(a - 0x40, 0);
                    w32(a, 0);
                    w32(a + 0x40, 0);
                    w32(a + 0x80, 0);
                    a += 4;
                    k += 1;
                }
                o += 1;
            }
            lf_checker_rt::callee_thiscall!(C_SUBINIT, u32, this + 0x16f0, ra(0x00E8B690));
            let t = mul(rgf(G_A18), rgf(G_RATE));
            let mut s = this + 0x1728;
            let mut f = this + 0x17c0;
            let mut k = 0u32;
            while k < 4 {
                w32(f - 0xa8, 0);
                lf_checker_rt::callee_thiscall!(C_GAIN4, u32, s, t.to_bits(), t.to_bits(), 0u32, F_ONE);
                s += 0x1c;
                w32(f, F_ONE);
                w32(f + 0x10, 0);
                f += 4;
                k += 1;
            }
        }

        // ---- more voices plus a zero block ----
    // ---- R7: 9 field stores ----
    w32(this + 0x588, 0x0);
    w32(this + 0x584, 0x0);
    w32(this + 0x580, 0x0);
    w32(this + 0x598, 0x0);
    w32(this + 0x594, 0x0);
    w32(this + 0x590, 0x0);
    w32(this + 0x5A0, 0x0);
    w32(this + 0x5A4, 0x0);
    w32(this + 0x1E80, 0x0);

        lf_checker_rt::callee_thiscall!(C_SUBINIT, u32, this + 0x1798, ra(0x00E8B6A4));
        lf_checker_rt::callee_thiscall!(C_SUBINIT, u32, this + 0x1ca0, ra(0x00E8B6C0));
        lf_checker_rt::callee_thiscall!(C_SUBINIT, u32, this + 0x1df8, ra(0x00E8B6DC));
        lf_checker_rt::callee_thiscall!(C_SUBINIT, u32, this + 0x1e20, ra(0x00E8B6F8));
        lf_checker_rt::callee_thiscall!(C_SUBINIT, u32, this + 0x1dd0, ra(0x00E8B718));
        lf_checker_rt::callee_thiscall!(C_SUBINIT, u32, this + 0x1e48, ra(0x00E8B734));
        {
            let mut c = this + 0x2010;
            let mut d = this + 0x1ec4;
            let mut a = this + 0x1f14;
            let mut k = 0u32;
            while k < 0x10 {
                w32(d - 0x40, 0);
                w32(d, 0);
                w32(a + 4, 0);
                w32(a, 0);
                w32(a - 4, 0);
                w32(c, 0);
                w32(c + 4, 0);
                w32(c + 8, 0);
                d += 4;
                a += 0x10;
                c += 0xc;
                k += 1;
            }
        }

        // ---- 12 gain/filter slots (counter-sourced, raw rate) ----
        // The exhausted loop counter (0) lands in four fields first.
    // ---- R8: 4 field stores ----

        w32(this + 0x20D8, 0);
        w32(this + 0x20D4, 0);
        w32(this + 0x20D0, 0);
        w32(this + 0x20E0, 0);
        lf_checker_rt::callee_thiscall!(C_SUBINIT, u32, this + 0x21b4, ra(0x00E8B750));
        lf_checker_rt::callee_thiscall!(C_SUBINIT, u32, this + 0x218c, ra(0x00E8B76C));
        {
            let t = rgf(G_A00);
            let mut f = this + 0x21dc;
            let mut o = 0u32;
            while o < 4 {
                let mut k = 0u32;
                while k < 3 {
                    lf_checker_rt::callee_thiscall!(C_GAIN4, u32, f, t.to_bits(), t.to_bits(), 0u32, F_ONE);
                    f += 0x1c;
                    k += 1;
                }
                o += 1;
            }
        }

        // ---- global-picked voices plus two gain/filter slots ----
        {
            let mut a = this + 0x2114;
            let mut n = 0u32;
            while n < 3 {
                lf_checker_rt::callee_thiscall!(C_SUBINIT, u32, a, rg32(G_ARR + n * 4));
                a += 0x28;
                n += 1;
            }
        }
    // ---- R9: 5 field stores ----
    w32(this + 0x233C, 0x0);
    w32(this + 0x2334, 0x0);
    w32(this + 0x2338, 0x0);
    w32(this + 0x2330, 0x0);
    w32(this + 0x2740, 0x0);

        lf_checker_rt::callee_thiscall!(C_SUBINIT, u32, this + 0x2760, ra(0x00E8B784));
        {
            let t = rgf(G_A30);
            lf_checker_rt::callee_thiscall!(C_GAIN4, u32, this + 0x2744, t.to_bits(), t.to_bits(), 0u32, F_ONE);
        }
        {
            let a0 = rgf(G_A38);
            let a1 = rgf(G_A3C);
            lf_checker_rt::callee_thiscall!(C_GAIN4, u32, this + 0x29bc, a0.to_bits(), a1.to_bits(), 0u32, F_ONE);
        }

        // ---- repeat fill plus two 4-slot runs ----
        {
            let mut i = 0u32;
            while i < 0x51 {
                w32(this + 0x2788 + i * 4, 0);
                i += 1;
            }
            let t = rgf(G_A30);
            let mut s = this + 0x28cc;
            let mut f = this + 0x293c;
            let mut k = 0u32;
            while k < 4 {
                lf_checker_rt::callee_thiscall!(C_GAIN4, u32, s, t.to_bits(), t.to_bits(), 0u32, F_ONE);
                s += 0x1c;
                w32(f, 0);
                f += 4;
                k += 1;
            }
            s = this + 0x294c;
            f = this + 0x29ec;
            k = 0;
            while k < 4 {
                lf_checker_rt::callee_thiscall!(C_GAIN4, u32, s, t.to_bits(), t.to_bits(), 0u32, F_ONE);
                s += 0x1c;
                w32(f - 0x10, 0);
                w32(f, 0);
                w32(f + 0x10, 0);
                f += 4;
                k += 1;
            }
        }

        // ---- tail: voice, slots, random table, buffer, guard ----
        lf_checker_rt::callee_thiscall!(C_GAIN4, u32, this + 0x2a10, 0x42200000u32, 0x42200000u32, 0x44fa0000u32, 0x46bb8000u32);
        {
            let v: f32 = lf_checker_rt::callee_thiscall!(C_F32P, f32, this + 0x2a10, 0x46bb8000u32, rg32(G_ID));
            wf(this + 0x2a2c, v);
        }
    // ---- R11: 20 field stores ----
    w32(this + 0x3004, 0x0);
    w8(this + 0x3000, 0x0);
    w8(this + 0x3024, 0x0);
    w16(this + 0x3025, 0x0);
    w8(this + 0x3027, 0x0);
    w32(this + 0x3028, 0x0);
    w32(this + 0x1E74, 0x0);
    w32(this + 0x1E7C, 0x0);
    w32(this + 0x304C, 0x0);
    w32(this + 0x31D8, 0xFFFFFFFF);
    w32(this + 0x29D8, 0x0);
    w32(this + 0x2D30, 0x0);
    w32(this + 0x31D4, 0x80001);
    w32(this + 0x2FE4, 0xC2C80000);
    w8(this + 0x2FA0, 0x0);
    w32(this + 0x2A0C, F_ONE);
    w8(this + 0x31DC, 0x0);
    w16(this + 0x3210, 0x0);
        w32(this + 0x3048, rg32(G_H));
        {
            let u = mul(rgf(G_A44), rgf(G_RATE));
            lf_checker_rt::callee_thiscall!(C_GAIN4, u32, this + 0x3008, u.to_bits(), u.to_bits(), 0u32, F_ONE);
        }
        {
            let v = mul(rgf(G_A48), rgf(G_RATE));
            lf_checker_rt::callee_thiscall!(C_GAIN4, u32, this + 0x302c, v.to_bits(), v.to_bits(), 0u32, F_ONE);
        }
        {
            let mut f = this + 0x2a30;
            let mut k = 0u32;
            while k < 0x40 {
                w32(f + 0x100, 0);
                w32(f, 0);
                let r: u32 = lf_checker_rt::callee_cdecl!(C_RNG, u32, 0x1388u32, 0x7530u32);
                w32(f + 0x200, r);
                f += 4;
                k += 1;
            }
        }
        lf_checker_rt::callee_cdecl!(C_DEL, u32, r32(this + 0x31d0));
        // The multiply-overflow dance always yields size 4 (1 * 4 cannot
        // overflow), and the freshly stored id word always reads back 1,
        // so the fill length below is always 4.
        let fresh: u32 = lf_checker_rt::callee_cdecl!(C_NEW, u32, 4u32);
        w32(this + 0x31d0, fresh);
        let kept: u32 = lf_checker_rt::callee_cdecl!(C_FILL, u32, fresh, 0u32, 4u32);
        lf_checker_rt::callee_cdecl!(C_COOKIE, u32, );
        kept
    }
});
