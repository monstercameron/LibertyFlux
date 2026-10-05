// original: 0x005235A0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Race47NoHolds, player_schema::LeaderboardInfo, 10>::vf14
// K (callee-B probe constant) for this instantiation: 0x21

/// Per-index leaderboard probe over seven slots.
///
/// `this` is a leaderboard-info object: vtable slot `+0x2c` answers one
/// probe id for the whole call, slot `+0x30` answers one id per index.
/// `a4` is an opaque context forwarded to three callees. `a0` seeds a
/// running accumulator capped at `a0+a5` (wrapping add, unsigned compare);
/// `a5` is only that cap summand. Outputs: a 64-bit single-bit mask at
/// `[a2]`, a success byte at `[a3]`, an 8-byte record copy at `[a1]`.
///
/// Algorithm: zero `[a2]`, `[a2+4]` and `[a3]`; fetch `v1` from slot
/// `+0x2c`; call callee B (first arg `K`, second a 24-byte scratch buffer
/// whose word at +8 is zeroed first) and return B's value untouched when
/// its low byte is zero. Otherwise run indices 0 through 6: fetch `v2`
/// from slot `+0x30`; call callee D(`a4`, `v2`) and skip the index unless
/// its low byte is zero; call callee E with the table word at scratch+8
/// indexed by `v2` and map its answer to a step (8 for 1, 2, 3 or 5,
/// else 0). When `v1` equals the index, call callee F(`a4`, `v2`) for a
/// record: a null record, or callee G's answer above 8 signed, clears
/// the flag, otherwise 8 bytes move from record+4 to `[a1]` and set it. On other
/// indices the accumulator advances by the step and, when still within
/// the cap, callee H sees (`a4`, `v2`, previous accumulator, step); a
/// nonzero low byte stores bit `index` into the mask at `[a2]` and sets
/// the flag. Any failing check clears the flag instead. The flag also
/// gates the loop: every iteration starts by testing it, so a cleared
/// flag ends the scan right after the current index (a skipped index
/// leaves the flag untouched and never ends the scan by itself).
///
/// Returns the flag in the low 8 bits. The upper 24 bits are whatever
/// the original last held in eax: D's answer after a skipped index, `a3`
/// after a probe-index path, the E answer decremented by the switch
/// (except -1, which skips the decrement) after an over-cap index, H's
/// answer after a zero H answer, or `a2` after a stored mask. The mask
/// split past bit 31 never triggers (indices stay below 7), and the
/// switch's fourth case is the only one that keeps step 0; the rewrite
/// states each directly.
///
/// Original: 0x005235A0 (thiscall, six stack words, callee pops 0x18).
lf_checker_rt::export!(thiscall, rw_005235A0(this: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    unsafe {
        const K: u32 = 0x21;
        const VT_SLOT_PROBE: u32 = 0x2c;
        const VT_SLOT_INDEX: u32 = 0x30;
        const N_INDEX: u32 = 7;
        const STEP: u32 = 8;
        const COPY_OK_MAX: u32 = 8;
        const SCRATCH_TABLE_WORD: usize = 2;
        const CAL_B: u32 = 2;
        const CAL_D: u32 = 4;
        const CAL_E: u32 = 5;
        const CAL_F: u32 = 6;
        const CAL_G: u32 = 7;
        const CAL_H: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        /// The original's bit-test-and-split: bit `e` of a 64-bit mask as
        /// (low, high), with the split the two conditional moves express.
        #[inline(always)]
        fn mask64(e: u32) -> (u32, u32) {
            if e < 32 {
                (1u32.wrapping_shl(e), 0)
            } else if e < 64 {
                (0, 1u32.wrapping_shl(e - 32))
            } else {
                (0, 0)
            }
        }

        wr32(a2, 0);
        wr32(a2.wrapping_add(4), 0);
        wr8(a3, 0);
        let limit = a0.wrapping_add(a5);
        let mut accum = a0;
        let vt = rd32(this);
        let probe: u32 = {
            let f: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(vt.wrapping_add(VT_SLOT_PROBE)) as usize);
            f(this)
        };
        let mut scratch = [0u32; 6];
        scratch[SCRATCH_TABLE_WORD] = 0;
        let b_ret: u32 =
            lf_checker_rt::callee_fastcall!(CAL_B, u32, K, scratch.as_mut_ptr() as u32);
        if b_ret & 0xff == 0 {
            return b_ret;
        }
        let mut flag: u8 = 1;
        let mut eax_left: u32 = b_ret;
        for index in 0..N_INDEX {
            if flag == 0 {
                break;
            }
            let slot: u32 = {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(vt.wrapping_add(VT_SLOT_INDEX)) as usize);
                f(this, index)
            };
            let d_ret: u32 = lf_checker_rt::callee_thiscall!(CAL_D, u32, a4, slot);
            eax_left = d_ret;
            if d_ret & 0xff != 0 {
                continue;
            }
            let table = scratch[SCRATCH_TABLE_WORD];
            let class_word = rd32(table.wrapping_add(slot.wrapping_mul(4)));
            let class: u32 = lf_checker_rt::callee_thiscall!(CAL_E, u32, class_word);
            eax_left = if class == 0xffff_ffff {
                class
            } else {
                class.wrapping_sub(1)
            };
            let step: u32 = match class {
                1 | 2 | 3 | 5 => STEP,
                _ => 0,
            };
            if probe == index {
                flag = 0;
                let rec: u32 = lf_checker_rt::callee_thiscall!(CAL_F, u32, a4, slot);
                if rec != 0 {
                    let size: u32 = lf_checker_rt::callee_thiscall!(CAL_G, u32, rec);
                    // Signed compare: the original jumps on greater (jg), so
                    // a negative answer still copies.
                    if (size as i32) <= (COPY_OK_MAX as i32) {
                        wr32(a1, rd32(rec.wrapping_add(4)));
                        wr32(a1.wrapping_add(4), rd32(rec.wrapping_add(8)));
                        flag = 1;
                    }
                }
                wr8(a3, flag);
                eax_left = a3;
            } else {
                let prev = accum;
                accum = accum.wrapping_add(step);
                if accum > limit {
                    flag = 0;
                } else {
                    let h_ret: u32 =
                        lf_checker_rt::callee_thiscall!(CAL_H, u32, a4, slot, prev, step);
                    if h_ret & 0xff == 0 {
                        flag = 0;
                        eax_left = h_ret;
                    } else {
                        flag = 1;
                        // @MASK-STORE-BEGIN
                        let (lo, hi) = mask64(index);
                        wr32(a2, lo);
                        wr32(a2.wrapping_add(4), hi);
                        // @MASK-STORE-END
                        eax_left = a2;
                    }
                }
            }
        }
        (eax_left & 0xffff_ff00) | (flag as u32)
    }
});
