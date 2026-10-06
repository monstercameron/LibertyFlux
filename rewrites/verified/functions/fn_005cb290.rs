// original: 0x005cb290 HUD_WEAPON_DOT

/// Build the per-frame input-event table: append one event per control, then
/// tag each event object.
///
/// No stack arguments (cdecl); both callers ignore `eax`, so the rewrite
/// returns 0. The body is a straight-line sequence of about thirty event
/// appends (each taking a name string, a kind, an address pair, a flag and
/// two parameter words, returning the event index, which is stored to its
/// own global), each followed by field writes to the looked-up event object
/// (`+0x34` mode, `+0x38`/`+0x04`/`+0x5a`/`+0x60` flags, `+0x14` tag) and, for
/// some events, a setter call with a constant. Two float blocks compare the
/// current thread id (read through the import slot, twice each) against a
/// stored id, divide the selected pair, and store a constant when 1.0
/// strictly exceeds the quotient. A six-word thread-local header is filled
/// from the platform test before the first setter call. Two loops append
/// indexed events (four iterations each, addresses derived from the loop
/// cursor), and a final conditional block runs only when a mode word equals
/// 2 and its index slot is still zero.
///
/// The platform test is `(byte_A == 0x6a) || (byte_B != 0)` (unsigned byte
/// comparison, `byte_A` tested first). Loop bounds use signed comparison.
/// Float order is the original's: both words converted with `cvtdq2ps`,
/// then `divss`, then `comiss` against 1.0 with the store skipped on
/// below-or-equal (unordered included, matching Rust's `>`).
lf_checker_rt::export!(cdecl, rw_005cb290() -> u32 {
    unsafe {
        const EVT_TABLE: u32 = 0x118E7F8;
        const PLAT_A: u32 = 0x116C250;
        const PLAT_B: u32 = 0x116C253;
        const PLAT_CONSOLE: u8 = 0x6a;
        const IAT_TID: u32 = 0xE731AC;
        const G_TID: u32 = 0x110DD14;
        const G_SEL_A0: u32 = 0x105C880;
        const G_SEL_A1: u32 = 0x105C87C;
        const G_SEL_B0: u32 = 0x105C884;
        const G_SEL_B1: u32 = 0x105C888;
        const ONE_BITS: u32 = 0x3F800000;
        const C_APPEND_A: u32 = 1;
        const C_APPEND_B: u32 = 2;
        const C_APPEND_C: u32 = 3;
        const C_STRSET: u32 = 4;
        const C_SETV: u32 = 5;
        const C_FMT: u32 = 6;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rel(a: u32) -> u32 {
            lf_checker_rt::relocated(a)
        }
        #[inline(always)]
        unsafe fn ev(idx: u32) -> u32 {
            unsafe {
                lf_checker_rt::global::<u32>(EVT_TABLE)
                    .add(idx as usize)
                    .read_unaligned()
            }
        }
        #[inline(always)]
        unsafe fn append(
            id: u32,
            a1: u32,
            a2: u32,
            a3: u32,
            a4: u32,
            a5: u32,
            a6: u32,
            a7: u32,
        ) -> u32 {
            unsafe { lf_checker_rt::callee_cdecl!(id, u32, a1, a2, a3, a4, a5, a6, a7) }
        }
        #[inline(always)]
        unsafe fn plat_other() -> bool {
            unsafe {
                rd8(lf_checker_rt::relocated(PLAT_A)) == PLAT_CONSOLE
                    || rd8(lf_checker_rt::relocated(PLAT_B)) != 0
            }
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        /// One float block: select two constants by comparing two thread-id
        /// reads against the stored id, divide, store `bits` to `store`
        /// when 1.0 strictly exceeds the quotient.
        #[inline(always)]
        unsafe fn ratio_store(
            tid: extern "cdecl" fn() -> u32,
            store: u32,
            bits: u32,
        ) {
            unsafe {
                let g = rd32(rel(G_TID));
                let t1 = tid();
                let t2 = tid();
                let s = if t1 == g {
                    rd32(rel(G_SEL_A1))
                } else {
                    rd32(rel(G_SEL_A0))
                };
                let c = if t2 == g {
                    rd32(rel(G_SEL_B1))
                } else {
                    rd32(rel(G_SEL_B0))
                };
                // Signed conversion, exactly the original's cvtdq2ps pair.
                let q = div((c as i32) as f32, (s as i32) as f32);
                if f32::from_bits(ONE_BITS) > q {
                    wr32(rel(store), bits);
                }
            }
        }

        // Blocks 1-6.
        let idx = append(
            C_APPEND_A,
            rel(0xF8F310),
            5,
            rel(0x118EC58),
            rel(0x118EC60),
            1,
            rd32(rel(0x118EC6C)),
            rd32(rel(0x118EC68)),
        );
        wr32(rel(0x118EC54), idx);
        let o = ev(idx);
        wr32(o.wrapping_add(0x34), 4);
        wr8(o.wrapping_add(0x38), 0);
        wr8(o.wrapping_add(0x5a), 0);
        wr8(o.wrapping_add(4), 1);
        let idx = append(
            C_APPEND_A,
            rel(0xF8F298),
            5,
            rel(0x118EC74),
            rel(0x118EC7C),
            1,
            rd32(rel(0x118EC88)),
            rd32(rel(0x118EC84)),
        );
        wr32(rel(0x118EC70), idx);
        let o = ev(idx);
        wr32(o.wrapping_add(0x34), 4);
        wr8(o.wrapping_add(0x38), 0);
        wr8(o.wrapping_add(4), 1);
        let idx = append(
            C_APPEND_A,
            rel(0xF8F2A4),
            5,
            rel(0x118ECE4),
            rel(0x118ECEC),
            1,
            rd32(rel(0x118ECF8)),
            rd32(rel(0x118ECF4)),
        );
        wr32(rel(0x118ECE0), idx);
        let o = ev(idx);
        wr32(o.wrapping_add(0x34), 4);
        wr8(o.wrapping_add(0x38), 0);
        wr8(o.wrapping_add(4), 1);
        let idx = append(
            C_APPEND_A,
            rel(0xF8F26C),
            5,
            rel(0x118ED00),
            rel(0x118ED08),
            1,
            rd32(rel(0x118ED14)),
            rd32(rel(0x118ED10)),
        );
        wr32(rel(0x118ECFC), idx);
        let o = ev(idx);
        wr32(o.wrapping_add(0x34), 4);
        wr8(o.wrapping_add(0x38), 0);
        wr8(o.wrapping_add(4), 1);
        let idx = append(
            C_APPEND_A,
            rel(0xF8F284),
            5,
            rel(0x118ECAC),
            rel(0x118ECB4),
            1,
            rd32(rel(0x118ECC0)),
            rd32(rel(0x118ECBC)),
        );
        wr32(rel(0x118ECA8), idx);
        let o = ev(idx);
        wr32(o.wrapping_add(0x34), 4);
        wr8(o.wrapping_add(4), 1);
        wr8(o.wrapping_add(0x38), 0);
        let idx = append(
            C_APPEND_A,
            rel(0xF8F2D0),
            5,
            rel(0x118ECC8),
            rel(0x118ECD0),
            1,
            rd32(rel(0x118ECDC)),
            rd32(rel(0x118ECD8)),
        );
        wr32(rel(0x118ECC4), idx);
        let o = ev(idx);
        wr32(o.wrapping_add(0x34), 4);
        wr8(o.wrapping_add(0x38), 0);
        // Thread-local header: six words, then a zero word.
        let tls0 = lf_checker_rt::tls_slot(0);
        let hv: u16 = if plat_other() { 0x5e } else { 0xfc };
        let mut k = 0u32;
        while k < 6 {
            wr16(tls0.wrapping_add(0x78 + k * 2), hv);
            k += 1;
        }
        wr16(tls0.wrapping_add(0x84), 0);
        let o6 = ev(rd32(rel(0x118ECC4)));
        lf_checker_rt::callee_thiscall!(C_STRSET, u32, o6, tls0.wrapping_add(0x78));
        wr8(ev(rd32(rel(0x118ECC4))).wrapping_add(4), 1);
        // Blocks 7-8.
        let idx = append(
            C_APPEND_A,
            rel(0xF8F2E0),
            5,
            rel(0x118ED1C),
            rel(0x118ED24),
            1,
            rd32(rel(0x118ED30)),
            rd32(rel(0x118ED2C)),
        );
        wr32(rel(0x118ED18), idx);
        let o = ev(idx);
        wr32(o.wrapping_add(0x34), 4);
        wr8(o.wrapping_add(4), 1);
        wr8(o.wrapping_add(0x38), 0);
        let idx = append(
            C_APPEND_A,
            rel(0xF8F2B0),
            2,
            rel(0x118ED38),
            rel(0x118ED40),
            1,
            rd32(rel(0x118ED4C)),
            rd32(rel(0x118ED48)),
        );
        wr32(rel(0x118ED34), idx);
        wr8(ev(idx).wrapping_add(4), 1);
        // First ratio block; the import slot is read once for both blocks.
        let tid: extern "cdecl" fn() -> u32 =
            unsafe { core::mem::transmute(rd32(rel(IAT_TID)) as usize) };
        ratio_store(tid, 0x118ED58, 0x3F651EB8);
        // Blocks 9-10.
        let idx = append(
            C_APPEND_A,
            rel(0xF8F2C0),
            5,
            rel(0x118ED54),
            rel(0x118ED5C),
            1,
            rd32(rel(0x118ED68)),
            rd32(rel(0x118ED64)),
        );
        wr32(rel(0x118ED50), idx);
        wr32(ev(idx).wrapping_add(0x34), if plat_other() { 2 } else { 6 });
        let idx = append(
            C_APPEND_A,
            rel(0xF8F1EC),
            5,
            rel(0x118ED70),
            rel(0x118ED78),
            1,
            rd32(rel(0x118ED84)),
            rd32(rel(0x118ED80)),
        );
        wr32(rel(0x118ED6C), idx);
        wr32(ev(idx).wrapping_add(0x34), if plat_other() { 2 } else { 6 });
        ratio_store(tid, 0x118ED90, 0x3F600000);
        // Blocks 11-13.
        let idx = append(
            C_APPEND_A,
            rel(0xF8F1FC),
            5,
            rel(0x118ED8C),
            rel(0x118ED94),
            1,
            rd32(rel(0x118EDA0)),
            rd32(rel(0x118ED9C)),
        );
        wr32(rel(0x118ED88), idx);
        wr32(ev(idx).wrapping_add(0x34), 6);
        let idx = append(
            C_APPEND_A,
            rel(0xF8F1B8),
            2,
            rel(0x118EDA8),
            rel(0x118EDB0),
            1,
            rd32(rel(0x118EDBC)),
            rd32(rel(0x118EDB8)),
        );
        wr32(rel(0x118EDA4), idx);
        let idx = append(
            C_APPEND_A,
            rel(0xF8F1D0),
            5,
            rel(0x118EDC4),
            rel(0x118EDCC),
            0,
            rd32(rel(0x118EDD8)),
            rd32(rel(0x118EDD4)),
        );
        wr32(rel(0x118EDC0), idx);
        wr32(ev(idx).wrapping_add(0x34), if plat_other() { 2 } else { 0 });
        // Blocks 14-16.
        let idx = append(
            C_APPEND_A,
            rel(0xF8F23C),
            0x0c,
            rel(0x118EDE0),
            rel(0x118EDE8),
            0,
            rd32(rel(0x118EDF4)),
            rd32(rel(0x118EDF0)),
        );
        wr32(rel(0x118EDDC), idx);
        let idx = append(
            C_APPEND_A,
            rel(0xF8F254),
            2,
            rel(0x118EDFC),
            rel(0x118EE04),
            0,
            rd32(rel(0x118EE10)),
            rd32(rel(0x118EE0C)),
        );
        wr32(rel(0x118EDF8), idx);
        lf_checker_rt::callee_thiscall!(C_SETV, u32, ev(idx), rel(0x19D3AB4));
        let idx = append(
            C_APPEND_A,
            rel(0xF8F210),
            2,
            rel(0x118EE18),
            rel(0x118EE20),
            0,
            rd32(rel(0x118EE2C)),
            rd32(rel(0x118EE28)),
        );
        wr32(rel(0x118EE14), idx);
        lf_checker_rt::callee_thiscall!(C_SETV, u32, ev(idx), rel(0x19D3ACC));
        // Blocks 17-21.
        let idx = append(
            C_APPEND_B,
            rel(0xF8F224),
            3,
            rel(0x118EE34),
            rel(0x118EE3C),
            0,
            rd32(rel(0x118EE48)),
            rd32(rel(0x118EE44)),
        );
        wr32(rel(0x118EE30), idx);
        lf_checker_rt::callee_thiscall!(C_SETV, u32, ev(idx), rel(0x19D3A98));
        wr32(ev(idx).wrapping_add(0x14), 7);
        let idx = append(
            C_APPEND_B,
            rel(0xF8F118),
            4,
            rel(0x118EE50),
            rel(0x118EE58),
            0,
            rd32(rel(0x118EE64)),
            rd32(rel(0x118EE60)),
        );
        wr32(rel(0x118EE4C), idx);
        lf_checker_rt::callee_thiscall!(C_SETV, u32, ev(idx), rel(0x19D3A9C));
        wr32(ev(idx).wrapping_add(0x14), 7);
        let idx = append(
            C_APPEND_B,
            rel(0xF8F134),
            4,
            rel(0x118EE6C),
            rel(0x118EE74),
            0,
            rd32(rel(0x118EE80)),
            rd32(rel(0x118EE7C)),
        );
        wr32(rel(0x118EE68), idx);
        lf_checker_rt::callee_thiscall!(C_SETV, u32, ev(idx), rel(0x19D3A9C));
        wr32(ev(idx).wrapping_add(0x14), 7);
        let idx = append(
            C_APPEND_B,
            rel(0xF8F0F4),
            1,
            rel(0x118EE88),
            rel(0x118EE90),
            0,
            rd32(rel(0x118EE9C)),
            rd32(rel(0x118EE98)),
        );
        wr32(rel(0x118EE84), idx);
        wr32(ev(idx).wrapping_add(0x14), 7);
        let idx = append(
            C_APPEND_B,
            rel(0xF8F104),
            5,
            rel(0x118EEA4),
            rel(0x118EEAC),
            1,
            rd32(rel(0x118EEB8)),
            rd32(rel(0x118EEB4)),
        );
        wr32(rel(0x118EEA0), idx);
        let o = ev(idx);
        wr8(o.wrapping_add(4), 1);
        wr32(o.wrapping_add(0x34), if plat_other() { 2 } else { 0 });
        // Indexed loop 1.
        let mut edi = 0x33u32;
        let mut esi = 0x118EED4u32;
        while (esi as i32) <= (0x118EF7Cu32 as i32) {
            lf_checker_rt::callee_cdecl!(
                C_FMT,
                u32,
                tls0.wrapping_add(0x4c8),
                rel(0xF8F17C),
                edi.wrapping_sub(0x32)
            );
            let idx = append(
                C_APPEND_B,
                tls0.wrapping_add(0x4c8),
                5,
                rel(esi.wrapping_sub(0x14)),
                rel(esi.wrapping_sub(0x0c)),
                1,
                rd32(rel(esi)),
                rd32(rel(esi.wrapping_sub(4))),
            );
            wr32(rel(esi.wrapping_sub(0x18)), idx);
            let o = ev(idx);
            wr8(o.wrapping_add(4), 1);
            wr32(o.wrapping_add(0x34), if plat_other() { 2 } else { 0 });
            esi = esi.wrapping_add(0x38);
            edi = edi.wrapping_add(2);
        }
        // Indexed loop 2.
        edi = 0x34;
        esi = 0x118EEF0;
        while (esi as i32) <= (0x118EF98u32 as i32) {
            lf_checker_rt::callee_cdecl!(
                C_FMT,
                u32,
                tls0.wrapping_add(0x4c8),
                rel(0xF8F198),
                edi.wrapping_sub(0x33)
            );
            let idx = append(
                C_APPEND_B,
                tls0.wrapping_add(0x4c8),
                9,
                rel(esi.wrapping_sub(0x14)),
                rel(esi.wrapping_sub(0x0c)),
                1,
                rd32(rel(esi)),
                rd32(rel(esi.wrapping_sub(4))),
            );
            wr32(rel(esi.wrapping_sub(0x18)), idx);
            wr8(ev(idx).wrapping_add(4), 1);
            esi = esi.wrapping_add(0x38);
            edi = edi.wrapping_add(2);
        }
        // Blocks 22-27.
        let idx = append(
            C_APPEND_C,
            rel(0xF8F150),
            2,
            rel(0x118EC90),
            rel(0x118EC98),
            0,
            rd32(rel(0x118ECA4)),
            0xfa,
        );
        wr32(rel(0x118EC8C), idx);
        lf_checker_rt::callee_thiscall!(C_SETV, u32, ev(idx), rel(0x19D3B08));
        wr32(ev(idx).wrapping_add(0x14), 7);
        let idx = append(
            C_APPEND_C,
            rel(0xF8F164),
            0x0d,
            rel(0x118EFF4),
            rel(0x118EFFC),
            0,
            rd32(rel(0x118F008)),
            rd32(rel(0x118F004)),
        );
        wr32(rel(0x118EFF0), idx);
        let idx = append(
            C_APPEND_C,
            rel(0xF8F384),
            2,
            rel(0x118F010),
            rel(0x118F018),
            0,
            rd32(rel(0x118F024)),
            rd32(rel(0x118F020)),
        );
        wr32(rel(0x118F00C), idx);
        let idx = append(
            C_APPEND_C,
            rel(0xF8F39C),
            3,
            rel(0x118EFA0),
            rel(0x118EFA8),
            0,
            rd32(rel(0x118EFB4)),
            rd32(rel(0x118EFB0)),
        );
        wr32(rel(0x118EF9C), idx);
        let o = ev(idx);
        wr32(o.wrapping_add(0x14), 6);
        wr8(o.wrapping_add(0x60), 4);
        let idx = append(
            C_APPEND_C,
            rel(0xF8F354),
            2,
            rel(0x118EFBC),
            rel(0x118EFC4),
            0,
            rd32(rel(0x118EFD0)),
            rd32(rel(0x118EFCC)),
        );
        wr32(rel(0x118EFB8), idx);
        lf_checker_rt::callee_thiscall!(C_SETV, u32, ev(idx), rel(0x19D3AAC));
        wr32(ev(idx).wrapping_add(0x14), 1);
        let idx = append(
            C_APPEND_C,
            rel(0xF8F36C),
            2,
            rel(0x118EFD8),
            rel(0x118EFE0),
            0,
            rd32(rel(0x118EFEC)),
            rd32(rel(0x118EFE8)),
        );
        wr32(rel(0x118EFD4), idx);
        lf_checker_rt::callee_thiscall!(C_SETV, u32, ev(idx), rel(0x19D3AB0));
        wr32(ev(idx).wrapping_add(0x14), 1);
        // Conditional tail block.
        if rd32(rel(0x11D6FD4)) == 2 && rd32(rel(0x118F028)) == 0 {
            let idx = append(
                C_APPEND_C,
                rel(0xF8F3CC),
                2,
                rel(0x118F02C),
                rel(0x118F034),
                2,
                rd32(rel(0x118F040)),
                rd32(rel(0x118F03C)),
            );
            wr32(rel(0x118F028), idx);
            lf_checker_rt::callee_thiscall!(C_SETV, u32, ev(idx), rel(0x19D3B20));
            wr32(ev(idx).wrapping_add(0x14), 1);
        }
        0
    }
});
