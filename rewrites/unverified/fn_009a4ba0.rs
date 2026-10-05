// original: 0x009a4ba0 EP1_INTRO_MUSIC_TRACK

/// Advance one step of the intro-music track state machine.
///
/// `this` points to the track object. Dword at `+STATE` selects the step:
/// 1 looks up the current track id and either confirms it or falls back,
/// 2 refreshes the track id from the sub-object and resolves it against the
/// global track table, 3 rebuilds the track's parameter block, 4 finishes a
/// pending switch, and any other value runs the default path, which picks a
/// track id from the sibling object or the previous id and arms state 1.
/// The sub-object pointer at `+SUB` may be null (the state is then forced
/// to 0 first); the sibling pointer at `+OTHER` is always valid on the
/// paths that read it.
///
/// Track ids use two sentinels: `NONE` (0xFE, no track) and `INVALID`
/// (0xFF, unusable). All fifteen outgoing calls are intercepted by the
/// checker and answered by script; the parameter-block builder (callee 14)
/// writes twelve words through a frame pointer, and the applier (callee 15)
/// receives that block as its second stack argument.
///
/// Original: 0x009a4ba0 (thiscall, one stack word; returns the confirmed
/// track id, a quotient, or incidental values such as the incoming
/// dispatch index on early exits, exactly as the original leaves eax).
lf_checker_rt::export!(thiscall, rw_009a4ba0(this: u32, arg0: u32) -> u32 {
    unsafe {
        const SUB: u32 = 0x2c;
        const OTHER: u32 = 0x28;
        const STATE: u32 = 0x6c;
        const TRACK: u32 = 0x74;
        const OUT: u32 = 0x78;
        const PREV: u32 = 0x7c;
        const TARGET: u32 = 0x80;
        const ENABLED: u32 = 0x84;
        const HOLD: u32 = 0x85;
        const BASE: u32 = 0x1c;
        const SUB_TRACK_BIAS: u32 = 0xa20;
        const SUB_ID0: u32 = 0xd10;
        const SUB_ID1: u32 = 0xd11;
        const SUB_PARAM: u32 = 0xd18;
        const SUB_FLAG: u32 = 0xd3b;
        const NONE: u32 = 0xfe;
        const INVALID: u32 = 0xff;
        const LOOKUP_KEY: u32 = 0xd;
        const TABLE_ADDR: u32 = 0x00e9_1280;
        const BASE_BIAS: u32 = 0x3a98;

        const G_FIRST_RUN: u32 = 0x0128_4678;
        const G_TABLE_ID: u32 = 0x0128_4674;
        const G_WANT_ID: u32 = 0x0128_8534;
        const G_REMAINDER: u32 = 0x0128_45c4;
        const G_FLAGS: u32 = 0x0128_45c8;
        const G_APPLY_ARG: u32 = 0x0103_8e5c;
        const G_MODE: u32 = 0x011d_6fd4;
        const G_DIRTY: u32 = 0x011d_7628;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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
        unsafe fn grd(a: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(a).read() }
        }
        #[inline(always)]
        unsafe fn gwr(a: u32, v: u32) {
            unsafe { lf_checker_rt::global::<u32>(a).write(v) }
        }
        #[inline(always)]
        unsafe fn grb(a: u32) -> u8 {
            unsafe { lf_checker_rt::global::<u8>(a).read() }
        }
        #[inline(always)]
        unsafe fn gwb(a: u32, v: u8) {
            unsafe { lf_checker_rt::global::<u8>(a).write(v) }
        }

        // Build the parameter block through callee 14, then apply it with
        // callee 15. The twelve words are scripted by the checker on both
        // sides; the block itself lives in this frame.
        unsafe fn rebuild(this: u32) {
            unsafe {
                let mut block = [0u32; 12];
                let bp = block.as_mut_ptr() as u32;
                lf_checker_rt::callee_thiscall!(14, u32, bp);
                let apply_arg = (lf_checker_rt::global::<u32>(G_APPLY_ARG)).read();
                lf_checker_rt::callee_thiscall!(15, u32, this, apply_arg, bp, 0xffff_ffff, 0, 0);
            }
        }

        let sub = rd32(this + SUB);
        let mut handle: u32;
        if sub == 0 {
            wr32(this + STATE, 0);
            handle = 0;
        } else {
            handle = sub.wrapping_add(SUB_TRACK_BIAS);
        }
        let dispatch = rd32(this + STATE).wrapping_sub(1);
        if dispatch > 3 {
            // Default path.
            lf_checker_rt::callee_thiscall!(7, u32, this);
            if rd8(this + ENABLED) == 0 {
                return 0;
            }
            if rd8(this + HOLD) != 0 {
                return 0;
            }
            let other = rd32(this + OTHER);
            let edi = this + TRACK;
            let mut eax = rd8(other + SUB_ID0) as u32;
            wr32(edi, eax);
            if rd8(other + SUB_FLAG) != 0 && grb(G_FLAGS) == 0 {
                if eax == NONE || eax == INVALID {
                    return eax;
                }
                wr32(this + STATE, 2);
                return eax;
            }
            if eax != NONE && eax != INVALID {
                wr32(this + STATE, 2);
                return eax;
            }
            if grd(G_MODE) != 2 && grb(G_DIRTY) != 0 {
                let r8: u32 = lf_checker_rt::callee_cdecl!(8, u32);
                eax = r8.wrapping_sub(1);
                wr32(edi, eax);
                lf_checker_rt::callee_thiscall!(13, u32, this);
                gwb(G_DIRTY, 0);
            } else {
                let alt = rd8(other + SUB_ID1);
                if alt == 0xff || alt == 0xfe {
                    // Re-resolve through the lookup pair; a still-invalid id
                    // parks the machine in state 0, otherwise continue with
                    // eax holding callee 13's answer (scripted 0).
                    let prev = rd32(this + PREV);
                    let mut again = prev == NONE || prev == INVALID;
                    if !again && arg0 >= rd32(this + BASE).wrapping_add(BASE_BIAS) {
                        again = true;
                    }
                    if again {
                        let r1: u32 =
                            lf_checker_rt::callee_thiscall!(1, u32, this, rd32(other + SUB_PARAM));
                        lf_checker_rt::callee_cdecl!(2, u32, edi, r1);
                        if rd32(edi) == INVALID {
                            wr32(edi, INVALID);
                            wr32(this + STATE, 0);
                            return 0;
                        }
                        lf_checker_rt::callee_thiscall!(13, u32, this);
                    } else {
                        wr32(edi, prev);
                        lf_checker_rt::callee_thiscall!(13, u32, this);
                    }
                    eax = 0;
                } else {
                    eax = alt as u32;
                    wr32(edi, eax);
                }
            }
            if grb(G_FLAGS + 1) != 0 {
                wr32(this + STATE, 1);
                return eax;
            }
            rebuild(this);
            wr32(this + STATE, 1);
            return 0;
        }
        if dispatch == 0 {
            // State 1: confirm the current track id.
            let edi = this + TRACK;
            if rd32(edi) == NONE {
                let r1: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, LOOKUP_KEY);
                lf_checker_rt::callee_cdecl!(2, u32, edi, r1);
            }
            let r3: u32 = lf_checker_rt::callee_cdecl!(3, u32, rd32(edi), handle, 7);
            handle = r3;
            let sub2 = rd32(this + SUB);
            let bl = r3 as u8;
            wr8(sub2 + SUB_ID0, bl);
            if (bl as u32) < NONE {
                wr8(sub2 + SUB_ID1, bl);
            }
            let r4: u32 = lf_checker_rt::callee_cdecl!(4, u32, rd32(edi));
            if handle == rd32(edi) && r4 != 0 {
                let r5: u32 = lf_checker_rt::callee_thiscall!(5, u32, r4);
                if (r5 & 0xff) != 0 {
                    let confirmed = rd32(edi);
                    wr32(this + STATE, 2);
                    wr32(this + OUT, confirmed);
                    return confirmed;
                }
            }
            lf_checker_rt::callee_thiscall!(6, u32, this);
            let fallback = rd32(edi);
            wr32(this + OUT, fallback);
            return fallback;
        }
        if dispatch == 1 {
            // State 2: refresh the id, then resolve it.
            lf_checker_rt::callee_thiscall!(7, u32, this);
            let sub2 = rd32(this + SUB);
            wr32(this + TRACK, rd8(sub2 + SUB_ID0) as u32);
            let r8: u32 = lf_checker_rt::callee_cdecl!(8, u32);
            if rd32(this + TRACK) >= r8 {
                wr32(this + TRACK, 0);
            }
            let track = rd32(this + TRACK);
            wr32(this + OUT, track);
            wr32(this + PREV, track);
            let mut eax: u32;
            let first = grd(G_FIRST_RUN);
            if first & 1 != 0 {
                eax = grd(G_TABLE_ID);
            } else {
                gwr(G_FIRST_RUN, first | 1);
                let table = lf_checker_rt::relocated(TABLE_ADDR);
                let r9: u32 = lf_checker_rt::callee_cdecl!(9, u32, table, 0);
                gwr(G_TABLE_ID, r9);
                eax = r9;
            }
            if grd(G_WANT_ID) != eax {
                if rd8(this + ENABLED) == 0 {
                    wr32(this + STATE, 0);
                    return eax;
                }
                wr32(this + BASE, arg0);
                let rem = grd(G_REMAINDER);
                if rem > 0 {
                    let d: u32 = lf_checker_rt::callee_cdecl!(8, u32);
                    let n = rd32(this + TRACK).wrapping_add(grd(G_REMAINDER));
                    wr32(this + STATE, 4);
                    wr32(this + OUT, n.wrapping_rem(d));
                    return n.wrapping_div(d);
                }
                if rd32(this + TARGET) == NONE {
                    return arg0;
                }
                let d: u32 = lf_checker_rt::callee_cdecl!(8, u32);
                let n = rd32(this + TARGET)
                    .wrapping_sub(rd32(this + TRACK))
                    .wrapping_add(d);
                gwr(G_REMAINDER, n.wrapping_rem(d));
                let target = rd32(this + TARGET);
                wr32(this + OUT, target);
                wr32(this + STATE, 4);
                wr32(this + TARGET, NONE);
                return target;
            }
            wr32(this + STATE, 3);
            return eax;
        }
        if dispatch == 2 {
            // State 3: rebuild the parameter block (eax holds 2 here).
            if rd8(this + ENABLED) == 0 {
                wr32(this + STATE, 0);
                return 2;
            }
            if grb(G_FLAGS + 1) == 0 {
                rebuild(this);
            }
            lf_checker_rt::callee_cdecl!(10, u32, handle);
            wr32(this + TRACK, INVALID);
            wr32(this + OUT, INVALID);
            wr32(this + STATE, 0);
            return 0;
        }
        // State 4: finish a pending switch (eax holds 3 on entry, but every
        // path below sets it before returning).
        lf_checker_rt::callee_thiscall!(6, u32, this);
        if grd(G_REMAINDER) > 0 {
            let d: u32 = lf_checker_rt::callee_cdecl!(8, u32);
            let n = rd32(this + TRACK).wrapping_add(grd(G_REMAINDER));
            wr32(this + OUT, n.wrapping_rem(d));
            lf_checker_rt::callee_cdecl!(11, u32, handle);
            return 0;
        }
        let r12: u32 = lf_checker_rt::callee_cdecl!(12, u32, handle);
        if (r12 & 0xff) == 0 {
            wr32(this + STATE, 2);
        }
        r12
    }
});
