// original: 0x00cc6020 euphoria_behaviour_update
//! Per-tick update over one behaviour record.
//!
//! Calling convention: thiscall with the owner object in ECX and five stack
//! words: a frame pointer, an output slot pointer, a weight pointer, a level
//! pointer and a float bias. Returns the tick's blend factor as an f32.
//!
//! The record is reached through the frame, the channel resolver beside it.
//! The update first polls the record's own update entry: a null answer skips
//! the gain stage and carries the incoming bias, otherwise the answered gain
//! and the answered object's stored gain are gated against zero and 0.85 and
//! a refusal parks the bias at zero.
//!
//! The level dispatch then picks the timed path or the fallback chains. The
//! timed path needs a zero bias, a clear hold flag, parked state, and a probe
//! plus tick-limit check; two quality switches choose thresholds that route
//! to the fast, middle or slow binding mode. Each mode resolves its four live
//! binding ids through the channel, keeps the newest enabled gain found as
//! the accumulator, parks every visited slot, folds the accumulator into its
//! band (folded bands pass through, over-range clears, the middle band pays
//! half), matches the owner tags, touches the channel and passes the blend
//! gate, then locks the blend factor and reports -1. Any refusal on the way
//! parks the match slots, locks the blend factor and reports -2.
//!
//! The fallback path runs a six-id chain when the level is exactly zero and
//! retires the slot otherwise. A found object passes an optional gated pair
//! measurement and an adjust entry that reports -1 under its cap. The retire
//! path sweeps a ten-id chain: a found object stages its gain (negative or
//! exactly-zero gains drop out), optionally sums two blend terms against a
//! ceiling (a shortfall reports exactly 1), then commits class stamps, sweeps
//! the owner table parking every id still present, and reports 3 for a high
//! level flag or 2 with the stored level written through. No object, a failed
//! tail gate, or a level at or under one with its flags set, answers from the
//! piecewise level tail: the level plus one inside the open bands around 2,
//! 3 and 4 at the exact joints, 2 through the gates, 1 for a missing staged
//! object or a negative final gain, else 0.
//!
//! Floating point is bit-exact throughout, including NaN handling: the
//! unordered outcomes of the original's comparisons take the same arms here
//! (below-or-unordered rather than plain below), and every arithmetic
//! operation pins its operand order.

use lf_aq27_rt::{callee_cdecl, callee_thiscall, export, relocated};

// ---------------------------------------------------------------------------
// Callee ids. These must match the contract.
// ---------------------------------------------------------------------------
const C_GAIN: u32 = 2; // thiscall/0, answers f32: per-object gain
const C_GATE2: u32 = 3; // thiscall/2, answers bool: state gate
const C_PROBE: u32 = 4; // thiscall/1, answers nonzero or zero
const C_QUALITY: u32 = 5; // thiscall/0, answers bool: quality switches
const C_FIND_LOOP: u32 = 6; // thiscall/1, id lookup over the binding loop
const C_MATCH2: u32 = 7; // first tag-match site
const C_MATCH2B: u32 = 22; // second tag-match site (own script) // cdecl/2, answers bool: tag match
const C_TOUCH: u32 = 8; // thiscall/1, answer discarded
const C_BLEND5: u32 = 9; // cdecl/5, answers bool: blend gate
const C_SEED: u32 = 10; // thiscall/1, float seed, answer unused
const C_FIND_FIRST: u32 = 11; // thiscall/1, first fallback id chain
const C_SUM_A: u32 = 16; // thiscall/0, answers f32: first blend term
const C_SUM_B: u32 = 17; // thiscall/0, answers f32: second blend term
const C_SUM_C: u32 = 18; // thiscall/0, answers f32: blend ceiling
const C_COMMIT: u32 = 19; // thiscall/1, commit, answer unused
const C_FIND_LAST: u32 = 20; // thiscall/1, final id lookup
const C_GATE2B: u32 = 21; // thiscall/2, answers bool: tail gate
const C_DECAY: u32 = 13; // thiscall/1, float decay, answer unused
const C_PAIR: u32 = 14; // cdecl/2 of two floats, answers f32
const C_FIND_SECOND: u32 = 15; // thiscall/1, second fallback id chain

// ---------------------------------------------------------------------------
// Game data: file VAs of the float constants and tunables the original loads.
// ---------------------------------------------------------------------------
const F_GATE_085: u32 = 0xFE88B0;
const F_ONE: u32 = 0xFE88E8;
const F_NEG_TWO: u32 = 0xFE8DB0;
const F_ONE_HALF: u32 = 0xFE8960;
const F_TWO: u32 = 0xFE8A24;
const F_LO_FALLBACK: u32 = 0xE833D0;
const F_THREE: u32 = 0xFE8A94;
const F_QUARTER: u32 = 0xFE87E4;
const F_THREE_Q: u32 = 0xFE888C;
const F_HALF: u32 = 0xFE8830;
const F_NEG_ONE: u32 = 0xFE8D94;
const F_ZERO: u32 = 0xFE8628;
const F_ARC: u32 = 0xE9B9CC;
const F_FOUR: u32 = 0xFE8AB8;
const ABS_MASK_W: u32 = 0xFE8F80;
const G_BLEND_CAP: u32 = 0x1051520;
const G_LEVEL_STORE: u32 = 0x105149C;
const G_TICK: u32 = 0x11735B4;
const G_TICK_LIMIT: u32 = 0x171C0F4;
const G_GATE_BYTE: u32 = 0x1051525;

// ---------------------------------------------------------------------------
// Record layout: offsets reached inside the behaviour record and the owner.
// ---------------------------------------------------------------------------
const CTX_RECORD: u32 = 0xA80;
const CTX_CHANNEL: u32 = 0x78;
const CTX_MODE_FLAG: u32 = 0x29C;
const CTX_PAIR_A: u32 = 0xAA0;
const CTX_PAIR_B: u32 = 0xAA4;
const REC_VTABLE: u32 = 0x0;
const REC_STATE: u32 = 0x48;
const REC_FLAGS: u32 = 0x50;
const REC_BLEND: u32 = 0x8;
const REC_RATE: u32 = 0x54;
const REC_TICK: u32 = 0x74;
const REC_LEVEL: u32 = 0x20;
const REC_MATCH0: u32 = 0xAC;
const REC_MATCH1: u32 = 0xB0;
const REC_MATCH2: u32 = 0xB4;
const REC_MATCH3: u32 = 0xB8;
const REC_CLASS_A: u32 = 0x1C;
const REC_CLASS_B: u32 = 0x28;
const REC_CLASS_C: u32 = 0x2C;
const REC_LEVEL_COPY: u32 = 0x78;
const OWN_TAG: u32 = 0x0;
const OWN_TAG2: u32 = 0x4;
const OWN_FLAGS: u32 = 0x378;
const OWN_TABLE: u32 = 0xB0;
const OWN_STRIDE: u32 = 0x18;
const OWN_ROWS: u32 = 7;
const VT_UPDATE: u32 = 0x54;
const VT_ADJUST: u32 = 0x8;
const OBJ_VTABLE: u32 = 0x0;
const OBJ_FLAGS: u32 = 0x4;
const OBJ_CODE: u32 = 0xC;
const OBJ_MARK: u32 = 0x46;
const OBJ_GAIN: u32 = 0x4C;

// Immediate bit patterns the original materialises in registers.
const BITS_NEG_ONE_F: u32 = 0xBF800000;
const BITS_HARD_DECAY: u32 = 0xC3FA0000;
const BITS_BLEND_ARG: u32 = 0x40800000;
const FRESH_BLEND: u32 = 0;
// Blend factor stored on the reset path and the committed path.
const BLEND_LOCKED: u32 = 0x40400000;
const LEVEL_HIGH_BITS: u32 = 0x40000000;
const LEVEL_LOW_BITS: u32 = 0x3F800000;

const MODE_FAST: u32 = 0x27;
const MODE_FAST_ADJ: u32 = 0x26;
const MODE_MID: u32 = 0x31;
const MODE_MID_ADJ: u32 = 0x30;
const MODE_SLOW: u32 = 0x36;
const MODE_SLOW_ADJ: u32 = 0x35;

// Id chains tried through the channel resolver, in order.
const FIRST_CHAIN: [u32; 6] = [0x35, 0x36, 0x30, 0x31, 0x26, 0x27];
const SECOND_CHAIN: [u32; 10] = [0x12, 0x15, 0x16, 0x13, 0x14, 0x17, 0x1A, 0x1B, 0x18, 0x19];
const FINAL_ID: u32 = 0x0F;
const BIND_SLOTS: u32 = 4;

#[inline(always)]
unsafe fn r32(a: u32) -> u32 {
    *(a as *const u32)
}

#[inline(always)]
unsafe fn w32(a: u32, v: u32) {
    *(a as *mut u32) = v;
}

#[inline(always)]
unsafe fn r8(a: u32) -> u8 {
    *(a as *const u8)
}

#[inline(always)]
unsafe fn rf(a: u32) -> f32 {
    f32::from_bits(r32(a))
}

/// Game float constant by file VA.
#[inline(always)]
unsafe fn gf(va: u32) -> f32 {
    f32::from_bits(r32(relocated(va)))
}

/// Game word global by file VA.
#[inline(always)]
unsafe fn gw(va: u32) -> u32 {
    r32(relocated(va))
}

// Reset path: clear the match slots, lock the blend factor, report -2.
#[inline(always)]
unsafe fn reset_path(record: u32) -> f32 {
    w32(record + REC_MATCH3, 0xFFFF_FFFF);
    w32(record + REC_MATCH2, 0xFFFF_FFFF);
    w32(record + REC_MATCH1, 0xFFFF_FFFF);
    w32(record + REC_MATCH0, 0xFFFF_FFFF);
    w32(record + REC_RATE, BLEND_LOCKED);
    w32(record + REC_BLEND, FRESH_BLEND);
    gf(F_NEG_TWO)
}

// See the published copy in out/rewrites for the full specification; the
// export macro cannot carry a doc comment.
export!(thiscall, rw_cc6020(owner: u32, frame: u32, slot: u32, weight_ptr: u32, level_ptr: u32, bias_bits: u32) -> f32 {
    unsafe {
        let bb = core::hint::black_box;
        let record = r32(frame + CTX_RECORD);
        let channel = r32(frame + CTX_CHANNEL);
        let tick_left = gw(G_TICK).wrapping_sub(r32(record + REC_TICK));

        // Poll the record through its update entry; a null answer skips the
        // gain stage and carries the incoming bias instead.
        let vt: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(r32(r32(record + REC_VTABLE) + VT_UPDATE));
        let picked = vt(record, channel);
        let picked_any = (picked != 0) as u8;
        let mut biased: u32;
        if picked != 0 {
            let gain: f32 = callee_thiscall!(C_GAIN, f32, picked);
            let gate = gf(F_GATE_085);
            if !(0.0f32 > gain) || !(gate > rf(picked + OBJ_GAIN)) {
                biased = bias_bits;
            } else {
                biased = 0.0f32.to_bits();
            }
        } else {
            biased = bias_bits;
        }

        // Level dispatch: a positive level, or a record level above one, runs
        // the timed path; an exact record level of -2 re-checks the state
        // gate first; anything else falls to the fallback chains.
        let level = rf(level_ptr);
        let mut fallback = false;
        if !(level > 0.0f32) {
            let rec_level = rf(record + REC_LEVEL);
            let one = gf(F_ONE);
            if !(rec_level > one) {
                if rec_level != gf(F_NEG_TWO) {
                    fallback = true;
                } else {
                    let ok: u32 = callee_thiscall!(C_GATE2, u32, owner, frame, 2);
                    if (ok as u8) != 0 {
                        fallback = false;
                    } else {
                        fallback = picked_any == 0;
                    }
                }
            }
        }
        if fallback {
            return fallback_path(owner, frame, slot, level_ptr, record, channel, biased);
        }

        // Timed path. A nonzero bias, a held record, or any state but -1
        // falls back to the chains.
        if f32::from_bits(biased) != 0.0f32 {
            return fallback_path(owner, frame, slot, level_ptr, record, channel, biased);
        }
        if r8(record + REC_FLAGS) & 0x20 != 0 {
            return fallback_path(owner, frame, slot, level_ptr, record, channel, biased);
        }
        if r32(record + REC_STATE) != 0xFFFF_FFFF {
            return fallback_path(owner, frame, slot, level_ptr, record, channel, biased);
        }
        let probe: u32 = callee_thiscall!(C_PROBE, u32, record, channel);
        // The running accumulator starts at zero; the original keeps it in
        // the fourth argument slot, which the contract does not compare.
        let mut acc_bits: u32 = 0.0f32.to_bits();
        if gf(F_ONE_HALF) > rf(level_ptr) && (probe as u8) == 0 && tick_left < gw(G_TICK_LIMIT) {
            return reset_path(record);
        }
        if picked_any != 0 {
            return reset_path(record);
        }
        if ((r32(record + REC_FLAGS) >> 8) as u8) & 1 != 0 {
            return reset_path(record);
        }

        // Quality switches choose the two thresholds for this tick.
        let q0: u32 = callee_thiscall!(C_QUALITY, u32, frame);
        let thresh_a = if (q0 as u8) != 0 { gf(F_TWO) } else { gf(F_LO_FALLBACK) };
        let q1: u32 = callee_thiscall!(C_QUALITY, u32, frame);
        let thresh_b = if (q1 as u8) != 0 { gf(F_ONE) } else { gf(F_ONE_HALF) };
        let level_now = rf(level_ptr);
        let mut mode: u32 = MODE_FAST;
        let mut run_bindings = false;
        if thresh_b > level_now && rf(weight_ptr) > 0.0f32 {
            mode = MODE_FAST;
            if r8(owner + OWN_FLAGS) & 8 != 0 {
                run_bindings = true;
            } else {
                return reset_path(record);
            }
        } else if thresh_a > level_now && rf(weight_ptr) > 0.0f32 {
            mode = MODE_MID;
            if r8(owner + OWN_FLAGS) & 0x10 != 0 {
                run_bindings = true;
            } else {
                return reset_path(record);
            }
        } else {
            if gf(F_THREE) >= level_now && rf(weight_ptr) > 0.0f32 {
                mode = MODE_SLOW;
                if r8(owner + OWN_FLAGS) & 0x10 != 0 {
                    run_bindings = true;
                } else {
                    return reset_path(record);
                }
            } else {
                return fallback_path(owner, frame, slot, level_ptr, record, channel, biased);
            }
        }
        if run_bindings {
            // Resolve each live binding id through the channel; the newest
            // enabled gain found becomes the accumulator, then every visited
            // slot is parked.
            let mut binding = record + REC_MATCH0;
            let mut left = BIND_SLOTS;
            while left != 0 {
                let id = r32(binding);
                if id != 0xFFFF_FFFF {
                    let found: u32 = callee_thiscall!(C_FIND_LOOP, u32, channel, id);
                    if found != 0 && ((r32(found + OBJ_FLAGS) >> 8) as u8) & 1 != 0 {
                        acc_bits = r32(found + OBJ_GAIN);
                    }
                    w32(binding, 0xFFFF_FFFF);
                }
                binding += 4;
                left -= 1;
            }
            // Range-fold the accumulator into its band before matching.
            let mut acc = f32::from_bits(acc_bits);
            let three_q = gf(F_THREE_Q);
            if gf(F_QUARTER) > acc || acc >= three_q {
                mode = match mode {
                    MODE_FAST => MODE_FAST_ADJ,
                    MODE_MID => MODE_MID_ADJ,
                    MODE_SLOW => MODE_SLOW_ADJ,
                    m => m,
                };
                // A folded band passes the accumulator through untouched;
                // only an over-range one is cleared.
                if acc >= three_q {
                    acc_bits = 0.0f32.to_bits();
                }
            } else if !(acc >= gf(F_HALF)) {
                acc_bits = 0.0f32.to_bits();
            } else {
                acc = bb(acc) - bb(gf(F_HALF));
                acc_bits = acc.to_bits();
            }
            // Match the owner tags, touch the channel, then pass the blend
            // gate; any refusal reports zero.
            let tag = r32(owner + OWN_TAG);
            let m0: u32 = callee_cdecl!(C_MATCH2, u32, tag, mode);
            if m0 == 0 {
                let tag2 = r32(owner + OWN_TAG2);
                if tag2 == 0xFFFF_FFFF {
                    return 0.0f32;
                }
                let m1: u32 = callee_cdecl!(C_MATCH2B, u32, tag2, mode);
                if m1 == 0 {
                    return 0.0f32;
                }
            }
            let _: u32 = callee_thiscall!(C_TOUCH, u32, channel, 0);
            let blend_ok: u32 = callee_cdecl!(
                C_BLEND5, u32,
                channel, r32(owner + OWN_TAG), mode, BITS_BLEND_ARG, r32(owner + OWN_TAG2)
            );
            if blend_ok == 0 {
                return 0.0f32;
            }
            let _: u32 = callee_thiscall!(C_SEED, u32, blend_ok, acc_bits);
            w32(record + REC_BLEND, FRESH_BLEND);
            w32(record + REC_RATE, BLEND_LOCKED);
            return gf(F_NEG_ONE);
        }
        reset_path(record)
    }
});

// Fallback subtree: id chains, the pair gate, the commit loop and the level
// tail. Split out only because the timed dispatch above reaches it twice;
// behaviour is one function.
#[inline(always)]
unsafe fn fallback_path(
    owner: u32,
    frame: u32,
    slot: u32,
    level_ptr: u32,
    record: u32,
    channel: u32,
    biased: u32,
) -> f32 {
    let bb = core::hint::black_box;
    // A nonzero level skips the first chain and retires the slot first.
    if rf(level_ptr) != 0.0f32 {
        return retire_path(owner, frame, slot, level_ptr, record, channel);
    }
    for id in FIRST_CHAIN {
        let found: u32 = callee_thiscall!(C_FIND_FIRST, u32, channel, id);
        w32(slot, found);
        if found != 0 {
            // A gated pair measurement below the arc admits the adjust entry.
            if r8(relocated(G_GATE_BYTE)) != 0
                && r8(frame + CTX_MODE_FLAG) & 4 == 0
            {
                if f32::from_bits(biased) != gf(F_ZERO) {
                    return retire_path(owner, frame, slot, level_ptr, record, channel);
                }
                let pair: f32 =
                    callee_cdecl!(C_PAIR, f32, r32(frame + CTX_PAIR_B), r32(frame + CTX_PAIR_A));
                let mag = f32::from_bits(pair.to_bits() & r32(relocated(ABS_MASK_W)));
                if mag > gf(F_ARC) {
                    return retire_path(owner, frame, slot, level_ptr, record, channel);
                }
            }
            // The adjust entry blends the planted gain with the stored one
            // and reports -1 while under the cap.
            let held = r32(slot);
            if ((r32(held + OBJ_FLAGS) >> 4) as u8) & 1 != 0 {
                let vt: extern "thiscall" fn(u32) -> f32 =
                    core::mem::transmute(r32(r32(held + OBJ_VTABLE) + VT_ADJUST));
                let extra = vt(held);
                let total = bb(extra) + bb(rf(held + OBJ_GAIN));
                // Below-or-unordered, as above.
                if !(total >= gf(G_BLEND_CAP)) {
                    return gf(F_NEG_ONE);
                }
            }
            let _: u32 = callee_thiscall!(C_DECAY, u32, r32(slot), BITS_NEG_ONE_F);
            w32(slot, 0);
            return 0.0f32;
        }
    }
    // Every id missed: poll the record once more and report -2, or zero.
    let vt: extern "thiscall" fn(u32, u32) -> u32 =
        core::mem::transmute(r32(r32(record + REC_VTABLE) + VT_UPDATE));
    if vt(record, channel) == 0 {
        return 0.0f32;
    }
    gf(F_NEG_TWO)
}

// Retire the slot, sweep the second id chain, and either commit a found
// object or answer from the level tail.
#[inline(always)]
unsafe fn retire_path(
    owner: u32,
    frame: u32,
    slot: u32,
    level_ptr: u32,
    record: u32,
    channel: u32,
) -> f32 {
    let bb = core::hint::black_box;
    let held = r32(slot);
    if held != 0 {
        let _: u32 = callee_thiscall!(C_DECAY, u32, held, BITS_NEG_ONE_F);
    }
    w32(record + REC_STATE, 0xFFFF_FFFF);
    w32(slot, 0);
    let mut found_obj: u32 = 0;
    for id in SECOND_CHAIN {
        let found: u32 = callee_thiscall!(C_FIND_SECOND, u32, channel, id);
        if found != 0 {
            found_obj = found;
            break;
        }
    }
    if found_obj == 0 {
        return level_tail(owner, frame, level_ptr, record, 0);
    }
    // Gain staging: a negative gain, or a marked object whose second gain
    // reads exactly zero, drops to the level tail.
    let g0: f32 = callee_thiscall!(C_GAIN, f32, found_obj);
    // Ordered-below-or-unordered: the jump the original takes is also taken
    // for a NaN gain, so this is the negation of `>=`, not `<`.
    if !(g0 >= 0.0f32) {
        return level_tail(owner, frame, level_ptr, record, found_obj);
    }
    if ((r8(found_obj + OBJ_MARK) >> 2) & 1) != 0 {
        let g1: f32 = callee_thiscall!(C_GAIN, f32, found_obj);
        if g1 == 0.0f32 {
            return level_tail(owner, frame, level_ptr, record, found_obj);
        }
    }
    // Blend staging: an unmarked flagged object sums two terms and must
    // reach the ceiling, else the answer is exactly one.
    if ((r32(found_obj + OBJ_FLAGS) >> 4) as u8) & 1 != 0
        && ((r8(found_obj + OBJ_MARK) >> 2) & 1) == 0
    {
        let t0: f32 = callee_thiscall!(C_SUM_A, f32, found_obj);
        let t1: f32 = callee_thiscall!(C_SUM_B, f32, found_obj);
        let total = bb(t0) + bb(t1);
        let ceil: f32 = callee_thiscall!(C_SUM_C, f32, found_obj);
        // Below-or-unordered, as above.
        if !(total >= ceil) {
            return 1.0f32;
        }
    }
    // Commit: class ids stamp the record and raise the level flag.
    let code = r32(found_obj + OBJ_CODE);
    let mut high = false;
    if code == 0x18 || code == 0x19 || code == 0x1A || code == 0x1B {
        w32(record + REC_CLASS_A, BLEND_LOCKED);
        w32(record + REC_CLASS_B, 0);
        w32(record + REC_CLASS_C, 0);
        w32(record + REC_LEVEL_COPY, r32(relocated(G_LEVEL_STORE)));
        high = true;
    } else if code == 0x17 {
        high = true;
    }
    let _: u32 = callee_thiscall!(C_COMMIT, u32, owner, channel);
    // Sweep the owner table: any live match id still present there, and its
    // record slot, is parked.
    let mut want0 = r32(record + REC_MATCH0);
    let mut want1 = r32(record + REC_MATCH1);
    let mut want2 = r32(record + REC_MATCH2);
    let mut want3 = r32(record + REC_MATCH3);
    let mut row = owner + OWN_TABLE;
    let mut left = OWN_ROWS;
    while left != 0 {
        let have = r32(row);
        if want0 == have {
            want0 = 0xFFFF_FFFF;
            w32(record + REC_MATCH0, 0xFFFF_FFFF);
        }
        if want1 == have {
            want1 = 0xFFFF_FFFF;
            w32(record + REC_MATCH1, 0xFFFF_FFFF);
        }
        if want2 == have {
            want2 = 0xFFFF_FFFF;
            w32(record + REC_MATCH2, 0xFFFF_FFFF);
        }
        if want3 == have {
            want3 = 0xFFFF_FFFF;
            w32(record + REC_MATCH3, 0xFFFF_FFFF);
        }
        row += OWN_STRIDE;
        left -= 1;
    }
    let last: u32 = callee_thiscall!(C_FIND_LAST, u32, channel, FINAL_ID);
    if last != 0 {
        let _: u32 = callee_thiscall!(C_DECAY, u32, last, BITS_HARD_DECAY);
    }
    // The level flag selects the stored level and the reported constant.
    if high {
        w32(level_ptr, LEVEL_HIGH_BITS);
        w32(record + REC_BLEND, LEVEL_HIGH_BITS);
        gf(F_THREE)
    } else {
        w32(level_ptr, LEVEL_LOW_BITS);
        w32(record + REC_BLEND, LEVEL_LOW_BITS);
        gf(F_TWO)
    }
}

// Level tail: the piecewise answer over the stored level.
#[inline(always)]
unsafe fn level_tail(owner: u32, frame: u32, level_ptr: u32, record: u32, staged: u32) -> f32 {
    let bb = core::hint::black_box;
    let level = rf(level_ptr);
    if level == 0.0f32 {
        return 0.0f32;
    }
    let one = gf(F_ONE);
    // At-or-below-one, ordered: an unordered level takes the upper region.
    if one >= level {
        // At or below one: the tail gate, the owner and record flags, and a
        // final non-negative gain check all report two; a missing staged
        // object, or a negative gain, reports exactly one.
        let ok: u32 = callee_thiscall!(C_GATE2B, u32, owner, frame, 2);
        if ok != 0 {
            return gf(F_TWO);
        }
        if r8(owner + OWN_FLAGS) & 1 == 0 {
            return gf(F_TWO);
        }
        if r32(record + REC_FLAGS) & 0x20000 != 0 {
            return gf(F_TWO);
        }
        if staged == 0 {
            return 1.0f32;
        }
        let g: f32 = callee_thiscall!(C_GAIN, f32, staged);
        if !(0.0f32 > g) {
            return gf(F_TWO);
        }
        return 1.0f32;
    }
    let two = gf(F_TWO);
    if two > level {
        return bb(level) + bb(one);
    }
    if level == two {
        return gf(F_THREE);
    }
    let three = gf(F_THREE);
    if three > level {
        return bb(level) + bb(one);
    }
    if level != three {
        return 0.0f32;
    }
    gf(F_FOUR)
}
