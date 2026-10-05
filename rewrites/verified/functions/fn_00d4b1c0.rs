// original: 0x00D4B1C0 run-named-anim-start-helper (proposed)

#![allow(unsafe_code)]

#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}
#[inline(always)]
unsafe fn wr32(a: u32, v: u32) {
    unsafe { (a as *mut u32).write_unaligned(v) }
}
#[inline(always)]
unsafe fn wr16(a: u32, v: u16) {
    unsafe { (a as *mut u16).write_unaligned(v) }
}
#[inline(always)]
unsafe fn wr8(a: u32, v: u8) {
    unsafe { (a as *mut u8).write(v) }
}

/// Start (or restart) the named animation on the ped (`vf17` helper).
///
/// `this` is the run-anim task, `ped` the ped. When the loop counter at
/// `+0x9c` is non-negative the current timer value is stored at `+0xa0`
/// with the counter at `+0xa4` and a started flag at `+0xa8`, and flag
/// `0x4000` is set at `+0xb0` unless `0x8000` is already there. A linked
/// object at `+0x4`, when present, is asked for its task id through its
/// virtual slot `+0xc`; id `0x111` triggers a scan of the task table for
/// the row matching `+0x14`, whose nonzero prefix length decides a flag
/// used below. The animation is then looked up from the ped's anim state
/// (`+0x78`) with the group/name at `+0x80`/`+0x60`, the flags at
/// `+0xb0`/`+0xac` and the blend rate at `+0x98`; a null answer sets the
/// done bit at `+0x18` and returns zero, otherwise the clip time at
/// `+0xb4` is applied. Finally the state bits at `+0x18`/`+0x19` pick one
/// of two flag updates on the animation result (set `0x8000` and drop
/// `0x4000`, or the reverse), each preceded by the matching notifier call
/// with its mode tag. Returns the notifier result tag described above.
///
/// Original: 0x00D4B1C0 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00D4B1C0(this: u32, ped: u32) -> u32 {
    unsafe {
        const TIMER: u32 = 0x011735B4;
        const TABLE: u32 = 0x0167F780;
        const TAG_A: u32 = 0x00D49D40;
        const TAG_B: u32 = 0x00D49D30;
        const TAG_F: u32 = 0x00D49CF0;
        const VT_CALL: u32 = 0;
        const ANIM_LOOKUP: u32 = 1;
        const SET_CLIP: u32 = 2;
        const NOTIFY: u32 = 3;

        #[inline(always)]
        unsafe fn or32(a: u32, v: u32) {
            unsafe { wr32(a, rd32(a) | v) }
        }
        // Tail 1: notify with mode 1, set 0x8000, drop 0x4000 if present.
        // Returns the updated word shifted down by 14.
        #[inline(always)]
        unsafe fn tail_set(this: u32, t: u32, tag: u32) -> u32 {
            unsafe {
                let _: u32 = lf_checker_rt::callee_thiscall!(NOTIFY, u32, t, 1, lf_checker_rt::relocated(tag), this);
                let new = rd32(t + 4) | 0x8000;
                wr32(t + 4, new);
                if new & 0x4000 != 0 {
                    wr32(t + 4, new & !0x4000);
                }
                new >> 14
            }
        }
        // Tail 2: notify with mode 2, drop 0x8000 if present, set 0x4000.
        // Returns the animation handle.
        #[inline(always)]
        unsafe fn tail_clear(this: u32, t: u32) -> u32 {
            unsafe {
                let _: u32 = lf_checker_rt::callee_thiscall!(NOTIFY, u32, t, 2, lf_checker_rt::relocated(TAG_B), this);
                let v = rd32(t + 4);
                if v & 0x8000 != 0 {
                    wr32(t + 4, v & !0x8000);
                }
                or32(t + 4, 0x4000);
                t
            }
        }

        let counter = rd32(this + 0x9c);
        if (counter as i32) >= 0 {
            let now: u32 = (lf_checker_rt::global::<u32>(TIMER) as *const u32).read_unaligned();
            wr32(this + 0xa0, now);
            wr32(this + 0xa4, counter);
            wr8(this + 0xa8, 1);
        }
        let b0 = rd32(this + 0xb0);
        if b0 & 0x8000 == 0 {
            wr32(this + 0xb0, b0 | 0x4000);
        }
        let mut full_row = false;
        let linked = rd32(this + 4);
        if linked != 0 {
            let slot = rd32(rd32(linked) + 0x0c);
            let get_id: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
            if get_id(linked) == 0x111 {
                let linked2 = rd32(this + 4);
                if linked2 != 0 {
                    let idx = rd32(linked2 + 0x14);
                    let base = lf_checker_rt::relocated(TABLE)
                        .wrapping_add(0x18)
                        .wrapping_add(idx.wrapping_mul(0x6c));
                    let mut count = 0u32;
                    let mut p = base;
                    if rd32(p) != 0 {
                        loop {
                            p = p.wrapping_add(4);
                            count += 1;
                            if rd32(p) == 0 {
                                break;
                            }
                        }
                    }
                    full_row = rd32(linked2 + 0x18).wrapping_add(1) == count;
                }
            }
        }
        let anim_state = rd32(ped + 0x78);
        let anim: u32 = lf_checker_rt::callee_thiscall!(ANIM_LOOKUP, u32, anim_state, this + 0x80,
            this + 0x60, rd32(this + 0xb0), rd32(this + 0xac), rd32(this + 0x98));
        wr32(this + 0x14, anim);
        if anim == 0 {
            wr8(this + 0x18, rd8(this + 0x18) | 1);
            return 0;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(SET_CLIP, u32, anim, rd32(this + 0xb4));
        let flags = rd8(this + 0x18);
        if flags & 0x80 != 0 {
            return tail_set(this, anim, TAG_A);
        }
        if flags & 0x40 == 0 || full_row {
            if flags & 0x04 == 0 {
                if rd32(this + 0xb0) & 0x8000 == 0 {
                    return tail_clear(this, anim);
                } else {
                    return tail_set(this, anim, TAG_B);
                }
            }
            return tail_clear(this, anim);
        }
        if flags & 0x04 == 0 {
            if rd32(this + 0xb0) & 0x8000 == 0 {
                return tail_clear(this, anim);
            } else {
                return tail_set(this, anim, TAG_B);
            }
        }
        if rd8(this + 0x19) & 4 != 0 {
            return tail_clear(this, anim);
        }
        if (rd32(this + 0x9c) as i32) < 0 {
            return tail_set(this, anim, TAG_F);
        }
        tail_clear(this, anim)
    }
});
