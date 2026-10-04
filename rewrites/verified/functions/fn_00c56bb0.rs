// original: 0x00c56bb0 task_pick_weighted_slot (proposed)

/// Pick one of sixteen slots by bound, weight and a random roll.
///
/// Each slot holds an index, an addend, a gate word and a gate byte. A slot
/// is a candidate when its gate word is nonzero, its gate byte is set or the
/// low byte of `arg` is 0, and the current timer is below its bound (the word
/// at `this+idx*2+0x120` plus its addend). The winning slot is the last one
/// taken: a candidate whose weight byte (at `this+idx+0x100`, compared
/// unsigned) exceeds the best so far (compared signed, so once the best
/// reaches 0x80 every later candidate wins) is taken at once, otherwise a
/// random roll (callee 1) below 0x80 takes it. When the low byte of `arg` is
/// nonzero the current slot at `+0x210` is validated first (returning it at
/// once when still a candidate) and updated at the end. Returns the winning
/// entry address, or 0 when nothing was taken.
///
/// Original: 0x00c56bb0 (thiscall: `this` in ecx, one stack word).
lf_checker_rt::export!(thiscall, rw_00c56bb0(this: u32, arg: u32) -> u32 {
    unsafe {
        const TIMER_SLOT: u32 = 0x11735b4;
        const FLAGS: u32 = 0x1dc;
        const CURRENT: u32 = 0x210;
        const WEIGHTS: u32 = 0x100;
        const BOUNDS: u32 = 0x120;
        const RAND: u32 = 1;
        const KEEP_ROLL: u32 = 0x80;
        const NONE: u8 = 0xFF;
        #[inline(always)]
        unsafe fn bound(this: u32, entry: u32) -> u32 {
            unsafe {
                let ix = (entry as *const u32).read_unaligned();
                let w = ((this.wrapping_add(ix.wrapping_mul(2)).wrapping_add(BOUNDS))
                    as *const u16)
                    .read_unaligned() as u32;
                w.wrapping_add(((entry + 4) as *const u32).read_unaligned())
            }
        }
        #[inline(always)]
        unsafe fn weight(this: u32, entry: u32) -> u8 {
            unsafe {
                let ix = (entry as *const u32).read_unaligned();
                ((ix.wrapping_add(this).wrapping_add(WEIGHTS)) as *const u8).read()
            }
        }
        let timer: u32 = lf_checker_rt::global::<u32>(TIMER_SLOT).read();
        let flag = (arg & 0xff) != 0;
        if flag && ((this + FLAGS) as *const u8).read() & 4 != 0 {
            let cur = ((this + CURRENT) as *const u8).read();
            if cur != NONE {
                let e = this.wrapping_add((cur as i8 as i32 as u32).wrapping_mul(0x10));
                if ((e + 8) as *const u32).read_unaligned() != 0 && timer < bound(this, e) {
                    return e;
                }
            }
        }
        let mut best: u8 = 0;
        let mut slot: u8 = NONE;
        for k in 0..16u32 {
            let s = this.wrapping_add(k.wrapping_mul(0x10));
            if ((s + 8) as *const u32).read_unaligned() == 0 {
                continue;
            }
            if ((s + 0x0c) as *const u8).read() == 0 && flag {
                continue;
            }
            if timer >= bound(this, s) {
                continue;
            }
            let w = weight(this, s);
            let take = if (w as i32) > (best as i8 as i32) {
                true
            } else {
                let r: u32 = lf_checker_rt::callee_cdecl!(RAND, u32,);
                (r & 0xffff) < KEEP_ROLL
            };
            if take {
                slot = k as u8;
                best = w;
            }
        }
        if flag {
            ((this + CURRENT) as *mut u8).write(slot);
        }
        if slot == NONE {
            0
        } else {
            this.wrapping_add((slot as u32).wrapping_mul(0x10))
        }
    }
});

