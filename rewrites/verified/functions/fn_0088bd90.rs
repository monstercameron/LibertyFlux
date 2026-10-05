// original: 0x0088bd90 audVoice_control_88BD90 (proposed)
/// Lock one voice's channel, drain its pending sample windows, unlock.
///
/// `this` points to the voice object. A non-zero low byte of `arg0` resets
/// the toggle at `+0xd8` and selects a budget of `0x20000` (start flag `2`);
/// a zero byte bumps the counter at `+0xcc` and selects `0x10000` (flag
/// `0`). The channel object at `+0x90` is locked through its slot at
/// `+0x2c` with eight stack arguments (object, encoded toggle, budget, two
/// out-pointers into this frame, two zeroes, start flag); a negative answer
/// fails the channel through slot `+0x8` and returns that answer.
///
/// Otherwise the cursor at `+0xe4` selects a 64-byte slot whose words at
/// `+0x124` (position), `+0x128` (remaining) and `+0xec` (base) are drained
/// in up to two passes against the budget: each pass hands at most the
/// remaining count to a sink helper (three arguments), advances the
/// position, shrinks the remainder, and rotates the cursor to
/// `(cursor + 1) % divisor` (divisor at `+0xe8`) when a remainder hits zero.
/// Whatever the budget still lacks is handed to a fill helper (three
/// arguments), which also latches the word at `+0xdc` and the byte at
/// `+0x9d` to `1` when the byte was clear. The channel is unlocked through
/// slot `+0x4c` with the two locked words (five arguments). With a non-zero
/// argument byte the unlock answer is returned, else the toggle becomes
/// `(toggle - 1) & 1` and is returned.
///
/// Original: 0x0088bd90 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_0088bd90(this: u32, arg0: u32) -> u32 {
    unsafe {
        const TOGGLE: u32 = 0xd8;
        const START_COUNT: u32 = 0xcc;
        const CHANNEL_OBJ: u32 = 0x90;
        const LOCK_SLOT: u32 = 0x2c;
        const FAIL_SLOT: u32 = 0x08;
        const UNLOCK_SLOT: u32 = 0x4c;
        const CURSOR: u32 = 0xe4;
        const DIVISOR: u32 = 0xe8;
        const SLOT_SHIFT: u32 = 6;
        const SLOT_POS: u32 = 0x124;
        const SLOT_LEFT: u32 = 0x128;
        const SLOT_BASE: u32 = 0xec;
        const LATCH_WORD: u32 = 0xdc;
        const LATCH_BYTE: u32 = 0x9d;
        const BUDGET_HI: u32 = 0x20000;
        const BUDGET_LO: u32 = 0x10000;
        const CAL_SINK: u32 = 3;
        const CAL_FILL: u32 = 4;
        // Lock/unlock/fail are callee ids 1, 5 and 2 in the contract,
        // reached through the planted channel vtable like the original.

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let fresh = (arg0 as u8) != 0;
        let (budget, start) = if fresh {
            wr32(this.wrapping_add(TOGGLE), 0);
            (BUDGET_HI, 2u32)
        } else {
            let c = this.wrapping_add(START_COUNT);
            wr32(c, rd32(c).wrapping_add(1));
            (BUDGET_LO, 0u32)
        };
        let obj = rd32(this.wrapping_add(CHANNEL_OBJ));
        let vt = rd32(obj);
        let d8 = rd32(this.wrapping_add(TOGGLE));
        let enc = d8.wrapping_shl(17).wrapping_shr(1);
        let mut out_lock = 0u32;
        let mut out_pos = 0u32;
        let lock: extern "stdcall" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(vt.wrapping_add(LOCK_SLOT)) as usize);
        let r = lock(
            obj,
            enc,
            budget,
            &mut out_pos as *mut u32 as u32,
            &mut out_lock as *mut u32 as u32,
            0,
            0,
            start,
        );
        if (r as i32) < 0 {
            let fail: extern "stdcall" fn(u32) -> u32 =
                core::mem::transmute(rd32(vt.wrapping_add(FAIL_SLOT)) as usize);
            return fail(obj);
        }
        let mut take = {
            let e4 = rd32(this.wrapping_add(CURSOR));
            let slot = this.wrapping_add(e4.wrapping_shl(SLOT_SHIFT));
            let left = rd32(slot.wrapping_add(SLOT_LEFT));
            let t = if budget < left { budget } else { left };
            if t != 0 {
                let p = rd32(slot.wrapping_add(SLOT_POS)).wrapping_add(rd32(slot.wrapping_add(SLOT_BASE)));
                lf_checker_rt::callee_cdecl!(CAL_SINK, u32, out_pos, p, t);
                wr32(slot.wrapping_add(SLOT_POS), rd32(slot.wrapping_add(SLOT_POS)).wrapping_add(t));
                wr32(slot.wrapping_add(SLOT_LEFT), left.wrapping_sub(t));
                if rd32(slot.wrapping_add(SLOT_LEFT)) == 0 {
                    let div = rd32(this.wrapping_add(DIVISOR));
                    wr32(this.wrapping_add(CURSOR), e4.wrapping_add(1) % div);
                }
            }
            t
        };
        if take < budget {
            let e4 = rd32(this.wrapping_add(CURSOR));
            let slot = this.wrapping_add(e4.wrapping_shl(SLOT_SHIFT));
            let left = rd32(slot.wrapping_add(SLOT_LEFT));
            let rem = budget.wrapping_sub(take);
            let rest = if rem < left { rem } else { left };
            if rest != 0 {
                let p = rd32(slot.wrapping_add(SLOT_POS)).wrapping_add(rd32(slot.wrapping_add(SLOT_BASE)));
                lf_checker_rt::callee_cdecl!(CAL_SINK, u32, out_pos.wrapping_add(take), p, rest);
                wr32(slot.wrapping_add(SLOT_POS), rd32(slot.wrapping_add(SLOT_POS)).wrapping_add(rest));
                wr32(slot.wrapping_add(SLOT_LEFT), left.wrapping_sub(rest));
                if rd32(slot.wrapping_add(SLOT_LEFT)) == 0 {
                    let div = rd32(this.wrapping_add(DIVISOR));
                    wr32(this.wrapping_add(CURSOR), e4.wrapping_add(1) % div);
                }
            }
            take = take.wrapping_add(rest);
            if take < budget {
                lf_checker_rt::callee_cdecl!(
                    CAL_FILL,
                    u32,
                    out_pos.wrapping_add(take),
                    0u32,
                    budget.wrapping_sub(take)
                );
                let lb = this.wrapping_add(LATCH_BYTE);
                if (lb as *const u8).read() == 0 {
                    wr32(this.wrapping_add(LATCH_WORD), 1);
                    (lb as *mut u8).write(1);
                }
            }
        }
        let unlock: extern "stdcall" fn(u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(vt.wrapping_add(UNLOCK_SLOT)) as usize);
        let u = unlock(obj, out_pos, out_lock, 0, 0);
        if fresh {
            u
        } else {
            let t = this.wrapping_add(TOGGLE);
            let v = rd32(t).wrapping_sub(1) & 1;
            wr32(t, v);
            v
        }
    }
});
