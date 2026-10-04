// original: 0x00e3fd50 frontend_menu_input_update (proposed)
//
// Placeholder merged name: FRONTEND_MENU_HIGHLIGHT.

/// Drive one tick of the frontend menu state machine from menu-toggle, pad
/// and setting inputs, updating the selection, highlight and colour state.
///
/// `this` points to the menu object (at least `0x452` bytes). The object
/// holds two row arrays (`ARR_A` at `+0x54`, `ARR_B` at `+0x58`, rows of
/// `ROW_STRIDE` bytes with a flag dword at `+0x14` and a value dword at
/// `+0x10`), a selected pair (`SEL_A`/`SEL_B`), a saved pair (`SAVE_A`/
/// `SAVE_B`) plus a stashed pair (`STASH_A`/`STASH_B`), small count/level
/// bytes at `+0x310..0x313`, a signed depth byte (`DEPTH`), a mode byte
/// (`MODE`: 0 normal, 1 alt list, 2 second list) and state bytes (`STATE` at
/// `+0x44d`, `DIRTY` at `+0x3c0`, `PENDING` at `+0x3d8` with the pending
/// action id in `ACTION`).
///
/// Behaviour in order: return at once when `ENABLED` is clear. When
/// `PENDING` is set, dispatch on `ACTION - 1` (ids 1..6; anything else
/// returns): one group polls menu-toggle id 11 and on success clears
/// `PENDING` and raises the "state 5" exit, the other polls id 8 and on
/// success clears `PENDING` and `DIRTY` and returns. Otherwise run the row
/// source for the current mode and walk a chain of menu-toggle gates
/// (ids 11, 20, 9, 19, 10), each either returning, falling through, or
/// entering its block: the state-5 exit, a ready-flag toggle pair, two row
/// scans that resolve the first flagged row through helper 7 and convert
/// setting 0x33 to an integer for three highlight writers, and
/// save/stash shuffles. After the gates come two pad blocks (pad object
/// from helper 10, buttons at `+0x328c`/`+0x328d`, stick pairs near
/// `+0x2c6c`/`+0x2c7c`): each either steps the selection through helpers
/// 11/12/13 or latches a value through helper 15, and both join a common
/// tail (helpers 11 then 7). Then come the depth gates (helpers 17 with
/// three text ids) and finally a float block: two scale factors built from
/// integer globals times float globals, clamped against a constant limit,
/// two setting floats (ids 0x63/0x64) reshaped by helper 18, and up to two
/// highlight-colour resolutions (helper 19) whose dereferenced colour is
/// stored to the colour global and whose mask bit steps `DEPTH` down/up.
///
/// Calling convention: thiscall with no stack words (`this` in ECX);
/// returns nothing. Float arithmetic follows the original's operand order
/// through pinning helpers; ordered comparisons match `comiss` NaN
/// behaviour (`!(x > y)` is taken for NaN, like `jbe`). Two float slots in
/// the final block are read from stack the original never writes (the
/// helper-18 callee only stores one word per pointer on this path, shown
/// by reading that callee: its two-word stores target its fourth argument,
/// which is null here); the harness fills them with a fixed word
/// (`stack_fill`), so they read as `UNINIT_SLOT` here. The pristine
/// globals leave every colour gate shut (both clamped scales read `0.0`
/// against a `0.0` slot), so the contract seeds the scale/rate/mask
/// globals to plausible runtime values; the same values reach both sides.
lf_checker_rt::export!(thiscall, rw_00e3fd50(this: u32) -> u32 {
    unsafe {
        const ARR_B_SEL: u8 = 2;
        const ROW_STRIDE: u32 = 0x2b0;
        const ROW_VALUE: u32 = 0x10;
        const ROW_FLAG: u32 = 0x14;
        const FLAG_SET: u32 = 1;
        const FLAG_EMPTY: u32 = 0xffff_ffff;
        const LINK: u32 = 0x48;
        const ARR_A: u32 = 0x54;
        const ARR_B: u32 = 0x58;
        const LEVEL: u32 = 0x310;
        const RANGE_LO: u32 = 0x311;
        const COUNT: u32 = 0x312;
        const RANGE_HI: u32 = 0x313;
        const ALT_VIEW: u32 = 0x395;
        const LATCHED: u32 = 0x398;
        const LATCH_COUNT: u32 = 0x399;
        const LATCH_VALUE: u32 = 0x39c;
        const READY: u32 = 0x3a0;
        const MARKER: u32 = 0x3b8;
        const DIRTY: u32 = 0x3c0;
        const SEL_A: u32 = 0x3c4;
        const SEL_B: u32 = 0x3c8;
        const DEPTH: u32 = 0x3cc;
        const STEP_FLAG: u32 = 0x3cd;
        const MODE: u32 = 0x3ce;
        const TOGGLE: u32 = 0x3cf;
        const ACTION: u32 = 0x3d0;
        const PENDING: u32 = 0x3d8;
        const SAVE_A: u32 = 0x404;
        const SAVE_B: u32 = 0x408;
        const SAVE_COUNT: u32 = 0x40c;
        const STASH_A: u32 = 0x410;
        const STASH_B: u32 = 0x414;
        const STASH_COUNT: u32 = 0x418;
        const ENABLED: u32 = 0x44c;
        const STATE: u32 = 0x44d;
        const SEEN: u32 = 0x451;
        const PAD_BUTTON_A: u32 = 0x328c;
        const PAD_BUTTON_B: u32 = 0x328d;
        const STICK_LO: u32 = 0x2c6c;
        const STICK_HI: u32 = 0x2c6e;
        const STICK2_LO: u32 = 0x2c7c;
        const STICK2_HI: u32 = 0x2c7e;
        const STICK_DEAD: u8 = 0x7f;
        // Intercepted callees.
        const C_TOGGLE: u32 = 1;
        const C_GUARD: u32 = 2;
        const C_LINK_ON: u32 = 3;
        const C_LINK_OFF: u32 = 4;
        const C_MARK: u32 = 5;
        const C_FRONTEND: u32 = 6;
        const C_LOOKUP: u32 = 7;
        const C_SETTING: u32 = 8;
        const C_HIGHLIGHT: u32 = 9;
        const C_PAD: u32 = 10;
        const C_STEP: u32 = 11;
        const C_PICK: u32 = 12;
        const C_SCAN: u32 = 13;
        const C_RESET: u32 = 14;
        const C_FIND: u32 = 15;
        const C_ADVANCE: u32 = 16;
        const C_TEXT: u32 = 17;
        const C_RESHAPE: u32 = 18;
        const C_COLOUR: u32 = 19;
        // File VAs of globals and constant addresses.
        const G_SCALE_A: u32 = 0x18b7a8c;
        const G_SCALE_B: u32 = 0x18b7a80;
        const G_RATE_A: u32 = 0x17accf0;
        const G_RATE_B: u32 = 0x17acce8;
        const G_LIMIT: u32 = 0xfe88e8;
        const G_MASK_A: u32 = 0x18b7a88;
        const G_MASK_B: u32 = 0x18b7a84;
        const G_COLOUR: u32 = 0x1030bc0;
        const HUD_A: u32 = 0x1161698;
        const HUD_B: u32 = 0x11616c8;
        const HUD_C: u32 = 0x1161578;
        const TEXT_OBJ: u32 = 0x1176888;
        const TEXT_UP: u32 = 0xf15cd8;
        const TEXT_UP_ALT: u32 = 0xf15d08;
        const TEXT_DOWN_ALT: u32 = 0xf15d20;
        const TEXT_DOWN: u32 = 0xf15cf0;
        /// Stack slots the original reads without writing; the harness
        /// fills uninitialised stack with this fixed word (`stack_fill`
        /// `0x3e800000`), so these read as `0.25` on both sides.
        const UNINIT_SLOT: f32 = 0.25;

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
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        /// Truncate a float toward zero exactly like `cvttss2si`: NaN and
        /// out-of-range values (including +inf and 2^31) yield `i32::MIN`,
        /// where a plain `as` cast would saturate or give zero.
        #[inline(always)]
        fn cvtt(v: f32) -> u32 {
            if v.is_nan() || v >= 2147483648.0 {
                0x8000_0000
            } else {
                (v as i32) as u32
            }
        }
        /// Clamp `v` into `[0, limit]`, passing NaN through, matching the
        /// original's ordered `comiss` pair.
        #[inline(always)]
        fn clamp01(v: f32, limit: f32) -> f32 {
            if v < 0.0 {
                0.0
            } else if v > limit {
                limit
            } else {
                v
            }
        }
        /// Menu-toggle poll: seven stack words, id first.
        #[inline(always)]
        unsafe fn toggle(id: u32, a1: u32, a2: u32) -> u8 {
            unsafe { (lf_checker_rt::callee_cdecl!(C_TOGGLE, u32, id, a1, a2, 0u32, 0u32, 0u32, 0u32) & 0xff) as u8 }
        }
        /// Read setting `id` as a float through the shared convert slot,
        /// convert to int. The slot is one frame word the original reuses
        /// for every convert call, so it persists across calls (later
        /// snapshots see earlier stub writes); it starts as the harness
        /// stack fill like every other untouched frame word.
        #[inline(always)]
        unsafe fn setting_int(slot: &mut f32, id: u32) -> u32 {
            unsafe {
                let echoed: u32 = lf_checker_rt::callee_cdecl!(C_SETTING, u32, slot as *mut f32 as u32, id);
                cvtt(f32::from_bits(rd32(echoed)))
            }
        }
        /// Triple highlight refresh used by the save/stash paths.
        #[inline(always)]
        unsafe fn refresh_highlights(slot: &mut f32) {
            unsafe {
                let v = setting_int(slot, 0x33);
                lf_checker_rt::callee_thiscall!(C_HIGHLIGHT, u32, lf_checker_rt::relocated(HUD_A), v, 0u32, 0xffu32);
                let v = setting_int(slot, 0x33);
                lf_checker_rt::callee_thiscall!(C_HIGHLIGHT, u32, lf_checker_rt::relocated(HUD_B), v, 0u32, 0xffu32);
                let v = setting_int(slot, 0x33);
                lf_checker_rt::callee_thiscall!(C_HIGHLIGHT, u32, lf_checker_rt::relocated(HUD_C), v, 0u32, 0xffu32);
            }
        }
        /// Common tail of the pad/step paths: step then look up.
        #[inline(always)]
        unsafe fn step_lookup_tail(obj: u32, rows: u32, fourth: u32) {
            unsafe {
                let stepped: u32 = lf_checker_rt::callee_thiscall!(
                    C_STEP, u32, obj, rows, rd32(obj + SEL_B), rd8(obj + COUNT) as u32, fourth);
                wr32(obj + SEL_B, stepped);
                let found: u32 = lf_checker_rt::callee_thiscall!(C_LOOKUP, u32, obj, stepped);
                wr32(obj + SEL_A, found);
                wr8(obj + STEP_FLAG, 1);
            }
        }

        let obj = this;
        let mut conv_slot: f32 = UNINIT_SLOT;
        if rd8(obj + ENABLED) == 0 {
            return 0;
        }
        if rd8(obj + PENDING) != 0 {
            let action = rd32(obj + ACTION).wrapping_sub(1);
            if action > 5 {
                return 0;
            }
            // Jump-table groups: {0,1,3} poll id 11, {2,4,5} poll id 8.
            if action == 0 || action == 1 || action == 3 {
                if toggle(0xb, 1, 2) == 0 {
                    return 0;
                }
                wr8(obj + PENDING, 0);
            } else {
                if toggle(8, 1, 2) == 0 {
                    return 0;
                }
                wr8(obj + PENDING, 0);
                wr8(obj + DIRTY, 0);
                return 0;
            }
            // State-5 exit shared with the gate below.
            wr8(obj + DIRTY, 1);
            wr8(obj + STATE, 5);
            lf_checker_rt::callee_cdecl!(C_FRONTEND, u32, 0x44u32);
            return 0;
        }
        let rows = if rd8(obj + MODE) == ARR_B_SEL { rd32(obj + ARR_B) } else { rd32(obj + ARR_A) };
        if toggle(0xb, 1, 0) != 0 {
            let state = rd8(obj + STATE);
            let mut guarded = true;
            match state {
                0 | 2 | 4 | 5 | 7 | 8 | 9 | 0xa => {}
                _ => {
                    let ok: u32 = lf_checker_rt::callee_thiscall!(C_GUARD, u32, obj);
                    if (ok & 0xff) == 0 {
                        guarded = false;
                    }
                }
            }
            if guarded {
                let link = rd32(obj + LINK);
                if link != 0 {
                    let on: u32 = lf_checker_rt::callee_thiscall!(C_LINK_ON, u32, link);
                    if (on & 0xff) != 0 {
                        lf_checker_rt::callee_thiscall!(C_LINK_OFF, u32, rd32(obj + LINK));
                    }
                }
                if rd32(obj + MARKER) == 1 {
                    lf_checker_rt::callee_cdecl!(C_MARK, u32, obj + MARKER);
                }
            }
            wr8(obj + DIRTY, 1);
            wr8(obj + STATE, 5);
            lf_checker_rt::callee_cdecl!(C_FRONTEND, u32, 0x44u32);
            return 0;
        }
        if rd8(obj + READY) == 0 {
            return 0;
        }
        if toggle(0x14, 1, 0) != 0 && rd8(obj + STATE) == 3 {
            wr8(obj + TOGGLE, (rd8(obj + TOGGLE) == 0) as u8);
            wr8(obj + DIRTY, (rd8(obj + DIRTY) == 0) as u8);
            return 0;
        }
        if rd8(obj + DIRTY) != 0 {
            return 0;
        }
        if toggle(9, 1, 0) != 0 {
            let mode = rd8(obj + MODE);
            if mode != ARR_B_SEL && mode != 1 {
                let bound = rd8(obj + COUNT) as u32;
                let mut any = false;
                if bound > 0 {
                    let mut p = rows.wrapping_add(ROW_FLAG);
                    let mut i = 0u32;
                    while i < bound {
                        if rd32(p) == FLAG_SET {
                            wr32(obj + SEL_B, i);
                            let found: u32 = lf_checker_rt::callee_thiscall!(C_LOOKUP, u32, obj, i);
                            wr32(obj + SEL_A, found);
                            any = true;
                        }
                        i += 1;
                        p = p.wrapping_add(ROW_STRIDE);
                    }
                }
                if any {
                    let v = setting_int(&mut conv_slot, 0x33);
                    lf_checker_rt::callee_thiscall!(C_HIGHLIGHT, u32, lf_checker_rt::relocated(HUD_A), v, 0u32, 0xffu32);
                    wr8(obj + MODE, 0);
                    return 0;
                }
                wr8(obj + STATE, 9);
                wr8(obj + DIRTY, 1);
                wr8(obj + SEEN, 1);
                wr8(obj + MODE, 0);
                return 0;
            }
        }
        if toggle(0x13, 1, 0) != 0 {
            let mode = rd8(obj + MODE);
            // Mode 1 skips this gate's block and continues at the next gate.
            if mode != 1 {
                let sel_a = rd32(obj + SEL_A);
                if mode == ARR_B_SEL {
                    wr8(obj + LEVEL, rd8(obj + LEVEL).wrapping_add(1));
                    wr32(obj + STASH_A, sel_a);
                    wr32(obj + STASH_B, rd32(obj + SEL_B));
                    wr32(obj + SEL_A, rd32(obj + SAVE_A));
                    wr32(obj + SEL_B, rd32(obj + SAVE_B));
                    wr8(obj + COUNT, rd8(obj + SAVE_COUNT));
                    wr8(obj + MODE, 0);
                    refresh_highlights(&mut conv_slot);
                    return 0;
                }
                let sel_b = rd32(obj + SEL_B);
                wr32(obj + SAVE_A, sel_a);
                wr32(obj + SAVE_B, sel_b);
                if rd32(obj + ARR_B) == 0 {
                    wr8(obj + STATE, 0xa);
                    wr8(obj + DIRTY, 1);
                    return 0;
                }
                wr8(obj + LEVEL, rd8(obj + LEVEL).wrapping_sub(1));
                wr32(obj + SAVE_A, sel_a);
                wr32(obj + SEL_A, rd32(obj + STASH_A));
                wr32(obj + SEL_B, rd32(obj + STASH_B));
                let stashed = rd8(obj + STASH_COUNT);
                wr8(obj + STATE, 3);
                wr8(obj + MODE, ARR_B_SEL);
                wr32(obj + SAVE_B, sel_b);
                wr8(obj + COUNT, stashed);
                refresh_highlights(&mut conv_slot);
                return 0;
            }
        }
        if toggle(0xa, 1, 0) != 0 {
            let mode = rd8(obj + MODE);
            if mode != ARR_B_SEL {
                if mode == 1 {
                    wr32(obj + SEL_A, rd32(obj + SAVE_A));
                    wr32(obj + SEL_B, rd32(obj + SAVE_B));
                    wr8(obj + MODE, 0);
                    wr8(obj + COUNT, rd8(obj + SAVE_COUNT));
                    refresh_highlights(&mut conv_slot);
                    return 0;
                }
                wr32(obj + SAVE_A, rd32(obj + SEL_A));
                wr32(obj + SAVE_B, rd32(obj + SEL_B));
                let saved = rd8(obj + SAVE_COUNT);
                wr8(obj + COUNT, saved);
                let mut any = false;
                if saved as u32 > 0 {
                    let mut p = rows.wrapping_add(ROW_FLAG);
                    let mut i = 0u32;
                    while i < saved as u32 {
                        if rd32(p) == FLAG_SET {
                            wr32(obj + SEL_B, i);
                            let found: u32 = lf_checker_rt::callee_thiscall!(C_LOOKUP, u32, obj, i);
                            wr32(obj + SEL_A, found);
                            any = true;
                        }
                        i += 1;
                        p = p.wrapping_add(ROW_STRIDE);
                    }
                }
                if !any {
                    wr8(obj + STATE, 9);
                    wr8(obj + DIRTY, 1);
                    return 0;
                }
                wr8(obj + MODE, 1);
                refresh_highlights(&mut conv_slot);
                return 0;
            }
        }
        // First pad block: button A plus toggle id 0x12, else button B
        // plus the second stick pair past its dead zone.
        let pad0: u32 = lf_checker_rt::callee_cdecl!(C_PAD, u32, 0u32);
        let mut down_a = false;
        if rd8(pad0 + PAD_BUTTON_A) != 0 && toggle(0x12, 0, 0) != 0 {
            down_a = true;
        } else {
            let pad1: u32 = lf_checker_rt::callee_cdecl!(C_PAD, u32, 0u32);
            if rd8(pad1 + PAD_BUTTON_B) != 0 {
                let pad2: u32 = lf_checker_rt::callee_cdecl!(C_PAD, u32, 0u32);
                let stick = rd8(pad2 + STICK2_HI) ^ rd8(pad2 + STICK2_LO);
                if stick > STICK_DEAD {
                    down_a = true;
                }
            }
        }
        if down_a {
            let mode = rd8(obj + MODE);
            if mode != 1 {
                let count = rd8(obj + COUNT) as u32;
                let stepped: u32 = lf_checker_rt::callee_thiscall!(
                    C_STEP, u32, obj, rows, rd32(obj + SEL_A), count, rd8(obj + LEVEL) as u32);
                // Latch flag: set when the stepped row exists and the old
                // mode was not the second list.
                let latch = if (stepped.wrapping_add(1) as i32) < (count as i32) {
                    0u8
                } else if rd32(rows.wrapping_add(stepped.wrapping_mul(ROW_STRIDE)).wrapping_add(ROW_FLAG)) == FLAG_EMPTY {
                    0u8
                } else if mode == ARR_B_SEL {
                    0u8
                } else {
                    1u8
                };
                let picked: u32 = lf_checker_rt::callee_thiscall!(C_PICK, u32, obj, rows, count);
                if latch == 0 {
                    if (picked as i32) < (count as i32) && rd32(obj + SEL_B) == picked.wrapping_sub(1) {
                        lf_checker_rt::callee_thiscall!(C_RESET, u32, obj + ACTION);
                    }
                } else if rd8(obj + LATCHED) == 0 {
                    let scanned: u32 = lf_checker_rt::callee_thiscall!(C_SCAN, u32, obj, rows, count);
                    let value = rd32(rows.wrapping_add(scanned.wrapping_mul(ROW_STRIDE)).wrapping_add(ROW_VALUE));
                    wr32(obj + LATCH_VALUE, value.wrapping_add(1));
                    wr8(obj + LATCH_COUNT, rd8(obj + COUNT));
                    wr8(obj + STATE, 2);
                    wr8(obj + DIRTY, 1);
                } else {
                    // Latch set but already latched: unconditional reset.
                    lf_checker_rt::callee_thiscall!(C_RESET, u32, obj + ACTION);
                }
                step_lookup_tail(obj, rows, rd8(obj + LEVEL) as u32);
                return 0;
            }
        }
        // Second pad block: button A plus toggle id 0x11, else button B
        // plus the first stick pair past its dead zone.
        let pad0: u32 = lf_checker_rt::callee_cdecl!(C_PAD, u32, 0u32);
        let mut down_b = false;
        if rd8(pad0 + PAD_BUTTON_A) != 0 && toggle(0x11, 0, 0) != 0 {
            down_b = true;
        } else {
            let pad1: u32 = lf_checker_rt::callee_cdecl!(C_PAD, u32, 0u32);
            if rd8(pad1 + PAD_BUTTON_B) != 0 {
                let pad2: u32 = lf_checker_rt::callee_cdecl!(C_PAD, u32, 0u32);
                let stick = rd8(pad2 + STICK_HI) ^ rd8(pad2 + STICK_LO);
                if stick > STICK_DEAD {
                    down_b = true;
                }
            }
        }
        if down_b {
            let mode = rd8(obj + MODE);
            if mode != 1 {
                let count = rd8(obj + COUNT) as u32;
                let found_at: u32 = lf_checker_rt::callee_thiscall!(C_FIND, u32, obj, rows, count);
                let at_value = rd32(rows.wrapping_add(found_at.wrapping_mul(ROW_STRIDE)).wrapping_add(ROW_VALUE));
                let span = if at_value > count { at_value - count } else { 1 };
                let first_flag = rd32(rd32(obj + ARR_A).wrapping_add(ROW_FLAG));
                if first_flag != FLAG_EMPTY
                    && at_value > 1
                    && mode != ARR_B_SEL
                    && rd32(obj + SEL_A) == 0
                    && span != 0
                {
                    wr32(obj + LATCH_VALUE, span);
                    wr8(obj + LATCH_COUNT, rd8(obj + COUNT));
                    wr8(obj + STATE, 2);
                    wr8(obj + DIRTY, 1);
                    wr8(obj + LATCHED, 0);
                }
                let advanced: u32 = lf_checker_rt::callee_thiscall!(
                    C_ADVANCE, u32, obj, rows, rd32(obj + SEL_B), rd8(obj + LEVEL) as u32);
                wr32(obj + SEL_B, advanced);
                let found: u32 = lf_checker_rt::callee_thiscall!(C_LOOKUP, u32, obj, advanced);
                wr32(obj + SEL_A, found);
                wr8(obj + STEP_FLAG, 0);
                return 0;
            }
        }
        if toggle(0, 1, 0) != 0 {
            let mode = rd8(obj + MODE);
            if mode != 1 {
                let sel_a = rd32(obj + SEL_A);
                if sel_a == rd32(obj + SEL_B) {
                    let count = rd8(obj + COUNT) as u32;
                    let found_at: u32 = lf_checker_rt::callee_thiscall!(C_FIND, u32, obj, rows, count);
                    let at_value = rd32(rows.wrapping_add(found_at.wrapping_mul(ROW_STRIDE)).wrapping_add(ROW_VALUE));
                    let span = if at_value > count { at_value - count } else { 1 };
                    if mode != ARR_B_SEL && span != 0 && sel_a == 0 && at_value > 1 {
                        wr32(obj + LATCH_VALUE, span);
                        wr8(obj + LATCH_COUNT, rd8(obj + COUNT));
                        wr8(obj + STATE, 2);
                        wr8(obj + DIRTY, 1);
                        wr8(obj + LATCHED, 0);
                    }
                }
                let advanced: u32 = lf_checker_rt::callee_thiscall!(
                    C_ADVANCE, u32, obj, rows, rd32(obj + SEL_B), 1u32);
                wr32(obj + SEL_B, advanced);
                let found: u32 = lf_checker_rt::callee_thiscall!(C_LOOKUP, u32, obj, advanced);
                wr32(obj + SEL_A, found);
                wr8(obj + STEP_FLAG, 0);
                return 0;
            }
        }
        if toggle(1, 1, 0) != 0 {
            let mode = rd8(obj + MODE);
            if mode != 1 {
                let count = rd8(obj + COUNT) as u32;
                let sel_b = rd32(obj + SEL_B);
                let stepped: u32 = lf_checker_rt::callee_thiscall!(
                    C_STEP, u32, obj, rows, sel_b, count, 1u32);
                let latch = if (stepped.wrapping_add(1) as i32) < (count as i32) {
                    0u8
                } else if rd32(rows.wrapping_add(stepped.wrapping_mul(ROW_STRIDE)).wrapping_add(ROW_FLAG)) == FLAG_EMPTY {
                    0u8
                } else if mode == ARR_B_SEL {
                    0u8
                } else {
                    1u8
                };
                let picked: u32 = lf_checker_rt::callee_thiscall!(C_PICK, u32, obj, rows, count);
                if latch == 0 {
                    if (picked as i32) < (count as i32) && sel_b == picked.wrapping_sub(1) {
                        lf_checker_rt::callee_thiscall!(C_RESET, u32, obj + ACTION);
                    }
                } else if rd8(obj + LATCHED) == 0 {
                    let scanned: u32 = lf_checker_rt::callee_thiscall!(C_SCAN, u32, obj, rows, count);
                    let value = rd32(rows.wrapping_add(scanned.wrapping_mul(ROW_STRIDE)).wrapping_add(ROW_VALUE));
                    wr32(obj + LATCH_VALUE, value.wrapping_add(1));
                    wr8(obj + LATCH_COUNT, rd8(obj + COUNT));
                    wr8(obj + STATE, 2);
                    wr8(obj + DIRTY, 1);
                } else {
                    // Latch set but already latched: unconditional reset.
                    lf_checker_rt::callee_thiscall!(C_RESET, u32, obj + ACTION);
                }
                step_lookup_tail(obj, rows, 1);
                return 0;
            }
        }
        // Depth gates: identical shape on the alt-view and plain sides,
        // except the plain side's range check omits the widening.
        let alt = rd8(obj + ALT_VIEW);
        if alt != 0 {
            if toggle(3, 1, 0) != 0 {
                let lo = rd8(obj + RANGE_LO) as u32;
                let range = (rd8(obj + RANGE_HI) as u32).wrapping_sub(lo).wrapping_add(1);
                let depth = (rd8(obj + DEPTH) as i8) as i32 + 2;
                if depth < range as i32 {
                    lf_checker_rt::callee_thiscall!(
                        C_TEXT, u32, lf_checker_rt::relocated(TEXT_OBJ), lf_checker_rt::relocated(TEXT_UP));
                    wr8(obj + DEPTH, rd8(obj + DEPTH).wrapping_add(1));
                    return 0;
                }
            }
            if toggle(2, 1, 0) != 0 && (rd8(obj + DEPTH) as i8) > 1 {
                lf_checker_rt::callee_thiscall!(
                    C_TEXT, u32, lf_checker_rt::relocated(TEXT_OBJ), lf_checker_rt::relocated(TEXT_DOWN));
                wr8(obj + DEPTH, rd8(obj + DEPTH).wrapping_sub(1));
                return 0;
            }
        } else {
            if toggle(3, 1, 0) != 0 {
                let lo = rd8(obj + RANGE_LO) as u32;
                let range = (rd8(obj + RANGE_HI) as u32).wrapping_sub(lo);
                let depth = (rd8(obj + DEPTH) as i8) as i32 + 2;
                if depth < range as i32 {
                    lf_checker_rt::callee_thiscall!(
                        C_TEXT, u32, lf_checker_rt::relocated(TEXT_OBJ), lf_checker_rt::relocated(TEXT_UP_ALT));
                    wr8(obj + DEPTH, rd8(obj + DEPTH).wrapping_add(1));
                    return 0;
                }
            }
            if toggle(2, 1, 0) != 0 && (rd8(obj + DEPTH) as i8) > 1 {
                lf_checker_rt::callee_thiscall!(
                    C_TEXT, u32, lf_checker_rt::relocated(TEXT_OBJ), lf_checker_rt::relocated(TEXT_DOWN_ALT));
                wr8(obj + DEPTH, rd8(obj + DEPTH).wrapping_sub(1));
                return 0;
            }
        }
        // Final float block: clamped scales, reshaped settings, colours.
        let pad: u32 = lf_checker_rt::callee_cdecl!(C_PAD, u32, 1u32);
        if rd8(pad + PAD_BUTTON_B) == 0 {
            return 0;
        }
        let limit = f32::from_bits(rd32(lf_checker_rt::relocated(G_LIMIT)));
        let raw_hi = mul((rd32(lf_checker_rt::relocated(G_SCALE_A)) as i32) as f32,
            rdf(lf_checker_rt::relocated(G_RATE_A)));
        let slot_hi = clamp01(raw_hi, limit);
        let raw_lo = mul((rd32(lf_checker_rt::relocated(G_SCALE_B)) as i32) as f32,
            rdf(lf_checker_rt::relocated(G_RATE_B)));
        let slot_lo = clamp01(raw_lo, limit);
        let depth_b = rd8(obj + DEPTH);
        let gate = if (depth_b as i8) > 1 { 1u32 } else { 0u32 };
        let range_lo = rd8(obj + RANGE_LO) as u32;
        let range_hi = rd8(obj + RANGE_HI) as u32;
        let depth_i = (depth_b as i8) as i32 + 2;
        let tied = if alt != 0 {
            depth_i < (range_hi.wrapping_sub(range_lo).wrapping_add(1)) as i32
        } else {
            depth_i < (range_hi.wrapping_sub(range_lo)) as i32
        };
        let mut f1: f32 = UNINIT_SLOT;
        let mut f2: f32 = UNINIT_SLOT;
        lf_checker_rt::callee_cdecl!(C_SETTING, u32, &mut f1 as *mut f32 as u32, 0x63u32);
        lf_checker_rt::callee_cdecl!(C_SETTING, u32, &mut f2 as *mut f32 as u32, 0x64u32);
        lf_checker_rt::callee_cdecl!(
            C_RESHAPE, u32, 3u32, &mut f1 as *mut f32 as u32, &mut f2 as *mut f32 as u32, 0u32);
        let mut x1 = f1;
        let x3 = f2;
        if gate != 0 {
            if tied {
                x1 = sub(x1, x3);
                f1 = x1;
            }
            let x2 = slot_lo;
            if !(x2 > x1) {
                // skip
            } else {
                let x0 = add(x1, x3);
                if x0 > x2 {
                    let x4 = UNINIT_SLOT;
                    let x2 = slot_hi;
                    if x2 > x4 {
                        let x0 = add(UNINIT_SLOT, x4);
                        if x0 > x2 {
                            let mut slot = gate;
                            let colour: u32 = lf_checker_rt::callee_cdecl!(
                                C_COLOUR, u32, &mut slot as *mut u32 as u32, 1u32);
                            let mask = rd32(lf_checker_rt::relocated(G_MASK_A))
                                ^ rd32(lf_checker_rt::relocated(G_MASK_B));
                            let resolved = rd32(colour);
                            let masked = mask & rd32(lf_checker_rt::relocated(G_MASK_A));
                            x1 = f1;
                            wr32(lf_checker_rt::relocated(G_COLOUR), resolved);
                            if (masked & 1) != 0 {
                                wr8(obj + DEPTH, rd8(obj + DEPTH).wrapping_sub(1));
                            }
                        }
                    }
                }
            }
            x1 = add(x1, x3);
            f1 = x1;
        }
        if !tied {
            return 0;
        }
        let x0 = slot_lo;
        if !(x0 > x1) {
            return 0;
        }
        x1 = add(x1, x3);
        if !(x1 > x0) {
            return 0;
        }
        let x4 = UNINIT_SLOT;
        let x1b = slot_hi;
        if !(x1b > x4) {
            return 0;
        }
        let x0 = add(UNINIT_SLOT, x4);
        if !(x0 > x1b) {
            return 0;
        }
        let mut slot = gate;
        let colour: u32 = lf_checker_rt::callee_cdecl!(
            C_COLOUR, u32, &mut slot as *mut u32 as u32, 1u32);
        let mask = rd32(lf_checker_rt::relocated(G_MASK_A)) ^ rd32(lf_checker_rt::relocated(G_MASK_B));
        let resolved = rd32(colour);
        let masked = mask & rd32(lf_checker_rt::relocated(G_MASK_A));
        wr32(lf_checker_rt::relocated(G_COLOUR), resolved);
        if (masked & 1) != 0 {
            wr8(obj + DEPTH, rd8(obj + DEPTH).wrapping_add(1));
        }
        0
    }
});
