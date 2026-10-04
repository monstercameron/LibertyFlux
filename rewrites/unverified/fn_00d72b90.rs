// original: 0x00d72b90 replay_overlay_update (proposed)

/// Refresh the replay overlay and repaint its seventeen channel strips.
///
/// `this` is the overlay object. Word at `+0x04` points at the inner state
/// (flag byte at `+0x1b`); the strip base is at `+0x64` (eight bytes per
/// strip, two float words each); words at `+0x3c`/`+0x40` select two special
/// strips; byte at `+0x48` gates the whole update.
///
/// Behaviour: stamp the manager record (reached through a global pointer)
/// with generation `0x3e`, then return early (leaving that stamp as the only
/// write) when the overlay is disabled, the inner flag is set, or the inner
/// probe callee answers zero. Otherwise run two query rounds (query, select,
/// fetch), notify the overlay twice through the sibling entry point, draw the
/// level meters, then walk strips 1..=17: query each strip's state, draw its
/// meters unless told to skip, and repaint the strip itself. Finishes with a
/// flush call. Returns the flush answer on the long path, the manager
/// pointer on the two gate paths, zero when the probe answers zero.
///
/// Original: 0x00d72b90 (thiscall, no stack words). The entry security-cookie
/// dance is invisible to the checker (below-entry stack is never compared),
/// so only the trailing cookie-check call is reproduced, with an uncompared
/// register argument. All seventeen strip calls and both notify calls take
/// `this` in ECX.
lf_checker_rt::export!(thiscall, rw_00d72b90(this: u32) -> u32 {
    unsafe {
        const MGR_PTR_G: u32 = 0x0118E868;
        const LEVEL_TABLE_G: u32 = 0x01056908;
        const GENERATION: u32 = 0x3e;
        const NESTED_THIS: u32 = 0x0116bff0;
        const INNER_PROBE: u32 = 1;
        const NET_QUERY: u32 = 2;
        const NET_SELECT: u32 = 3;
        const NET_FETCH: u32 = 4;
        const NET_COMMIT: u32 = 5;
        const OVERLAY_NOTIFY: u32 = 6;
        const METER_DRAW: u32 = 7;
        const LEVEL_DRAW: u32 = 8;
        const STRIP_STATE: u32 = 9;
        const STRIP_QUERY: u32 = 10;
        const STRIP_COMMIT: u32 = 11;
        const METER_MODE: u32 = 12;
        const STRIP_PAINT: u32 = 13;
        const OVERLAY_FLUSH: u32 = 14;
        const COOKIE_CHECK: u32 = 15;
        const INNER_STATE_OFF: u32 = 0x04;
        const INNER_FLAG_OFF: u32 = 0x1b;
        const FETCH_A_OFF: u32 = 0x1c;
        const FETCH_B_OFF: u32 = 0x2c;
        const SPECIAL_A_OFF: u32 = 0x3c;
        const SPECIAL_B_OFF: u32 = 0x40;
        const MGR_GEN_OFF: u32 = 0x44;
        const ENABLE_OFF: u32 = 0x48;
        const METER_LO_OFF: u32 = 0x5c;
        const METER_HI_OFF: u32 = 0x60;
        const STRIPS_OFF: u32 = 0x64;
        const STRIP_FIRST: u32 = 1;
        const STRIP_PAST: u32 = 0x12;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn cookie_check() {
            unsafe {
                let _: u32 = lf_checker_rt::callee_thiscall!(COOKIE_CHECK, u32, 0);
            }
        }

        let mgr = rd32(lf_checker_rt::global::<u32>(MGR_PTR_G) as u32);
        wr32(mgr.wrapping_add(MGR_GEN_OFF), GENERATION);
        if rd8(this.wrapping_add(ENABLE_OFF)) == 0 {
            cookie_check();
            return mgr;
        }
        let inner = rd32(this.wrapping_add(INNER_STATE_OFF));
        if rd8(inner.wrapping_add(INNER_FLAG_OFF)) != 0 {
            cookie_check();
            return mgr;
        }
        let probe: u32 = lf_checker_rt::callee_thiscall!(INNER_PROBE, u32, inner);
        if probe == 0 {
            cookie_check();
            return 0;
        }

        // Two query rounds: query, select, fetch, commit.
        let mut cell_a = 0u32;
        let _: u32 = lf_checker_rt::callee_cdecl!(
            NET_QUERY, u32,
            &mut cell_a as *mut u32 as u32, 2, 0xc8
        );
        let _: u32 = lf_checker_rt::callee_cdecl!(NET_SELECT, u32, 0, 1);
        let fetch_a = this.wrapping_add(FETCH_A_OFF);
        let _: u32 = lf_checker_rt::callee_cdecl!(
            NET_FETCH, u32,
            fetch_a, &mut cell_a as *mut u32 as u32
        );
        let _: u32 = lf_checker_rt::callee_cdecl!(NET_COMMIT, u32,);
        let mut cell_b = 0u32;
        let q: u32 = lf_checker_rt::callee_cdecl!(
            NET_QUERY, u32,
            &mut cell_b as *mut u32 as u32, 0x3d, 0xff
        );
        cell_b = rd32(q);
        let _: u32 = lf_checker_rt::callee_cdecl!(NET_SELECT, u32, 0, 1);
        let fetch_b = this.wrapping_add(FETCH_B_OFF);
        let _: u32 = lf_checker_rt::callee_cdecl!(
            NET_FETCH, u32,
            fetch_b, &mut cell_b as *mut u32 as u32
        );
        let _: u32 = lf_checker_rt::callee_cdecl!(NET_COMMIT, u32,);

        // Notify + overall meters, twice (once per level channel).
        let table = lf_checker_rt::global::<u32>(LEVEL_TABLE_G) as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(OVERLAY_NOTIFY, u32, this, 0);
        let nested = lf_checker_rt::relocated(NESTED_THIS);
        let lv0: u32 =
            lf_checker_rt::callee_thiscall!(METER_DRAW, u32, nested, rd32(table));
        let _: u32 = lf_checker_rt::callee_cdecl!(
            LEVEL_DRAW, u32,
            rd32(this.wrapping_add(METER_LO_OFF)),
            rd32(this.wrapping_add(METER_HI_OFF)),
            lv0, 0xffff_ffff, 0xffff_ffff
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(OVERLAY_NOTIFY, u32, this, 1);

        // Seventeen strips.
        let special_a = rd32(this.wrapping_add(SPECIAL_A_OFF));
        let special_b = rd32(this.wrapping_add(SPECIAL_B_OFF));
        let mut strip = STRIP_FIRST;
        while strip < STRIP_PAST {
            let base = this
                .wrapping_add(STRIPS_OFF)
                .wrapping_add((strip - STRIP_FIRST).wrapping_mul(8));
            let state: u32 = lf_checker_rt::callee_thiscall!(STRIP_STATE, u32, this, strip);
            if (state & 0xff) == 0 {
                let mut cell_c = 0u32;
                let mut cell_d = 0u32;
                let mut cell_e = 0u32;
                let (cell, mode) = if strip == special_a {
                    (&mut cell_c as *mut u32 as u32, GENERATION)
                } else if strip == special_b {
                    (&mut cell_d as *mut u32 as u32, 1)
                } else {
                    (&mut cell_e as *mut u32 as u32, 0x3b)
                };
                let sq: u32 =
                    lf_checker_rt::callee_cdecl!(STRIP_QUERY, u32, cell, mode);
                let _: u32 = lf_checker_rt::callee_cdecl!(STRIP_COMMIT, u32, rd32(sq));
                let _: u32 = lf_checker_rt::callee_cdecl!(METER_MODE, u32, 1);
                let lv: u32 = lf_checker_rt::callee_thiscall!(
                    METER_DRAW, u32,
                    nested,
                    rd32(table.wrapping_add(strip.wrapping_mul(4)))
                );
                let _: u32 = lf_checker_rt::callee_cdecl!(
                    LEVEL_DRAW, u32,
                    rd32(base), rd32(base.wrapping_add(4)),
                    lv, 0xffff_ffff, 0xffff_ffff
                );
                let _: u32 = lf_checker_rt::callee_cdecl!(METER_MODE, u32, 2);
                let mut cell_f = 0u32;
                let painted: u32 = lf_checker_rt::callee_thiscall!(
                    STRIP_PAINT, u32,
                    this, strip, &mut cell_f as *mut u32 as u32
                );
                if painted != 0 {
                    let _: u32 = lf_checker_rt::callee_cdecl!(
                        LEVEL_DRAW, u32,
                        rd32(base), rd32(base.wrapping_add(4)),
                        painted, 0xffff_ffff, 0xffff_ffff
                    );
                }
            }
            strip = strip.wrapping_add(1);
        }

        let flushed: u32 = lf_checker_rt::callee_cdecl!(OVERLAY_FLUSH, u32,);
        cookie_check();
        flushed
    }
});
