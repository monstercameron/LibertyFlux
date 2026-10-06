// original: 0x00660C30 rage::snJoinGamersToRlineTask::vf7

/// Task step for joining gamers to a session line: either flag new members
/// or resolve pending ones, then release the shared session object.
///
/// `this` is the task: dword at `+0x60` is the session context, dword at
/// `+0x540` a SIGNED member count, dword at `+0x94` a state flag. `a0`
/// selects the phase (1 = flag, anything else = resolve); `a1` is forwarded
/// to the member and release callees.
///
/// Flag phase: for each of the `count` members, look it up through the
/// session callee (thiscall on the context with the member slot at
/// `+0xa0`, stride 16); when the member's flag byte (at `+0x520`) is clear
/// and the lookup answers non-null, set bit 1 of the answer's byte at
/// `+0x80`.
///
/// Resolve phase: for each member whose state word (at `+0x420`, stride 8)
/// is 1, ask the resolver callee (thiscall, three stack words: an output
/// triple, an input pointer preset to the member slot, zero) for the
/// member's session triple. The resolver runs against the alternate session
/// object when the image flag byte is set, else against nothing. When the
/// triple's middle word is non-null and the member slot is at or above the
/// triple's first word (UNSIGNED comparison), read the middle word's object:
/// call its virtual slot `+0x28`; when that answers the join magic (an image
/// address, compared relocated) call the member slot `+0x1c` with (`a0`, 0),
/// else call slot `+0x28` once more and call `+0x1c` only when it answers the
/// rejoin magic.
///
/// Tail: when the state flag is 1 and the context's slot at `+0x898` holds
/// an object linked back to the flag, call the function at its virtual slot
/// `+8`; when that answers nonzero, call slot `+0xc`, then slot `+0` with
/// (0), then — when the image object word is non-null — its virtual slot
/// `+0xc` with the object, and clear the slot. Finally call the release
/// callee (thiscall on the task with (`a0`, `a1`)), clear the context word
/// and return the release callee's answer.
///
/// Original: 0x00660C30 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00660C30(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const CTX: u32 = 0x60;
        const COUNT: u32 = 0x540;
        const STATE: u32 = 0x94;
        const SLOT_A: u32 = 0xa0;
        const STRIDE_A: u32 = 16;
        const FLAG_A: u32 = 0x520;
        const SLOT_B: u32 = 0x420;
        const STRIDE_B: u32 = 8;
        const TAIL_SLOT: u32 = 0x898;
        const LOOKUP_ID: u32 = 1;
        const RESOLVE_ID: u32 = 2;
        const RELEASE_ID: u32 = 9;
        const GET_MAGIC_SLOT: u32 = 0x28;
        const MEMBER_SLOT: u32 = 0x1c;
        const MAGIC_JOIN: u32 = 0x01C9_A0B0;
        const MAGIC_REJOIN: u32 = 0x01C9_A0B8;
        const FLAG_BYTE: u32 = 0x018B_8309;
        const ALT_SESSION: u32 = 0x019F_3A10;
        const GOBJ: u32 = 0x018B_8304;

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

        if a0 == 1 {
            let n = rd32(this.wrapping_add(COUNT)) as i32;
            if n > 0 {
                let ctx = rd32(this.wrapping_add(CTX));
                for i in 0..n {
                    let el = this.wrapping_add(SLOT_A).wrapping_add((i as u32).wrapping_mul(STRIDE_A));
                    let r: u32 = lf_checker_rt::callee_thiscall!(LOOKUP_ID, u32, ctx, el);
                    if rd8(this.wrapping_add(i as u32).wrapping_add(FLAG_A)) == 0 && r != 0 {
                        let a = r.wrapping_add(0x80);
                        wr8(a, rd8(a) | 2);
                    }
                }
            }
        } else {
            let n = rd32(this.wrapping_add(COUNT)) as i32;
            if n > 0 {
                for j in 0..n {
                    let el = this.wrapping_add(SLOT_B).wrapping_add((j as u32).wrapping_mul(STRIDE_B));
                    if rd32(el) != 1 {
                        continue;
                    }
                    let alt = if rd8(lf_checker_rt::relocated(FLAG_BYTE)) == 0 {
                        0
                    } else {
                        lf_checker_rt::relocated(ALT_SESSION)
                    };
                    let mut out = [0u32; 3];
                    let mut inp = el;
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        RESOLVE_ID,
                        u32,
                        alt.wrapping_add(0x10),
                        core::ptr::addr_of_mut!(out) as u32,
                        core::ptr::addr_of_mut!(inp) as u32,
                        0
                    );
                    let first = out[0];
                    let obj = out[1];
                    if obj == 0 || el < first {
                        continue;
                    }
                    // The original compares against relocated image addresses
                    // (both immediates carry relocation entries), not file values.
                    let want_join = lf_checker_rt::relocated(MAGIC_JOIN);
                    let want_rejoin = lf_checker_rt::relocated(MAGIC_REJOIN);
                    let vt = rd32(obj);
                    let get: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(rd32(vt.wrapping_add(GET_MAGIC_SLOT)) as usize);
                    let go = if get(obj) == want_join {
                        true
                    } else {
                        let vt2 = rd32(obj);
                        let get2: extern "thiscall" fn(u32) -> u32 =
                            core::mem::transmute(rd32(vt2.wrapping_add(GET_MAGIC_SLOT)) as usize);
                        get2(obj) == want_rejoin
                    };
                    if go {
                        let vt3 = rd32(obj);
                        let member: extern "thiscall" fn(u32, u32, u32) -> u32 =
                            core::mem::transmute(rd32(vt3.wrapping_add(MEMBER_SLOT)) as usize);
                        member(obj, a0, 0);
                    }
                }
            }
        }

        if rd32(this.wrapping_add(STATE)) == 1 {
            let ctx = rd32(this.wrapping_add(CTX));
            let t = rd32(ctx.wrapping_add(TAIL_SLOT));
            if t != 0 {
                let link = rd32(t.wrapping_add(0x0c));
                if link != 0 && link == this.wrapping_add(STATE) {
                    let f: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(rd32(rd32(t).wrapping_add(8)) as usize);
                    if (f(t) as u8) != 0 {
                        let t2 = rd32(ctx.wrapping_add(TAIL_SLOT));
                        let g: extern "thiscall" fn(u32) -> u32 =
                            core::mem::transmute(rd32(rd32(t2).wrapping_add(0x0c)) as usize);
                        g(t2);
                        let b = rd32(ctx.wrapping_add(TAIL_SLOT));
                        if b != 0 {
                            let h: extern "thiscall" fn(u32, u32) -> u32 =
                                core::mem::transmute(rd32(rd32(b)) as usize);
                            h(b, 0);
                            let go = rd32(lf_checker_rt::relocated(GOBJ));
                            if go != 0 {
                                let k: extern "thiscall" fn(u32, u32) -> u32 =
                                    core::mem::transmute(rd32(rd32(go).wrapping_add(0x0c)) as usize);
                                k(go, b);
                            }
                        }
                        wr32(ctx.wrapping_add(TAIL_SLOT), 0);
                    }
                }
            }
        }

        let r: u32 = lf_checker_rt::callee_thiscall!(RELEASE_ID, u32, this, a0, a1);
        wr32(this.wrapping_add(CTX), 0);
        r
    }
});
