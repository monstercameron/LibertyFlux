// original: 0x00CD0090 melee_create_action (proposed)

/// Pick and create a melee attack action for a ped task object.
///
/// `obj` points to a task/ped object the function reads but never writes:
/// readiness flags (byte at `+PED_READY0` clear and byte at `+PED_READY1` set
/// mean the "wanted" path), and a two-level link at `+PED_LINK` used by the
/// fallback. `bval` and `cval` are opaque values forwarded to the creation
/// callees; only the low byte of `flags` is read (nonzero selects the
/// fallback creation attempt). The function preserves its caller's ECX and
/// returns its result in EAX (cdecl, four stack words).
///
/// Behaviour: when the mode global reads 2 and the gate check answers
/// nonzero while the gate byte is set, there is nothing to do (returns 0).
/// Otherwise a readiness flag is derived from the two flag bytes and a
/// twelve-argument factory call builds a candidate from the fixed factory
/// object; a second call resolves it. On the wanted path the candidate is
/// refined through two more creation calls and a four-argument combine call
/// whose `this` is the caller's preserved ECX; on the other path one
/// creation call suffices. A null at any step, or a null combine result,
/// falls through to the fallback: unless `flags` is set, return 0; walk the
/// link chain (a null link skips the lookup), check the looked-up tag word
/// equals 1, and make the final creation call, whose answer is returned.
/// One dead store of an intermediate answer to scratch is not reproduced.
///
/// Original: 0x00CD0090 (cdecl, four stack words; ECX preserved and reused
/// as `this` for two calls, which the contract leaves uncompared).
lf_checker_rt::export!(cdecl, rw_00CD0090(obj: u32, bval: u32, cval: u32, flags: u32) -> u32 {
    // Fallback creation path, shared by every null above.
    unsafe fn fallback(obj: u32, bval: u32, cval: u32, flags: u32) -> u32 {
        unsafe {
            const PED_LINK: u32 = 0x2c4;
            const LINK_INNER: u32 = 0x25c;
            const LINK_ARG: u32 = 0x18;
            const LOOKUP_TAG: u32 = 0x0c;
            const CREATOR_THIS: u32 = 0x167e2a0;
            if (flags & 0xff) == 0 {
                return 0;
            }
            let mut e = ((obj + PED_LINK) as *const u32).read_unaligned();
            if e != 0 {
                e = ((e + LINK_INNER) as *const u32).read_unaligned();
            }
            if e != 0 {
                let arg = ((e + LINK_ARG) as *const u32).read_unaligned();
                let found: u32 = lf_checker_rt::callee_cdecl!(14, u32, arg);
                if ((found + LOOKUP_TAG) as *const u32).read_unaligned() != 1 {
                    return 0;
                }
            }
            let t: u32 = lf_checker_rt::callee_thiscall!(
                8, u32, lf_checker_rt::global::<u32>(CREATOR_THIS).read()
            );
            if t == 0 {
                return 0;
            }
            lf_checker_rt::callee_thiscall!(
                11, u32, t, 0u32, 0u32, 0u32, 0u32, 1u32, cval, bval
            )
        }
    }

    unsafe {
        const MODE_VALUE: u32 = 0x11d6fd4;
        const MODE_WANTED: u32 = 2;
        const GATE_BYTE: u32 = 0x18b6ed7;
        const CREATOR_THIS: u32 = 0x167e2a0;
        const FACTORY_OBJ: u32 = 0x171c968;
        const KIND_TAG: u32 = 0x17a4cd4;
        const PED_READY0: u32 = 0x218;
        const PED_READY1: u32 = 0x219;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn creator() -> u32 {
            unsafe { lf_checker_rt::global::<u32>(CREATOR_THIS).read() }
        }

        // Gate: mode 2 with a live gate check and a set gate byte means idle.
        if rd32(lf_checker_rt::relocated(MODE_VALUE)) == MODE_WANTED {
            let live: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
            if (live as u8) != 0 && rd8(lf_checker_rt::relocated(GATE_BYTE)) != 0 {
                return 0;
            }
        }

        let want = u32::from(rd8(obj + PED_READY0) == 0 && rd8(obj + PED_READY1) != 0);
        let cand: u32 = lf_checker_rt::callee_thiscall!(
            2, u32, lf_checker_rt::relocated(FACTORY_OBJ),
            1u32, lf_checker_rt::relocated(KIND_TAG), 1u32, 1u32, obj,
            0u32, 0u32, 0u32, 0u32, cval, bval, want
        );
        if cand == 0 {
            return fallback(obj, bval, cval, flags);
        }
        let mut handle: u32 = lf_checker_rt::callee_thiscall!(3, u32, cand);
        if handle == 0 {
            return fallback(obj, bval, cval, flags);
        }
        // The original stores one intermediate answer to scratch and never
        // reads it back; only its null check below matters.
        if want != 0 {
            if lf_checker_rt::callee_thiscall!(4, u32, creator()) == 0 {
                return fallback(obj, bval, cval, flags);
            }
            let t2 = lf_checker_rt::callee_thiscall!(5, u32, creator());
            if t2 != 0 {
                handle = lf_checker_rt::callee_thiscall!(
                    9, u32, t2, handle, 0u32, 0u32, 1u32, 0u32, cval, bval
                );
            } else {
                handle = 0;
            }
            let t3 = lf_checker_rt::callee_thiscall!(6, u32, creator());
            let aux = if t3 != 0 {
                lf_checker_rt::callee_thiscall!(12, u32, t3, cval, bval)
            } else {
                0
            };
            // The original passes its preserved entry ECX as `this` here; a
            // cdecl rewrite cannot read it, so the contract leaves this
            // register uncompared (see report) and the rewrite passes 0.
            let combined: u32 =
                lf_checker_rt::callee_thiscall!(13, u32, 0u32, aux, handle, 1u32, 0u32);
            if combined != 0 {
                return combined;
            }
            return fallback(obj, bval, cval, flags);
        }
        let t = lf_checker_rt::callee_thiscall!(7, u32, creator());
        if t == 0 {
            return fallback(obj, bval, cval, flags);
        }
        let made: u32 =
            lf_checker_rt::callee_thiscall!(10, u32, t, handle, 0u32, 0u32, 1u32, 0u32, cval, bval);
        if made != 0 {
            return made;
        }
        fallback(obj, bval, cval, flags)
    }
});
