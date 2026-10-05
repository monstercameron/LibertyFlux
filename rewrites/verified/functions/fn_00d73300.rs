// original: 0x00D73300 FRONTEND_MENU_MONTAGE_NAVIGATE_MT

/// Handle one frontend-menu montage navigation step.
///
/// `this` is the menu object. Its inner object (pointer at `INNER`) is
/// resolved twice through the lookup callees into the state object `st`
/// (never null) and the compare object `cmp` (sometimes null). Two flags
/// are latched from the state's first byte (`ST0 == 9` and `ST0 == 10`),
/// then the step index at `STEP` minus one selects one of seventeen cases;
/// anything else returns the compare pointer with no further calls.
///
/// Every case first logs its step name through the log callee (object at
/// file address `LOG_OBJ`). The cases then adjust one piece of state:
/// three toggle a flag byte on the menu object, one clears two, and the
/// rest edit bytes, words or the bit-charge word (`DIRTY`) on the state
/// object, several clamping a stepped value (division remainders,
/// wrap-around modulo 4, saturation against the limits at file addresses
/// `LIM_A`, `LIM_B`, `LIM_C`) and most comparing against the compare
/// object (or a constant when it is null) to decide whether a notify
/// callee runs. Case 14's modulo-4 stepping loops until the value leaves
/// {1, 2}; it terminates for every step delta the contract supplies (a
/// positive multiple of 4 would loop forever in the original too, so the
/// contract excludes multiples of 4). The default returns the compare
/// pointer; every other path returns whatever the last value-setting
/// instruction left in EAX (a toggled byte, a compared byte or word, a
/// remainder fixup, or a scripted callee answer), mirrored exactly.
///
/// Integer division truncates toward zero and the modulo-4 idiom equals
/// Rust's `%` (checked separately over the whole range); byte stores keep
/// the low 8 bits.
///
/// Original: 0x00D73300 (thiscall, one stack argument, callee pops 4).
lf_checker_rt::export!(thiscall, rw_00d73300(this: u32, step: u32) -> u32 {
    unsafe {
        const INNER: u32 = 0x04;
        const STEP: u32 = 0x3c;
        const TOGGLE_A: u32 = 0x50;
        const TOGGLE_B: u32 = 0x51;
        const TOGGLE_C: u32 = 0x52;
        const TOGGLE_D: u32 = 0x53;
        const AUX_FLAG: u32 = 0x54;
        const DIRTY: u32 = 0x0c;
        const LOG_OBJ: u32 = 0x01176888;
        const RESOLVER_OBJ: u32 = 0x0103E498;
        const PLACE_OBJ: u32 = 0x0116BFF0;
        const USE_OBJ: u32 = 0x01033130;
        const LIM_A: u32 = 0x01797674;
        const LIM_B: u32 = 0x01176D2C;
        const LIM_C: u32 = 0x01176D28;
        const RESOLVER_MARK: u32 = 0x39b;

        const C_LOOKUP_ST: u32 = 1;
        const C_LOOKUP_CMP: u32 = 2;
        const C_LOG: u32 = 3;
        const C_NOTIFY: u32 = 4;
        const C_RESOLVE: u32 = 5;
        const C_RESOLVE3: u32 = 6;
        const C_CHAIN: u32 = 7;
        const C_FINISH_A: u32 = 8;
        const C_FINISH_B: u32 = 9;
        const C_PLACE: u32 = 10;
        const C_USE: u32 = 11;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write(v) }
        }
        #[inline(always)]
        unsafe fn or32(a: u32, v: u32) {
            unsafe { wr32(a, rd32(a) | v) }
        }
        #[inline(always)]
        unsafe fn or8(a: u32, v: u8) {
            unsafe { wr8(a, rd8(a) | v) }
        }

        let inner = rd32(this + INNER);
        let st: u32 = lf_checker_rt::callee_thiscall!(C_LOOKUP_ST, u32, inner);
        let cmp: u32 = lf_checker_rt::callee_thiscall!(C_LOOKUP_CMP, u32, inner);
        let flag_a = rd8(st) == 9;
        let flag_b = rd8(st) == 10;
        let case = rd32(this + STEP).wrapping_sub(1);
        if case > 0x10 {
            return cmp;
        }
        let log_obj = lf_checker_rt::relocated(LOG_OBJ);
        let arg = step as i32;

        match case {
            // Toggle flag A.
            0 => {
                lf_checker_rt::callee_thiscall!(C_LOG, u32, log_obj, lf_checker_rt::relocated(0x00EEAF8C));
                let t = (rd8(this + TOGGLE_A) == 0) as u8;
                wr8(this + TOGGLE_A, t);
                return t as u32;
            }
            // Step the byte at +8 by the delta modulo 21.
            1 => {
                lf_checker_rt::callee_thiscall!(C_LOG, u32, log_obj, lf_checker_rt::relocated(0x00EEB01C));
                let r = ((rd8(st + 8) as i32).wrapping_add(arg)) % 0x15;
                let dl = if r < 0 { 0x14 } else { r } as u8;
                wr8(st + 8, dl);
                let same = if cmp != 0 { rd8(cmp + 8) == dl } else { dl == 0 };
                let mut notified = false;
                if !same {
                    lf_checker_rt::callee_thiscall!(C_NOTIFY, u32, inner, 0);
                    notified = true;
                }
                if dl != 0 {
                    return if notified { 0 } else { 0x14 };
                }
                lf_checker_rt::callee_cdecl!(C_FINISH_A, u32,);
                lf_checker_rt::callee_cdecl!(C_FINISH_B, u32, 1);
            }
            // Toggle flag B.
            2 => {
                lf_checker_rt::callee_thiscall!(C_LOG, u32, log_obj, lf_checker_rt::relocated(0x00EEAFD4));
                let t = (rd8(this + TOGGLE_B) == 0) as u8;
                wr8(this + TOGGLE_B, t);
                return t as u32;
            }
            // Step the byte at +1 by 25 times the delta modulo 250.
            3 => {
                lf_checker_rt::callee_thiscall!(C_LOG, u32, log_obj, lf_checker_rt::relocated(0x00EEB040));
                let r = arg.wrapping_mul(0x19).wrapping_add(rd8(st + 1) as i32) % 0xfa;
                let dl = (if r > 0 { r } else if arg < 0 { 0xe1 } else { r } & 0xff) as u8;
                let h: u32 = lf_checker_rt::callee_thiscall!(
                    C_PLACE, u32, lf_checker_rt::relocated(PLACE_OBJ),
                    lf_checker_rt::relocated(0x00EEB064), 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0xffff_ffff);
                wr8(st + 1, dl);
                lf_checker_rt::callee_thiscall!(C_USE, u32, lf_checker_rt::relocated(USE_OBJ), h);
                if rd8(st + 1) == 0 {
                    wr8(st + 1, 0x19);
                }
                let v = rd8(st + 1);
                if cmp != 0 {
                    let cv = rd8(cmp + 1);
                    if cv != v {
                        lf_checker_rt::callee_thiscall!(C_NOTIFY, u32, inner, 1);
                    } else {
                        return cv as u32;
                    }
                } else if v != 0x64 {
                    lf_checker_rt::callee_thiscall!(C_NOTIFY, u32, inner, 1);
                }
            }
            // Toggle flag C.
            4 => {
                lf_checker_rt::callee_thiscall!(C_LOG, u32, log_obj, lf_checker_rt::relocated(0x00EEAFB0));
                let t = (rd8(this + TOGGLE_C) == 0) as u8;
                wr8(this + TOGGLE_C, t);
                return t as u32;
            }
            // Clamp the word at +0x20 into [0 - LIM_B, 8 - LIM_B].
            5 => {
                lf_checker_rt::callee_thiscall!(C_LOG, u32, log_obj, lf_checker_rt::relocated(0x00EEB14C));
                let lim = rd32(lf_checker_rt::relocated(LIM_B)) as i32;
                let v = (rd32(st + 0x20) as i32).wrapping_add(arg);
                let lv = lim.wrapping_add(v);
                let nv = if lv > 8 {
                    0
                } else if v < 0 {
                    8i32.wrapping_sub(lim)
                } else {
                    v
                };
                or32(st + DIRTY, 8);
                wr32(st + 0x20, nv as u32);
                return lv as u32;
            }
            // Clamp the word at +0x24 into [0 - LIM_C, 8 - LIM_C].
            6 => {
                lf_checker_rt::callee_thiscall!(C_LOG, u32, log_obj, lf_checker_rt::relocated(0x00EEB170));
                let lim = rd32(lf_checker_rt::relocated(LIM_C)) as i32;
                let v = (rd32(st + 0x24) as i32).wrapping_add(arg);
                let lv = lim.wrapping_add(v);
                let nv = if lv > 8 {
                    0
                } else if v < 0 {
                    8i32.wrapping_sub(lim)
                } else {
                    v
                };
                or32(st + DIRTY, 0x10);
                wr32(st + 0x24, nv as u32);
                return lv as u32;
            }
            // Flip bit 2 of the byte at +3.
            7 => {
                lf_checker_rt::callee_thiscall!(C_LOG, u32, log_obj, lf_checker_rt::relocated(0x00EEB194));
                let a = rd8(st + 3);
                let mut c = a >> 2;
                c = !c;
                c <<= 2;
                c ^= a;
                c &= 4;
                c ^= a;
                or32(st + DIRTY, 0x20);
                wr8(st + 3, c);
                return a as u32;
            }
            // Clear flag D (and the aux flag with it); setting is one-shot.
            8 => {
                lf_checker_rt::callee_thiscall!(C_LOG, u32, log_obj, lf_checker_rt::relocated(0x00EEAFF8));
                let t = (rd8(this + TOGGLE_D) == 0) as u8;
                wr8(this + TOGGLE_D, t);
                if t == 0 {
                    wr8(this + AUX_FLAG, 0);
                }
                return t as u32;
            }
            // Reset the head state, then notify unless the compare agrees.
            9 => {
                lf_checker_rt::callee_thiscall!(C_LOG, u32, log_obj, lf_checker_rt::relocated(0x00EEB7F4));
                if flag_a || flag_b {
                    wr8(this + AUX_FLAG, 0);
                    wr8(st, 8);
                    wr8(st + 4, 0);
                    if rd8(st + 5) == 3 {
                        wr8(st + 5, 0);
                    }
                } else {
                    wr8(st, 9);
                    wr8(st + 4, 1);
                }
                let v = rd8(st + 4);
                if cmp != 0 {
                    let cv = rd8(cmp + 4);
                    if cv != v {
                        lf_checker_rt::callee_thiscall!(C_NOTIFY, u32, inner, 6);
                    } else {
                        return cv as u32;
                    }
                } else if v != 8 {
                    lf_checker_rt::callee_thiscall!(C_NOTIFY, u32, inner, 6);
                }
            }
            // Step the head byte by the delta modulo 9 with flag fixups.
            10 => {
                lf_checker_rt::callee_thiscall!(C_LOG, u32, log_obj, lf_checker_rt::relocated(0x00EEAF68));
                let dl: i32 = if flag_a {
                    0xa
                } else if flag_b {
                    9
                } else {
                    let r = ((rd8(st) as i32).wrapping_add(arg)) % 9;
                    if r >= 1 {
                        r
                    } else if arg > 0 {
                        1
                    } else if arg >= 0 {
                        r
                    } else {
                        8
                    }
                };
                let dlb = (dl & 0xff) as u8;
                wr8(st, dlb);
                if flag_a || flag_b {
                    let b5 = rd8(st + 5);
                    if dlb == 9 {
                        if b5 == 1 || b5 == 2 {
                            wr8(st + 5, 0);
                        }
                    } else if b5 == 3 {
                        wr8(st + 5, 0);
                    }
                }
                if dlb != 9 && rd8(st + 3) & 2 == 0 {
                    let p: u32 = lf_checker_rt::callee_thiscall!(
                        C_RESOLVE, u32, lf_checker_rt::relocated(RESOLVER_OBJ));
                    or8(p + RESOLVER_MARK, 2);
                }
                if rd8(st) == 10 && rd8(st + 5) == 3 {
                    wr8(st + 5, 2);
                }
                // EAX is reloaded from the head byte here; earlier answers
                // no longer matter.
                if rd8(st) != 9 {
                    return (rd8(st) as i32 - 1) as u32;
                }
                let b5 = rd8(st + 5);
                if b5 != 2 && b5 != 1 {
                    return b5 as u32;
                }
                wr8(st + 5, 3);
                return b5 as u32;
            }
            // Re-resolve the object at +0x10, then notify on change.
            11 => {
                lf_checker_rt::callee_thiscall!(C_LOG, u32, log_obj, lf_checker_rt::relocated(0x00EEB074));
                let p: u32 = lf_checker_rt::callee_thiscall!(
                    C_RESOLVE3, u32, lf_checker_rt::relocated(RESOLVER_OBJ), st, rd32(st + 0x10), step);
                lf_checker_rt::callee_thiscall!(C_CHAIN, u32, p);
                let v = rd32(st + 0x10);
                if cmp != 0 {
                    let cv = rd32(cmp + 0x10);
                    if cv != v {
                        lf_checker_rt::callee_thiscall!(C_NOTIFY, u32, inner, 8);
                    } else {
                        return cv;
                    }
                } else if v != 0xffff_ffff {
                    lf_checker_rt::callee_thiscall!(C_NOTIFY, u32, inner, 8);
                }
            }
            // Step the byte at +9 by the delta modulo 3.
            12 => {
                lf_checker_rt::callee_thiscall!(C_LOG, u32, log_obj, lf_checker_rt::relocated(0x00EEB0E0));
                let r = ((rd8(st + 9) as i32).wrapping_add(arg)) % 3;
                let dl = (if r < 0 { 2 } else { r } & 0xff) as u8;
                or32(st + DIRTY, 0x2000);
                wr8(st + 9, dl);
                return 2;
            }
            // Step the byte at +2 by five times the delta, clamped.
            13 => {
                lf_checker_rt::callee_thiscall!(C_LOG, u32, log_obj, lf_checker_rt::relocated(0x00EEB098));
                let b2 = rd8(st + 2);
                let lim = rd32(lf_checker_rt::relocated(LIM_A)) as i32;
                let v = arg.wrapping_mul(5).wrapping_add(b2 as i32);
                let (nv, retv): (u8, u32) = if v < 0xf && arg < 0 {
                    ((lim & 0xff) as u8, b2 as u32)
                } else if v > lim {
                    ((((if arg > 0 { 0x0f } else { v }) & 0xff) as u8), 0x0f)
                } else {
                    ((v & 0xff) as u8, b2 as u32)
                };
                or32(st + DIRTY, 0x200);
                wr8(st + 2, nv);
                return retv;
            }
            // Step the byte at +5 by the delta modulo 4, skipping 1 and 2.
            14 => {
                lf_checker_rt::callee_thiscall!(C_LOG, u32, log_obj, lf_checker_rt::relocated(0x00EEB104));
                let mut c = ((rd8(st + 5) as i32).wrapping_add(arg)) % 4;
                if !flag_a {
                    if c == 3 {
                        c = (arg.wrapping_add(3)) % 4;
                    }
                } else {
                    while c == 1 || c == 2 {
                        c = (c.wrapping_add(arg)) % 4;
                    }
                }
                if c < 0 {
                    c = 3;
                    if !flag_a {
                        c = (arg.wrapping_add(3)) % 4;
                    } else {
                        while c == 1 || c == 2 {
                            c = (c.wrapping_add(arg)) % 4;
                        }
                    }
                }
                let al = cmp != 0 && (rd8(cmp) == 9 || rd8(cmp) == 10);
                let aux = if c != 0 {
                    1
                } else if !al {
                    0
                } else if rd8(cmp + 5) != 0 {
                    1
                } else {
                    0
                };
                wr8(this + AUX_FLAG, aux);
                or32(st + DIRTY, 0x800);
                wr8(st + 5, (c & 0xff) as u8);
                return aux as u32;
            }
            // Step the byte at +6 by the delta modulo 4, gated by +5.
            15 => {
                lf_checker_rt::callee_thiscall!(C_LOG, u32, log_obj, lf_checker_rt::relocated(0x00EEB128));
                let b5 = rd8(st + 5);
                let nv: u8 = if b5 != 0 && cmp != 0 && rd8(cmp + 5) != 0 {
                    let r = ((rd8(st + 6) as i32).wrapping_add(arg)) % 4;
                    (if r < 0 { 3 } else { r } & 0xff) as u8
                } else if b5 != 0 {
                    if rd8(st + 6) == 0 { 2 } else { 0 }
                } else if cmp != 0 && rd8(cmp + 5) != 0 {
                    if rd8(st + 6) == 0 { 1 } else { 0 }
                } else {
                    0
                };
                or32(st + DIRTY, 0x1000);
                wr8(st + 6, nv);
                return nv as u32;
            }
            // Step the byte at +7 by the delta modulo 4.
            16 => {
                lf_checker_rt::callee_thiscall!(C_LOG, u32, log_obj, lf_checker_rt::relocated(0x00EEB0BC));
                let r = ((rd8(st + 7) as i32).wrapping_add(arg)) % 4;
                let dl = (if r < 0 { 3 } else { r } & 0xff) as u8;
                or32(st + DIRTY, 0x400);
                wr8(st + 7, dl);
                return dl as u32;
            }
            _ => {}
        }
        0
    }
});
