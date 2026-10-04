// original: 0x00b07760 CCamGame::vf4

/// Camera event/gate step (thiscall, `this` in ECX, no stack arguments,
/// returns 1 in AL). Two identical gate blocks, a middle update chain and a
/// tail clear. Each gate block asks a provider callee for a status object and
/// derives a one-bit verdict from xor-folded byte pairs: when the selector
/// byte at +0x328D is nonzero the verdict is set only if the first xor exceeds
/// 0x7F while the second does not; when it is zero the verdict needs the first
/// xor at or below 0x7F, the second above it, and a global inhibit flag clear.
/// A set verdict plus a third xor above 0x7F asks a watcher callee for a
/// target and, when non-null, notifies a sink callee with (this+0x10, a global
/// word, 1, 1); the two blocks differ only in which global word is passed.
/// The middle chain runs one update callee always and seven more only while a
/// global singleton exists (one of them is the neighbouring 0xB07A40 entry;
/// all intercepted). The tail asks the watcher once more with a different
/// selector, records whether the returned object's state byte equals 3,
/// clears its 0x20 bit and zeroes the state byte, then clears a global done
/// flag. No floating point.
lf_checker_rt::export!(thiscall, rw_00b07760(this: u32) -> u32 {
    unsafe {
        const GATE_SEL: u32 = 0x328D;
        const GATE_B0: u32 = 0x269C;
        const GATE_B1: u32 = 0x269E;
        const GATE_B2: u32 = 0x269F;
        const GATE_C0: u32 = 0x29BC;
        const GATE_C1: u32 = 0x29BE;
        const G_INHIB: u32 = 0x011F701F;
        const G_WORD1: u32 = 0x0104008C;
        const G_WORD2: u32 = 0x01040090;
        const G_MATCHED: u32 = 0x016154A3;
        const G_DONE: u32 = 0x016154B0;
        const TAIL_STATE: u32 = 0x22E;
        const TAIL_FLAGS: u32 = 0x18C;
        const TAIL_BUSY: u8 = 0x20;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }

        /// Pure byte verdict of one gate block. `obj` is known non-null.
        unsafe fn verdict(obj: u32, inhib: u8) -> u8 {
            unsafe {
                if rd8(obj + GATE_SEL) != 0 {
                    let base = rd8(obj + GATE_B0);
                    if rd8(obj + GATE_B1) ^ base > 0x7F {
                        if rd8(obj + GATE_B2) ^ base <= 0x7F {
                            return 1;
                        }
                    }
                    0
                } else {
                    let base = rd8(obj + GATE_B0);
                    if rd8(obj + GATE_B1) ^ base > 0x7F {
                        return 0;
                    }
                    if rd8(obj + GATE_B2) ^ base <= 0x7F {
                        return 0;
                    }
                    if inhib != 0 {
                        return 0;
                    }
                    1
                }
            }
        }

        let inhib = rd8(lf_checker_rt::relocated(G_INHIB));
        // Block 1.
        let o1: u32 = lf_checker_rt::callee_cdecl!(1, u32, 0, 0);
        if o1 != 0 && verdict(o1, inhib) != 0 {
            if rd8(o1 + GATE_C1) ^ rd8(o1 + GATE_C0) > 0x7F {
                let t: u32 = lf_checker_rt::callee_thiscall!(2, u32, this, 0x16, 0);
                if t != 0 {
                    let w = rd16(lf_checker_rt::relocated(G_WORD1)) as u32;
                    lf_checker_rt::callee_thiscall!(3, u32, this, this + 0x10, w, 1, 1);
                }
            }
        }
        // Middle chain.
        lf_checker_rt::callee_thiscall!(4, u32, this);
        let sing: u32 = lf_checker_rt::callee_cdecl!(5, u32,);
        if sing != 0 {
            lf_checker_rt::callee_thiscall!(6, u32, this);
            lf_checker_rt::callee_thiscall!(7, u32, this);
            lf_checker_rt::callee_thiscall!(8, u32, this);
            lf_checker_rt::callee_thiscall!(9, u32, this);
            lf_checker_rt::callee_thiscall!(10, u32, this);
            lf_checker_rt::callee_thiscall!(11, u32, this);
            lf_checker_rt::callee_thiscall!(12, u32, this);
        }
        // Block 2 (same shape, other word).
        let o2: u32 = lf_checker_rt::callee_cdecl!(13, u32, 0, 0);
        if o2 != 0 && verdict(o2, inhib) != 0 {
            if rd8(o2 + GATE_C1) ^ rd8(o2 + GATE_C0) > 0x7F {
                let t: u32 = lf_checker_rt::callee_thiscall!(14, u32, this, 0x16, 0);
                if t != 0 {
                    let w = rd16(lf_checker_rt::relocated(G_WORD2)) as u32;
                    lf_checker_rt::callee_thiscall!(15, u32, this, this + 0x10, w, 1, 1);
                }
            }
        }
        // Tail.
        let tail: u32 = lf_checker_rt::callee_thiscall!(16, u32, this, 2, 0);
        if tail != 0 {
            let matched = (rd8(tail + TAIL_STATE) == 3) as u8;
            wr8(lf_checker_rt::relocated(G_MATCHED), matched);
            let fl = rd8(tail + TAIL_FLAGS);
            if fl & TAIL_BUSY != 0 {
                wr8(tail + TAIL_FLAGS, fl & !TAIL_BUSY);
            }
            wr8(tail + TAIL_STATE, 0);
        }
        wr8(lf_checker_rt::relocated(G_DONE), 0);
        1
    }
});
