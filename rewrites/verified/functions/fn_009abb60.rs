// original: 0x009ABB60 audWeatherAudioEntity::vf1

/// Weather-audio initialiser: zeroes the entity's state, formats and loads
/// every wind/ambient curve by name, and initialises the auxiliary tables.
///
/// `this` is the weather audio entity (no stack arguments). The work falls
/// into three phases. Phase 1 clears the header words at +0x8..+0x20, then
/// runs ten identical iterations: six scratch buffers are zeroed, six wind
/// curve names (`WIND_VOLUME_BASS` and siblings, each with a `%s` suffix)
/// are formatted with successive words of the global table at 0x10390d8
/// into six more buffers, and six curve members (three near +0x88, three
/// near +0x510, all stepping 0x78 per iteration) each load one formatted
/// name. Phase 2 initialises the scalar blocks: a nine-word descriptor at
/// +0x20, the +0x9e8/+0x9c0/+0xa04/+0xc24/+0xc4c members, thirty-one cleared
/// words from +0xba8, the float constants at +0xc78/+0xd8c/+0xd54/+0xd88
/// (15.0, 1.0 and parts of 0.1/1000.0-scaled pairs), twelve cleared words
/// around +0xd20..+0xd48, the +0xd5c curve with its divisor triple
/// (85.0, 85.0, 70.0, 70.0, 0.4), a 4x3 grid of member initialisations from
/// +0xa54, the +0xfb0/+0xc7c/+0xccc/+0xcf4 members, a randomised value in
/// +0xda8 from the range call over (0, 360.0), sixteen member
/// initialisations from +0x11e0 with their flag bytes at +0x1190, the
/// +0xca4 curve, 128 cleared words from +0xdac/+0xeac, and four pairs of
/// member initialisations from +0x13b4/+0x1444 (storing -100.0 sentinels).
/// Phase 3 loads eleven named ambient curves into the members from +0x14b4
/// (with the last two, +0x1654 before +0x162c, swapped relative to layout
/// order), zeroes +0x15a4..+0x15b0, loads six curves into global audio
/// objects, and runs the final registration pass whose integer answer is
/// the return value.
///
/// Callee answers are ignored except the range call's (stored) and the
/// final call's (returned); the stack-cookie check is omitted (it guards
/// the original's frame, not behaviour). All float arithmetic keeps the
/// original's operand order.
///
/// Original: 0x009ABB60 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_009abb60(this: u32) -> u32 {
    unsafe {
        const MEMSET_LIKE: u32 = 1;
        const FORMAT_NAME: u32 = 2;
        const LOAD_LOOP: u32 = 3;
        const LOAD_FIXED: u32 = 4;
        const INIT_MEMBER4: u32 = 5;
        const INIT_DESC9: u32 = 6;
        const INIT_PAIR2: u32 = 7;
        const RANGE_RANDOM: u32 = 8;
        const REGISTER: u32 = 9;
        const COOKIE_CHECK: u32 = 10;
        const NAME_TABLE: u32 = 0x010390D8;
        const FMTS: [u32; 6] = [
            0x00E927E8, 0x00E927FC, 0x00E92810, 0x00E92828, 0x00E92840, 0x00E92858,
        ];
        const Q100: u32 = 0x01039108;
        const Q300: u32 = 0x01039120;
        const Q1000: u32 = 0x0103911C;
        const Q001: u32 = 0x00FE86B4;

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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn glob(va: u32) -> f32 {
            unsafe { rdf(lf_checker_rt::relocated(va)) }
        }

        // Phase 1: header clear + ten formatting iterations.
        wr32(this.wrapping_add(0x08), 0);
        wr32(this.wrapping_add(0x0c), 0);
        wr32(this.wrapping_add(0x10), 0);
        wr32(this.wrapping_add(0x14), 0);
        wr32(this.wrapping_add(0x18), 0);
        wr32(this.wrapping_add(0x1c), 0);
        // Eighteen 255-byte scratch buffers (six channels by three stages);
        // their addresses are skipped by the contract and their contents are
        // never observed (every consumer is an intercepted callee).
        let mut scratch = [[0u8; 256]; 18];
        for k in 0..10u32 {
            for c in 0..6usize {
                let _: u32 = lf_checker_rt::callee_cdecl!(
                    MEMSET_LIKE,
                    u32,
                    scratch[c * 3].as_mut_ptr() as u32,
                    0u32,
                    0xFFu32
                );
            }
            let word = rd32(lf_checker_rt::relocated(NAME_TABLE).wrapping_add(k.wrapping_mul(4)));
            for c in 0..6usize {
                let _: u32 = lf_checker_rt::callee_cdecl!(
                    FORMAT_NAME,
                    u32,
                    scratch[c * 3 + 1].as_mut_ptr() as u32,
                    0xFFu32,
                    lf_checker_rt::relocated(FMTS[c]),
                    word
                );
            }
            let base = this.wrapping_add(0x88).wrapping_add(k.wrapping_mul(0x78));
            let members = [
                base.wrapping_sub(0x28),
                base,
                base.wrapping_add(0x28),
                base.wrapping_add(0x488),
                base.wrapping_add(0x4b0),
                base.wrapping_add(0x4d8),
            ];
            for c in 0..6usize {
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    LOAD_LOOP,
                    u32,
                    members[c],
                    scratch[c * 3 + 2].as_mut_ptr() as u32
                );
            }
        }

        // Phase 2: scalar blocks and member grids.
        let q100 = glob(Q100);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            INIT_DESC9, u32, this.wrapping_add(0x20), 0x3DCCCCCDu32, 0x3DCCCCCDu32,
            0u32, 0x3F800000u32, 0u32, 0x3F800000u32, q100.to_bits(), 0u32, 0u32
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(
            INIT_MEMBER4, u32, this.wrapping_add(0x9e8), 0x3A83126Fu32,
            0x3A83126Fu32, 0u32, 0x3F800000u32
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(
            LOAD_FIXED, u32, this.wrapping_add(0x9c0),
            lf_checker_rt::relocated(0x00E92874)
        );
        for k in 0..31u32 {
            wr32(this.wrapping_add(0xba8).wrapping_add(k.wrapping_mul(4)), 0);
        }
        wr32(this.wrapping_add(0xc78), 0x3B449BA6);
        wr32(this.wrapping_add(0xc74), 0);
        wr32(this.wrapping_add(0xd8c), 0x41700000);
        wr32(this.wrapping_add(0xd54), 0x3F800000);
        wr32(this.wrapping_add(0xd88), 0x41700000);
        wr32(this.wrapping_add(0xa20), 0);
        let xa = mul(glob(Q300), glob(Q001));
        let xb = mul(glob(Q1000), glob(Q001));
        let _: u32 = lf_checker_rt::callee_thiscall!(
            INIT_MEMBER4, u32, this.wrapping_add(0xa04), xb.to_bits(), xa.to_bits(),
            0u32, 0x3F800000u32
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(
            LOAD_FIXED, u32, this.wrapping_add(0xc24),
            lf_checker_rt::relocated(0x00E92894)
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(
            LOAD_FIXED, u32, this.wrapping_add(0xc4c),
            lf_checker_rt::relocated(0x00E928A8)
        );
        for off in [0xd18u32, 0xd14, 0xd10, 0xd28, 0xd24, 0xd20, 0xd38, 0xd34, 0xd30, 0xd48, 0xd44, 0xd40] {
            wr32(this.wrapping_add(off), 0);
        }
        wr32(this.wrapping_add(0xd50), 0);
        wr32(this.wrapping_add(0xd90), 0x42AA0000);
        wr32(this.wrapping_add(0xd94), 0x42AA0000);
        wr32(this.wrapping_add(0xd98), 0x428C0000);
        wr32(this.wrapping_add(0xd9c), 0x428C0000);
        wr32(this.wrapping_add(0xda0), 0x3ECCCCCD);
        wr32(this.wrapping_add(0xd58), 0);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            LOAD_FIXED, u32, this.wrapping_add(0xd5c),
            lf_checker_rt::relocated(0x00E928BC)
        );
        (this.wrapping_add(0xd84) as *mut u8).write(0);
        for j in 0..12u32 {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                INIT_MEMBER4, u32,
                this.wrapping_add(0xa54).wrapping_add(j.wrapping_mul(0x1c)),
                0x3A83126Fu32, 0x3A83126Fu32, 0u32, 0x3F800000u32
            );
            wr32(this.wrapping_add(0xa24).wrapping_add(j.wrapping_mul(4)), 0);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(
            INIT_MEMBER4, u32, this.wrapping_add(0xfb0), 0x3A83126Fu32,
            0x3A83126Fu32, 0u32, 0x3F800000u32
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(
            LOAD_FIXED, u32, this.wrapping_add(0xc7c),
            lf_checker_rt::relocated(0x00E928D8)
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(
            LOAD_FIXED, u32, this.wrapping_add(0xccc),
            lf_checker_rt::relocated(0x00E928E8)
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(
            INIT_MEMBER4, u32, this.wrapping_add(0xcf4), 0x3C23D70Au32,
            0x3C23D70Au32, 0u32, 0x41C80000u32
        );
        wr32(this.wrapping_add(0xda4), 0);
        let frand: f32 =
            lf_checker_rt::callee_cdecl!(RANGE_RANDOM, f32, 0u32, 0x43B38000u32);
        wr32(this.wrapping_add(0xda8), frand.to_bits());
        for k in 0..16u32 {
            wr32(this.wrapping_add(0x10d0).wrapping_add(k.wrapping_mul(4)), 0);
            wr32(this.wrapping_add(0x1150).wrapping_add(k.wrapping_mul(4)), 0);
            (this.wrapping_add(0x1190).wrapping_add(k) as *mut u8).write(1);
            let edi = this.wrapping_add(0xfd4).wrapping_add(k.wrapping_mul(0x10));
            wr32(edi.wrapping_add(4), 0);
            wr32(edi, 0);
            wr32(edi.wrapping_sub(4), 0);
            let _: u32 = lf_checker_rt::callee_thiscall!(
                INIT_MEMBER4, u32,
                this.wrapping_add(0x11e0).wrapping_add(k.wrapping_mul(0x1c)),
                0x3B03126Fu32, 0x3B03126Fu32, 0u32, 0x3F800000u32
            );
            wr32(this.wrapping_add(0x11a0).wrapping_add(k.wrapping_mul(4)), 0);
        }
        wr32(this.wrapping_add(0x13a0), 0);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            LOAD_FIXED, u32, this.wrapping_add(0xca4),
            lf_checker_rt::relocated(0x00E92908)
        );
        for k in 0..64u32 {
            wr32(this.wrapping_add(0xeac).wrapping_add(k.wrapping_mul(4)), 0);
            wr32(this.wrapping_add(0xdac).wrapping_add(k.wrapping_mul(4)), 0);
        }
        for k in 0..4u32 {
            wr32(this.wrapping_add(0x13a4).wrapping_add(k.wrapping_mul(4)), 0);
            let _: u32 = lf_checker_rt::callee_thiscall!(
                INIT_PAIR2, u32,
                this.wrapping_add(0x13b4).wrapping_add(k.wrapping_mul(0x1c)),
                0x3A03126Fu32, 0x3A03126Fu32
            );
            wr32(this.wrapping_add(0x1424).wrapping_add(k.wrapping_mul(4)), 0xC2C80000);
            wr32(this.wrapping_add(0x1434).wrapping_add(k.wrapping_mul(4)), 0);
            let _: u32 = lf_checker_rt::callee_thiscall!(
                INIT_PAIR2, u32,
                this.wrapping_add(0x1444).wrapping_add(k.wrapping_mul(0x1c)),
                0x3A03126Fu32, 0x3A03126Fu32
            );
        }

        // Phase 3: ambient curves, global curves, registration.
        const FIXED_CURVES: [(u32, u32); 11] = [
            (0x14b4, 0x00E9291C), (0x14dc, 0x00E92944), (0x1504, 0x00E9296C),
            (0x152c, 0x00E92990), (0x1554, 0x00E929B4), (0x157c, 0x00E929D8),
            (0x15b4, 0x00E929FC), (0x15dc, 0x00E92A14), (0x1604, 0x00E92A30),
            (0x1654, 0x00E92A48), (0x162c, 0x00E92A64),
        ];
        for (off, name) in FIXED_CURVES {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                LOAD_FIXED, u32, this.wrapping_add(off),
                lf_checker_rt::relocated(name)
            );
        }
        wr32(this.wrapping_add(0x15a4), 0);
        wr32(this.wrapping_add(0x15a8), 0);
        wr32(this.wrapping_add(0x15ac), 0);
        wr32(this.wrapping_add(0x15b0), 0);
        const GLOBAL_CURVES: [(u32, u32); 6] = [
            (0x012891E0, 0x00E92A80), (0x0128A8DC, 0x00E92AA0),
            (0x01289208, 0x00E92AC0), (0x012891B8, 0x00E92AE0),
            (0x0128918C, 0x00E92AF4), (0x0128A8B0, 0x00E92B18),
        ];
        for (obj, name) in GLOBAL_CURVES {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                LOAD_FIXED, u32, lf_checker_rt::relocated(obj),
                lf_checker_rt::relocated(name)
            );
        }
        let r: u32 = lf_checker_rt::callee_thiscall!(REGISTER, u32, this);
        // Stack-cookie check omitted: with no frame of its own to guard, the
        // rewrite has nothing to verify; the stub preserves the answer.
        let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE_CHECK, u32,);
        r
    }
});
