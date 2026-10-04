// original: 0x00dde6b0 UITextField::vf110 (symbols)

/// Poll the shared UI manager and either forward to the field's follow-up
/// handler or return the leftover call answer.
///
/// `this` is the field object; the handler reads one child pointer from it
/// (`+0x1f0`). Behaviour: ask the UI manager (global object) for its status
/// triplet with arguments (0xc02, 1) and dispatch on the first word: 6 polls
/// the manager again with 1, 4 stores 1 through the set callee on this
/// field, 3 stores -1 instead, anything else does nothing. When the child at
/// `+0x1f0` is non-null, call its virtual slot `+0x124` with no arguments;
/// a nonzero low byte tail-jumps to the follow-up handler (the function at
/// 0x00dde730, scripted by the contract) with this field, and its answer is
/// the return value. Otherwise the function falls through and returns
/// whatever the last executed step left in the accumulator: the gate
/// answer when the gate ran, else the last set/poll answer, else the first
/// status word again (the original reloads it into the accumulator right
/// after copying the triplet through its frame with a vector move; the
/// other two words are read but never used).
///
/// Edge cases: a null child skips the gate and the tail jump; a status word
/// outside {3, 4, 6} skips the middle call; only the gate answer's low byte
/// decides the jump, the full word is returned on fall-through.
///
/// Original: 0x00dde6b0 (thiscall, no stack arguments; ends in a conditional
/// tail jump. True extent is 126 bytes: the bytes after the first return
/// belong to the following functions, not this one.)
lf_checker_rt::export!(thiscall, rw_00dde6b0(this: u32) -> u32 {
    unsafe {
        const UI_MANAGER: u32 = 0x019d2e08;
        const PROBE_SELECTOR: u32 = 0xc02;
        const CHILD: u32 = 0x1f0;
        const VT_GATE: u32 = 0x124;
        const CALLEE_PROBE: u32 = 1;
        const CALLEE_POLL: u32 = 2;
        const CALLEE_SET: u32 = 3;
        const CALLEE_TAIL: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mgr = lf_checker_rt::relocated(UI_MANAGER);
        let r: u32 = lf_checker_rt::callee_thiscall!(CALLEE_PROBE, u32, mgr, PROBE_SELECTOR, 1);
        let v = rd32(r);
        // The original also loads the other two words (kept for fault parity)
        // but overwrites the accumulator with the first word right after.
        let _middle = rd32(r.wrapping_add(4));
        let _last = rd32(r.wrapping_add(8));
        let mut residue = v;
        if v == 6 {
            residue = lf_checker_rt::callee_thiscall!(CALLEE_POLL, u32, mgr, 1);
        } else if v == 4 {
            residue = lf_checker_rt::callee_thiscall!(CALLEE_SET, u32, this, 1);
        } else if v == 3 {
            residue = lf_checker_rt::callee_thiscall!(CALLEE_SET, u32, this, 0xFFFF_FFFF);
        }
        let obj = rd32(this + CHILD);
        if obj != 0 {
            let vt = rd32(obj);
            let gate: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(vt + VT_GATE) as usize);
            let g: u32 = gate(obj);
            if (g as u8) != 0 {
                let t: u32 = lf_checker_rt::callee_thiscall!(CALLEE_TAIL, u32, this);
                return t;
            }
            return g;
        }
        residue
    }
});
