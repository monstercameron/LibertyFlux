// original: 0x005cad70 event_handler_update (proposed)

/// Refresh the input-event timer, then program the event-handler objects from
/// the configuration records, or park them when the subsystem is disabled.
///
/// No stack arguments (cdecl); the single caller ignores `eax`, so the
/// rewrite returns 0. All state lives in globals: `G_ENABLE` selects the tail
/// path (park every handler), `G_MODE` selects the middle path (single park
/// check plus the configuration loop), otherwise the main path runs (string
/// setup through a frame buffer and thread-local block, one handler
/// programmed, return).
///
/// Handler objects are reached through `HANDLER_TABLE` indexed by ids held in
/// `G_HANDLER_ID` and in the id array at `LOOP_IDS` (four entries, stride
/// `0x38`, each entry programming the object pair `[esi-0x1c]`/`[esi]`).
/// The parallel configuration array at `LOOP_CFG` (four records, stride
/// `0x88`) holds per entry: a type word at `+0`, a string at `+2`, an enable
/// byte at `+0x2c`, a value byte at `+0x2e`, and the second object's enable
/// byte at `-0x10`.
///
/// Per entry the original programs object A from the type word (a float pair
/// and a kind tag, or a platform-selected mode), then either copies the
/// string form into a frame buffer and hands it to the string setter, or
/// parses it as a base-10 integer, converts to float, scales by 0.01 and
/// hands that to the float setter. Object B is programmed only on that
/// configuration branch (platform-selected mode when its enable byte is set,
/// then the locale string and the value byte); when object A's enable byte
/// is clear the entry jumps to the next one after the park check, leaving
/// object B untouched. A handler whose active query answers nonzero is parked instead
/// (pair `(3, 0)`, flags `(0, 0)`); the active query result is tested in the
/// low byte only (`(an instruction of the original)`: an answer of `0x100` counts as inactive).
///
/// The platform test is `(byte_A == 0x6a) || (byte_B != 0)` (unsigned byte
/// comparison). The loop bound uses a signed comparison. Float order is the
/// original's: `int -> float` conversion, then multiply by the scale.
///
/// Stack quirk (verified by reading every push/cleanup): the numeric path
/// calls the cdecl string-to-integer helper without the caller cleanup, so
/// each numeric entry leaks 4 stack bytes; the return esp adjustment is
/// `-4 * (numeric entries)`. The proof leaves the esp check off for this
/// reason; the leaked slots hold only the helper argument and the scaled
/// float, both observed as call arguments. The frame-buffer pointers passed
/// to the string callees are skipped in the call comparison (their contents
/// are never read back by the function or the stubs).
lf_checker_rt::export!(cdecl, rw_005cad70() -> u32 {
    unsafe {
        const TIMER_OBJ: u32 = 0x11F61B8;
        const G_ENABLE: u32 = 0x11F630C;
        const G_MODE: u32 = 0x11F61F6;
        const G_SRC_STR: u32 = 0x11F61CC;
        const LOCALE_OBJ: u32 = 0x116BFF0;
        const G_LOCALE_ARG: u32 = 0x11F61BC;
        const G_HANDLER_ID: u32 = 0x118EEA0;
        const HANDLER_TABLE: u32 = 0x118E7F8;
        const LOOP_IDS: u32 = 0x118EED8;
        const LOOP_IDS_END: u32 = 0x118EFB8;
        const LOOP_ID_STEP: u32 = 0x38;
        const LOOP_CFG: u32 = 0x11F6210;
        const LOOP_CFG_STEP: u32 = 0x88;
        const PLAT_A: u32 = 0x116C250;
        const PLAT_B: u32 = 0x116C253;
        const PLAT_CONSOLE: u8 = 0x6a;
        const SCALE_BITS: u32 = 0x3C23D70A; // 0.01f
        const CFG_FLOAT_A0: u32 = 0x3DD91687;
        const CFG_FLOAT_A1: u32 = 0x3C54FDF4;
        const CFG_FLOAT_B0: u32 = 0x3F000000; // 0.5f
        const CFG_FLOAT_B1: u32 = 0x3F0CCCCD;
        const C_TIMEOUT: u32 = 0x320;
        const TLS_OFF: u32 = 0x78;
        const C_TIMER: u32 = 1;
        const C_COPY: u32 = 2;
        const C_LOCALE: u32 = 3;
        const C_CAT: u32 = 4;
        const C_MENU: u32 = 5;
        const C_STRSET_FRAME: u32 = 6;
        const C_STRSET: u32 = 7;
        const C_ISACTIVE: u32 = 8;
        const C_SETPAIR: u32 = 9;
        const C_SETFLAGS: u32 = 10;
        const C_COOKIE: u32 = 11;
        const C_STRTOL: u32 = 12;
        const C_SETFLOAT: u32 = 13;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn handler(idx: u32) -> u32 {
            unsafe {
                lf_checker_rt::global::<u32>(HANDLER_TABLE)
                    .add(idx as usize)
                    .read_unaligned()
            }
        }
        /// Low-byte activity test, exactly the original's `(an instruction of the original)`.
        #[inline(always)]
        fn lo_is_nz(ans: u32) -> bool {
            (ans & 0xFF) != 0 // AL-TEST
        }
        #[inline(always)]
        unsafe fn plat_other() -> bool {
            unsafe {
                rd8(lf_checker_rt::relocated(PLAT_A)) == PLAT_CONSOLE
                    || rd8(lf_checker_rt::relocated(PLAT_B)) != 0
            }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        /// Park one handler: pair `(3, 0)`, flags `(0, 0)`.
        #[inline(always)]
        unsafe fn park(obj: u32) {
            unsafe {
                lf_checker_rt::callee_thiscall!(C_SETPAIR, u32, obj, 3, 0);
                lf_checker_rt::callee_thiscall!(C_SETFLAGS, u32, obj, 0, 0);
            }
        }
        /// Program one handler's steady state: timeout, pair `(2, 0)`,
        /// flags `(1, 0)`. The object is reloaded before each step like the
        /// original (the id global is never written in between).
        #[inline(always)]
        unsafe fn steady(id_addr: u32) {
            unsafe {
                wr32(
                    handler(rd32(id_addr)).wrapping_add(0x3c),
                    C_TIMEOUT,
                );
                let o = handler(rd32(id_addr));
                lf_checker_rt::callee_thiscall!(C_SETPAIR, u32, o, 2, 0);
                let o = handler(rd32(id_addr));
                lf_checker_rt::callee_thiscall!(C_SETFLAGS, u32, o, 1, 0);
            }
        }

        lf_checker_rt::callee_thiscall!(
            C_TIMER,
            u32,
            lf_checker_rt::relocated(TIMER_OBJ)
        );
        if rd8(lf_checker_rt::relocated(G_ENABLE)) == 0 {
            // Tail path: park the single handler, then the simple loop.
            let o = handler(rd32(lf_checker_rt::relocated(G_HANDLER_ID)));
            let a = lf_checker_rt::callee_thiscall!(C_ISACTIVE, u32, o);
            if lo_is_nz(a) {
                park(handler(rd32(lf_checker_rt::relocated(G_HANDLER_ID))));
            }
            let mut ids = LOOP_IDS;
            while (ids as i32) < (LOOP_IDS_END as i32) {
                let ob = handler(rd32(
                    lf_checker_rt::relocated(ids.wrapping_sub(0x1c)),
                ));
                let a = lf_checker_rt::callee_thiscall!(C_ISACTIVE, u32, ob);
                if lo_is_nz(a) {
                    park(handler(rd32(
                        lf_checker_rt::relocated(ids.wrapping_sub(0x1c)),
                    )));
                }
                let oa = handler(rd32(lf_checker_rt::relocated(ids)));
                let a = lf_checker_rt::callee_thiscall!(C_ISACTIVE, u32, oa);
                if lo_is_nz(a) {
                    park(handler(rd32(lf_checker_rt::relocated(ids))));
                }
                ids = ids.wrapping_add(LOOP_ID_STEP);
            }
        } else if rd8(lf_checker_rt::relocated(G_MODE)) == 0 {
            // Middle path: single park check, then the configuration loop.
            let o = handler(rd32(lf_checker_rt::relocated(G_HANDLER_ID)));
            let a = lf_checker_rt::callee_thiscall!(C_ISACTIVE, u32, o);
            if lo_is_nz(a) {
                park(handler(rd32(lf_checker_rt::relocated(G_HANDLER_ID))));
            }
            let mut frame = [0u32; 8];
            let fbuf = frame.as_mut_ptr() as u32;
            let mut ids = LOOP_IDS;
            let mut cfg = LOOP_CFG;
            while (ids as i32) < (LOOP_IDS_END as i32) {
                let rcfg = lf_checker_rt::relocated(cfg);
                let rids = lf_checker_rt::relocated(ids);
                let rids_b = lf_checker_rt::relocated(ids.wrapping_sub(0x1c));
                if rd8(rcfg.wrapping_add(0x2c)) == 0 {
                    let o = handler(rd32(rids));
                    let a = lf_checker_rt::callee_thiscall!(C_ISACTIVE, u32, o);
                    if lo_is_nz(a) {
                        park(handler(rd32(rids)));
                    }
                } else {
                    let o = handler(rd32(rids));
                    let a = lf_checker_rt::callee_thiscall!(C_ISACTIVE, u32, o);
                    if !lo_is_nz(a) {
                        if rd16(rcfg) != 0 {
                            wr32(handler(rd32(rids)).wrapping_add(8), 9);
                            let w = handler(rd32(rids));
                            wr32(w.wrapping_add(0x20), CFG_FLOAT_A0);
                            wr32(w.wrapping_add(0x24), CFG_FLOAT_A1);
                        } else {
                            if plat_other() {
                                wr32(handler(rd32(rids)).wrapping_add(0x34), 2);
                            } else {
                                wr32(handler(rd32(rids)).wrapping_add(0x34), 0);
                            }
                            wr32(handler(rd32(rids)).wrapping_add(8), 5);
                            let w = handler(rd32(rids));
                            wr32(w.wrapping_add(0x20), CFG_FLOAT_B0);
                            wr32(w.wrapping_add(0x24), CFG_FLOAT_B1);
                        }
                        steady(rids);
                    }
                    if rd16(rcfg) == 0 {
                        lf_checker_rt::callee_cdecl!(
                            C_COPY,
                            u32,
                            lf_checker_rt::relocated(cfg.wrapping_add(2)),
                            fbuf
                        );
                        let v = rd8(rcfg.wrapping_add(0x2e)) as u32;
                        wr32(handler(rd32(rids)).wrapping_add(0x44), v);
                        wr32(handler(rd32(rids)).wrapping_add(8), 5);
                        let o = handler(rd32(rids));
                        lf_checker_rt::callee_thiscall!(C_STRSET_FRAME, u32, o, fbuf);
                    } else {
                        let n = lf_checker_rt::callee_cdecl!(
                            C_STRTOL,
                            u32,
                            lf_checker_rt::relocated(cfg.wrapping_add(2))
                        );
                        let f = (n as i32) as f32;
                        let g = mul(f, f32::from_bits(SCALE_BITS));
                        let v = rd8(rcfg.wrapping_add(0x2e)) as u32;
                        wr32(handler(rd32(rids)).wrapping_add(0x44), v);
                        wr32(handler(rd32(rids)).wrapping_add(8), 9);
                        let o = handler(rd32(rids));
                        lf_checker_rt::callee_thiscall!(C_SETFLOAT, u32, o, g.to_bits());
                    }
                    // Object B is programmed only on the configuration
                        // branch; the parked branch jumps to the next entry.
                        let cfg_b = cfg.wrapping_sub(0x10);
                        let rcfg_b = lf_checker_rt::relocated(cfg_b);
                        if rd8(rcfg_b) == 0 {
                            let o = handler(rd32(rids_b));
                            let a = lf_checker_rt::callee_thiscall!(C_ISACTIVE, u32, o);
                            if lo_is_nz(a) {
                                park(handler(rd32(rids_b)));
                            }
                        } else {
                            let o = handler(rd32(rids_b));
                            let a = lf_checker_rt::callee_thiscall!(C_ISACTIVE, u32, o);
                            if !lo_is_nz(a) {
                                if plat_other() {
                                    wr32(handler(rd32(rids_b)).wrapping_add(0x34), 2);
                                } else {
                                    wr32(handler(rd32(rids_b)).wrapping_add(0x34), 0);
                                }
                                steady(rids_b);
                            }
                            let s = lf_checker_rt::callee_thiscall!(
                                C_LOCALE,
                                u32,
                                lf_checker_rt::relocated(LOCALE_OBJ),
                                lf_checker_rt::relocated(cfg_b)
                            );
                            let v = rd8(rcfg.wrapping_add(0x2e)) as u32;
                            wr32(handler(rd32(rids_b)).wrapping_add(0x44), v);
                            let o = handler(rd32(rids_b));
                            lf_checker_rt::callee_thiscall!(C_STRSET, u32, o, s);
                    }
                }
                ids = ids.wrapping_add(LOOP_ID_STEP);
                cfg = cfg.wrapping_add(LOOP_CFG_STEP);
            }
        } else {
            // Main path: string setup, then one handler programmed.
            let mut frame = [0u32; 8];
            let fbuf = frame.as_mut_ptr() as u32;
            lf_checker_rt::callee_cdecl!(
                C_COPY,
                u32,
                lf_checker_rt::relocated(G_SRC_STR),
                fbuf
            );
            let s = lf_checker_rt::callee_thiscall!(
                C_LOCALE,
                u32,
                lf_checker_rt::relocated(LOCALE_OBJ),
                lf_checker_rt::relocated(G_LOCALE_ARG)
            );
            if rd8(lf_checker_rt::relocated(G_LOCALE_ARG)) != 0 {
                lf_checker_rt::callee_cdecl!(C_CAT, u32, fbuf, s);
            }
            let tls = lf_checker_rt::tls_slot(0).wrapping_add(TLS_OFF);
            lf_checker_rt::callee_cdecl!(C_MENU, u32, tls, fbuf, 0xFFFFFFFFu32);
            let o = handler(rd32(lf_checker_rt::relocated(G_HANDLER_ID)));
            lf_checker_rt::callee_thiscall!(C_STRSET, u32, o, tls);
            let o = handler(rd32(lf_checker_rt::relocated(G_HANDLER_ID)));
            let a = lf_checker_rt::callee_thiscall!(C_ISACTIVE, u32, o);
            if !lo_is_nz(a) {
                steady(lf_checker_rt::relocated(G_HANDLER_ID));
            }
        }
        lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
        0
    }
});
