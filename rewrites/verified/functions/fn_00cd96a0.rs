// original: 0x00cd96a0 CTaskComplexFollowLeaderAnyMeans::vf5

/// Leader-following verdict for a follow-leader task (thiscall, three stack words).
///
/// `a1` is the subject record, `a2` a mode, `a3` a counter record; `this`
/// only supplies the tail object at `+0x08`. The function returns 0 or 1 in
/// the low byte. It proceeds through a chain of probes and table reads, and
/// any failed gate diverts to a shared tail.
///
/// Chain: probe `a1`; a null `a3` or probe answer diverts. Probe the answer
/// `+8`; a null result, or one equal to `a1`, diverts. Read the `+0x20`
/// slot of `[a1+0x224]` (three times; the answer is stable within a trial):
/// a zero word at the answer's `+0x0c` diverts, and the `+0x0c` slot of that
/// word must answer `0x76c`. Then the `+0x128` slot of the second probe
/// answer selects the middle game by its low byte.
///
/// When it is zero, four check requests run against `[second+0x224]+0x2e0`
/// with codes `0x76c`, `0x772`, `0x779`, `0x11d` (each with a zero word):
/// the verdict is 1 unless the first passes and the second fails and
/// (the third fails or the fourth fails). When nonzero, bit 0 of the byte
/// at `[a1+0x224]+0x38` must be set, then two line requests run against
/// `[a1+0x224]+0x10` with (`[a1+0x20]+0x30`, 0) and
/// (`[[w+0x3c]+0x20]+0x30`, 0) where `w` is the word at the `+0x20` answer's
/// `+0x0c`: the verdict is 0 unless the first fails and (that `+0x3c` word
/// is null or the second fails). A null `[a1+0x20]` or inner `+0x20` only
/// shifts the passed address by `0x30`; nothing there is dereferenced.
///
/// With mode 2 or a zero verdict the tail runs at once. Otherwise the `+0x04`
/// slot of `a3` is read: an answer of `0x20` diverts to the tail, and a
/// reread `+0x20` slot whose `+0x08` word is zero increments `[a3+4]` and
/// returns 0. A nonzero word runs the terminal request on it and diverts.
///
/// The tail reads the mode (always `a2` on every entry): when bit 0 of the
/// byte at `[this+8]+0x0c` is set it returns 1; otherwise it runs the
/// `+0x14` slot of `[this+8]` with (`a1`, `a2`, `a3`): a zero low byte
/// returns 0, anything else sets bit 1 of that flag byte and returns 1.
///
/// Original: 0x00cd96a0 (thiscall, three stack words, returns 0/1 in `al`).
lf_checker_rt::export!(thiscall, rw_00cd96a0(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const VOBJ_OFF: u32 = 0x224;
        const POS_OFF: u32 = 0x20;
        const VSLOT_20: u32 = 0x20;
        const VSLOT_C: u32 = 0x0c;
        const VSLOT_128: u32 = 0x128;
        const VSLOT_4: u32 = 0x04;
        const VSLOT_14: u32 = 0x14;
        const V_OK: u32 = 0x76c;
        const CODE_C1: u32 = 0x76c;
        const CODE_C2: u32 = 0x772;
        const CODE_C3: u32 = 0x779;
        const CODE_C4: u32 = 0x11d;
        const Q_QUIT: u32 = 0x20;
        const MODE_DIRECT: u32 = 2;
        const CALLEE_PROBE1: u32 = 1;
        const CALLEE_PROBE2: u32 = 2;
        const CALLEE_LOS1: u32 = 3;
        const CALLEE_LOS2: u32 = 4;
        const CALLEE_C1: u32 = 5;
        const CALLEE_C2: u32 = 6;
        const CALLEE_C3: u32 = 7;
        const CALLEE_C4: u32 = 8;
        const CALLEE_TERM: u32 = 9;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        /// Call virtual slot `slot` on `obj` with no stack arguments.
        #[inline(always)]
        unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + slot) as usize);
                f(obj)
            }
        }
        /// The shared tail: flag byte at `[this+8]+0x0c`, else the `+0x14`
        /// request with (a1, a2, a3); returns 0 or 1.
        #[inline(always)]
        unsafe fn tail(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {
            unsafe {
                let tail_obj = rd32(this + 0x08);
                let flag = tail_obj + 0x0c;
                if ((flag as *const u8).read() & 1) != 0 {
                    return 1;
                }
                let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(tail_obj) + VSLOT_14) as usize);
                let z = f(tail_obj, a1, a2, a3);
                if (z & 0xff) == 0 {
                    return 0;
                }
                (flag as *mut u8).write((flag as *const u8).read() | 2);
                1
            }
        }

        let t1: u32 = lf_checker_rt::callee_thiscall!(CALLEE_PROBE1, u32, a1);
        if a3 == 0 || t1 == 0 {
            return tail(this, a1, a2, a3);
        }
        let t2: u32 = lf_checker_rt::callee_thiscall!(CALLEE_PROBE2, u32, t1.wrapping_add(8));
        if t2 == 0 || t2 == a1 {
            return tail(this, a1, a2, a3);
        }
        let vobj = rd32(a1 + VOBJ_OFF);
        let u1 = vcall0(vobj, VSLOT_20);
        if rd32(u1 + 0x0c) == 0 {
            return tail(this, a1, a2, a3);
        }
        let u2 = vcall0(vobj, VSLOT_20);
        let w = rd32(u2 + 0x0c);
        if vcall0(w, VSLOT_C) != V_OK {
            return tail(this, a1, a2, a3);
        }
        let u3 = vcall0(vobj, VSLOT_20);
        let x = rd32(u3 + 0x0c);
        let saved = rd32(x + 0x3c);
        let r = vcall0(t2, VSLOT_128);
        let verdict = if (r & 0xff) == 0 {
            // Four check requests against [t2+0x224]+0x2e0.
            let base = rd32(t2 + VOBJ_OFF).wrapping_add(0x2e0);
            let b1: u32 = lf_checker_rt::callee_thiscall!(CALLEE_C1, u32, base, CODE_C1, 0u32);
            if (b1 & 0xff) == 0 {
                1
            } else {
                let b2: u32 =
                    lf_checker_rt::callee_thiscall!(CALLEE_C2, u32, base, CODE_C2, 0u32);
                if (b2 & 0xff) != 0 {
                    1
                } else {
                    let b3: u32 =
                        lf_checker_rt::callee_thiscall!(CALLEE_C3, u32, base, CODE_C3, 0u32);
                    if (b3 & 0xff) == 0 {
                        0
                    } else {
                        let b4: u32 = lf_checker_rt::callee_thiscall!(
                            CALLEE_C4, u32, base, CODE_C4, 0u32
                        );
                        if (b4 & 0xff) != 0 {
                            1
                        } else {
                            0
                        }
                    }
                }
            }
        } else {
            if ((vobj + 0x38) as *const u8).read() & 1 == 0 {
                0
            } else {
                let ecx2 = vobj.wrapping_add(0x10);
                let pos1 = rd32(a1 + POS_OFF).wrapping_add(0x30);
                let l1: u32 =
                    lf_checker_rt::callee_thiscall!(CALLEE_LOS1, u32, ecx2, pos1, 0u32);
                if (l1 & 0xff) != 0 {
                    0
                } else if saved == 0 {
                    1
                } else {
                    let pos2 = rd32(saved + POS_OFF).wrapping_add(0x30);
                    let l2: u32 =
                        lf_checker_rt::callee_thiscall!(CALLEE_LOS2, u32, ecx2, pos2, 0u32);
                    if (l2 & 0xff) == 0 {
                        1
                    } else {
                        0
                    }
                }
            }
        };
        if a2 == MODE_DIRECT || verdict == 0 {
            return tail(this, a1, a2, a3);
        }
        let q = vcall0(a3, VSLOT_4);
        if q == Q_QUIT {
            return tail(this, a1, a2, a3);
        }
        let u4 = vcall0(vobj, VSLOT_20);
        if rd32(u4 + 0x08) == 0 {
            let p = (a3 + 4) as *mut u32;
            p.write_unaligned(p.read_unaligned().wrapping_add(1));
            return 0;
        }
        let u5 = vcall0(vobj, VSLOT_20);
        let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_TERM, u32, u5);
        tail(this, a1, a2, a3)
    }
});
