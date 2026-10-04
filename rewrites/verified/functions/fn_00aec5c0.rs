// original: 0x00aec5c0 timing_step_first (proposed)

/// First step dispatcher: refresh the row selector, then run one action.
///
/// Thiscall with five stack words (`obj`, `f`, `a`, `b`, `sel`); `this` is a
/// controller with a mode word at `+4`, a peer at `+8` and flags at `+0xc`.
/// The refresh callee takes the incoming `sel` with three out-pointers and
/// rewrites `sel` in place. If the row at `obj+0x40[sel]` is clear the step
/// returns 0, otherwise it
/// picks the mode object (`[this+4]` when its word at `+0x28` has `0x80` in
/// the `0x3c0` field), asks the ready callee, and derives two condition
/// bytes: `c1` (ready and flags not aligned) and `c2` (`c1` and flag bit 1).
/// A global gate decides the argument (1 or 0) of the fence callee, which
/// runs before and after the action. The action runs when `a` is nonzero
/// (through the mode object when present, else virtual slot `+0x30`/`+0x28`
/// on `obj` by `c1`/`c2`), when `a` is zero but `b` is not (same shape, with
/// the float bits as leading argument), or when both are zero (slot `+0x2c`
/// when `c1` is set without `c2`, else slot `+0x20`); a set row is
/// re-checked on every path and skips the action silently. Returns `al` 1
/// after the action or the silent skip.
///
/// The original spills the condition bytes and the updated selector over its
/// own incoming argument slots; the rewrite uses locals (every value stays
/// observed through the outgoing calls). The refresh callee's scratch words
/// are overwritten before any read; only the selector survives. The
/// meaningful return is the low byte; the upper bytes are stub residue.
lf_checker_rt::export!(thiscall, rw_00aec5c0(this: u32, obj: u32, f: u32, a: u32, b: u32, sel: u32) -> u32 {
    unsafe {
        const ROWS: u32 = 0x40;
        const THIS_MODE: u32 = 0x04;
        const THIS_PEER: u32 = 0x08;
        const THIS_FLAGS: u32 = 0x0c;
        const MODEW: u32 = 0x28;
        const MODE_THIS: u32 = 0x34;
        const GATE: u32 = 0x0103f96c;
        const VT_A: u32 = 0x30;
        const VT_B: u32 = 0x28;
        const VT_C: u32 = 0x2c;
        const VT_D: u32 = 0x20;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn vcall5(slot: u32, obj: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) {
            unsafe {
                let f: extern "thiscall" fn(u32, u32, u32, u32, u32, u32) =
                    core::mem::transmute(rd32(rd32(obj) + slot) as usize);
                f(obj, a0, a1, a2, a3, a4);
            }
        }
        #[inline(always)]
        unsafe fn vcall4(slot: u32, obj: u32, a0: u32, a1: u32, a2: u32, a3: u32) {
            unsafe {
                let f: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + slot) as usize);
                f(obj, a0, a1, a2, a3);
            }
        }

        let mut sel_w = sel;
        let mut scratch = 0u32;
        let mut scratch2 = 0u32;
        // Five stack words: the slot above the four explicit pushes carries
        // the incoming selector itself (loaded from the fifth argument slot,
        // not the float); the callee pops 0x14.
        lf_checker_rt::callee_thiscall!(
            1, u32, this, obj, &mut sel_w as *mut u32 as u32,
            &mut scratch2 as *mut u32 as u32, &mut scratch as *mut u32 as u32, sel,
        );
        let _ = scratch2;
        let _ = scratch;
        let row = || unsafe { rd32(obj + ROWS + sel_w.wrapping_mul(4)) };
        if row() == 0 {
            return 0;
        }
        let mode = rd32(this + THIS_MODE);
        let picked = if mode != 0 && rd32(mode + MODEW) & 0x3c0 == 0x80 {
            mode
        } else {
            0
        };
        let flags = rd32(this + THIS_FLAGS);
        let ready: u32 = lf_checker_rt::callee_thiscall!(2, u32, mode);
        let c1 = ready != 0 && flags & 3 != 0;
        let c2 = c1 && (flags.wrapping_shr(1) & 1) != 0;
        let gate = (lf_checker_rt::global::<u8>(GATE)).read();
        let peer_val = rd32(this + THIS_PEER);
        if gate != 0 {
            lf_checker_rt::callee_cdecl!(3, u32, 1u32);
        }
        if a != 0 {
            if row() != 0 {
                if picked != 0 && rd32(picked + MODE_THIS) != 0 {
                    // Last word is the flags: the original stores them over
                    // the refresh callee's scratch slot before reading it here.
                    lf_checker_rt::callee_thiscall!(
                        4, u32, rd32(picked + MODE_THIS), 0u32, a, peer_val, sel_w, flags,
                    );
                } else if c1 {
                    vcall5(VT_A, obj, 0u32, a, sel_w, if c2 { 1u32 } else { 2u32 }, peer_val);
                } else {
                    vcall4(VT_B, obj, 0u32, a, peer_val, sel_w);
                }
            }
        } else if b != 0 {
            if row() != 0 {
                if picked != 0 {
                    lf_checker_rt::callee_thiscall!(
                        4, u32, rd32(picked + MODE_THIS), f, b, peer_val, sel_w, flags,
                    );
                } else if c1 {
                    vcall5(VT_A, obj, f, b, sel_w, if c2 { 1u32 } else { 2u32 }, peer_val);
                } else {
                    vcall4(VT_B, obj, f, b, peer_val, sel_w);
                }
            }
        } else if row() != 0 {
            if c1 {
                if !c2 {
                    vcall4(VT_C, obj, f, sel_w, peer_val, 0u32);
                }
            } else {
                vcall4(VT_D, obj, f, peer_val, sel_w, 0u32);
            }
        }
        if gate != 0 {
            lf_checker_rt::callee_cdecl!(3, u32, 0u32);
        }
        1
    }
});
