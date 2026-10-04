// original: 0x00aec7b0 timing_step_second (proposed)

/// Second step dispatcher: refresh the row selector, then run one action.
///
/// Same shape as the first step dispatcher (`0x00aec5c0`) without the fence
/// calls: thiscall with five stack words (`obj`, `f`, `a`, `b`, `sel`);
/// `this` is a controller with a mode word at `+4`, a peer at `+8` and flags
/// at `+0xc`. The refresh callee takes the incoming `sel` with three
/// out-pointers and rewrites `sel` in place; a clear row at `obj+0x40[sel]`
/// returns 0. The mode object and the two condition bytes are derived the
/// same way (`c1`: ready and flags not aligned; the second byte kept in the
/// accumulator instead of spilled). The action tree is identical: `a`
/// nonzero runs through the mode object when its link is set, else virtual
/// slot `+0x30`/`+0x28` on `obj` by the condition bytes; `a` zero with `b`
/// nonzero repeats the shape with the float bits leading; both zero selects
/// slot `+0x2c` when `c1` is set without the second byte, else slot `+0x20`;
/// a set row is re-checked on every path and skips silently. Returns `al` 1.
///
/// The original spills the first condition byte and the updated selector
/// over its incoming argument slots; the rewrite uses locals (every value
/// stays observed through the outgoing calls). The meaningful return is the
/// low byte; the upper bytes are stub residue.
lf_checker_rt::export!(thiscall, rw_00aec7b0(this: u32, obj: u32, f: u32, a: u32, b: u32, sel: u32) -> u32 {
    unsafe {
        const ROWS: u32 = 0x40;
        const THIS_MODE: u32 = 0x04;
        const THIS_PEER: u32 = 0x08;
        const THIS_FLAGS: u32 = 0x0c;
        const MODEW: u32 = 0x28;
        const MODE_THIS: u32 = 0x34;
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
        let al = c1 && (flags.wrapping_shr(1) & 1) != 0;
        let peer_val = rd32(this + THIS_PEER);
        if a != 0 {
            if row() != 0 {
                let link = if picked != 0 { rd32(picked + MODE_THIS) } else { 0 };
                if link != 0 {
                    lf_checker_rt::callee_thiscall!(
                        4, u32, link, 0u32, a, peer_val, sel_w, flags,
                    );
                } else if c1 {
                    vcall5(VT_A, obj, 0u32, a, sel_w, if al { 1u32 } else { 2u32 }, peer_val);
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
                    vcall5(VT_A, obj, f, b, sel_w, if al { 1u32 } else { 2u32 }, peer_val);
                } else {
                    vcall4(VT_B, obj, f, b, peer_val, sel_w);
                }
            }
        } else if row() != 0 {
            if c1 {
                if !al {
                    vcall4(VT_C, obj, f, sel_w, peer_val, 0u32);
                }
            } else {
                vcall4(VT_D, obj, f, peer_val, sel_w, 0u32);
            }
        }
        1
    }
});
