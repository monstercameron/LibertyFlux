// original: 0x009838C0 WAVES_BREAK_HEAD
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated, tls_slot};

/// WAVES_BREAK_HEAD: per-frame update of one wave-break voice slot.
///
/// `this` points to a 0x160-byte slot object; `arg` is a float (level
/// target) passed by value on the stack. Returns nothing.
///
/// Behaviour, from the entry dispatch onward:
/// - Byte `B_MODE152` selects the path: 0 runs the multi-entry sweep
///   (`second_half`), nonzero runs the single-voice path.
/// - The single-voice path lazily initialises two hashed ids the first
///   time (guarded by `G_HASH_FLAG`), then dispatches on the state word:
///   0 initialises a fresh slot, 1 arms it (and may exit early when the
///   stored base already exceeds the target), 2 runs the main update,
///   anything else exits after clearing the active flag.
/// - The main update accumulates level and gain from the target, the
///   per-slot weight tables and global tunables, resolves the voice
///   handle through the global voice table, programs it through a
///   fixed chain of engine calls (filter, truncate, route, commit),
///   and clears the active flag.
/// - The sweep walks up to 8 entries (`C_COUNT` clamped), probing each
///   one through the engine, recording a pass/fail byte per entry and
///   refreshing peaks, limits and random retrigger delays.
///
/// Calling convention: thiscall with one stack argument; callee pops 4.
/// Float comparisons follow `comiss` semantics exactly (an unordered
/// result behaves as greater for `jb`/`jbe`-style branches), integer
/// arithmetic wraps, and table scaling uses full 32-bit products.
export!(thiscall, rw_009838C0(this: u32, arg: u32) -> () {
    unsafe { waves_break_head(this, arg); }
});

// ---- slot object layout (offsets from `this`) ----
const ST_STATE: u32 = 0x00; // state word: 0 fresh, 1 armed, 2 live
const F_LEVEL: u32 = 0x04; // accumulated level
const I_CURSOR: u32 = 0x08; // pair-table cursor
const F_BASE: u32 = 0x10; // stored base the target is measured against
const F_GAIN: u32 = 0x14; // accumulated gain
const P_VOICE: u32 = 0x18; // voice handle (0 or pointer to voice struct)
const F_AUX: u32 = 0x1c; // auxiliary term folded into the base at init
const F_SCALE: u32 = 0x20; // per-slot scale applied to the target
const F_DIV: u32 = 0x28; // divisor applied to the gain
const P_PAIRS: u32 = 0x2c; // pointer to float-pair table (8 bytes each)
const C_COUNT: u32 = 0x30; // u16 entry count (sweep length, clamped to 8)
const P_WEIGHTS: u32 = 0x34; // pointer to per-cursor weight floats
const B_FLAG151: u32 = 0x151; // active flag, cleared on every exit
const B_MODE152: u32 = 0x152; // path selector: 0 sweep, nonzero single

// ---- voice struct layout (offsets from the handle) ----
const V_CLASS: u32 = 0x04; // class byte (0xff means "no voice")
const V_BANK: u32 = 0x40; // bank byte selecting the table row

// ---- globals ----
const G_HASH_A: u32 = 0x123883c; // first lazily hashed id
const G_HASH_B: u32 = 0x1238840; // second lazily hashed id
const G_HASH_FLAG: u32 = 0x1238844; // low bit set once the ids are hashed
const G_TLS_SLOT: u32 = 0x17aba14; // TLS slot holding the audio context
const G_VOICE_MUL: u32 = 0x115d968; // voice-table column stride factor
const G_VOICE_TAB: u32 = 0x115d988; // voice-table base
const G_PAIR_TAB: u32 = 0x115df20; // context pair table (64 bytes each)
const G_BLEND: u32 = 0x12ddeb4; // global blend float
const G_LIMIT: u32 = 0x11735b4; // global retrigger limit

// ---- read-only float tunables ----
const K_ONE: u32 = 0xfe88e8; // 1.0
const K_HUNDRED: u32 = 0xfe8bb0; // 100.0
const K_FORTY: u32 = 0xfe8b5c; // 40.0
const K_INV60: u32 = 0xfe8724; // 1/60
const K_SQ_LIMIT: u32 = 0xfe8cb0; // 10000.0
const K_THREE: u32 = 0xfe8a94; // 3.0

// ---- callee ids (see the contract for sites and scripts) ----
const C_JOAAT_A: u32 = 1;
const C_SPAWN: u32 = 2; // 17-arg voice spawner
const C_XFORM: u32 = 3; // pair transform, answers two floats via ecx
const C_FILT: u32 = 4; // filter, answers f32 on the x87 stack
const C_SHAPE: u32 = 5; // shaper (cdecl), answers f32 on the x87 stack
const C_ROUTE2: u32 = 6; // voice router (float flavour)
const C_TRUNC: u32 = 7; // float-to-int truncation of the filter answer
const C_ROUTE1: u32 = 8; // voice router (index flavour)
const C_COMMIT_V: u32 = 9; // voice commit
const C_COMMIT_F: u32 = 10; // frame commit (takes a frame pointer)
const C_RELEASE: u32 = 11; // voice release
const C_PROBE: u32 = 12; // sweep probe (cdecl, 10 args)
const C_ZERO: u32 = 13; // frame-struct initialiser (zeroes 16 words)
const C_GATE: u32 = 14; // gate, answers 0/1 in al
const C_SUBMIT: u32 = 15; // sweep submit (takes heap + frame pointers)
const C_RAND: u32 = 16; // random integer in range (cdecl)
const C_JOAAT_B: u32 = 17;

// ---- fixed engine-object addresses passed as `this` ----
const O_SPAWN: u32 = 0x1231800;
const O_FILT_A: u32 = 0x1238760;
const O_FILT_B: u32 = 0x1231788;
const O_FILT_C: u32 = 0x12387b0;
const O_FILT_D: u32 = 0x12317d8;
const O_GATE: u32 = 0x1165880;

// ---- hashed names ----
const S_NAME_A: u32 = 0xe8d844;
const S_NAME_B: u32 = 0xe8d858;
const S_SUB_A: u32 = 0xe8d878;
const S_SUB_B: u32 = 0xe8d88c;

// ---- magic numbers ----
const VOICE_ROW: u32 = 0x6f40; // voice-table row stride
const VOICE_OFF: u32 = 0x6f14; // voice-table first-row offset
const SWEEP_MAX: u32 = 8;
const RAND_LO: u32 = 0x1388;
const RAND_HI: u32 = 0x36b0;

#[inline(always)]
unsafe fn rdu(base: u32, off: u32) -> u32 {
    unsafe { ((base.wrapping_add(off)) as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn rdf(base: u32, off: u32) -> f32 {
    unsafe { ((base.wrapping_add(off)) as *const f32).read_unaligned() }
}

#[inline(always)]
unsafe fn rdu8(base: u32, off: u32) -> u8 {
    unsafe { ((base.wrapping_add(off)) as *const u8).read() }
}

#[inline(always)]
unsafe fn rdu16(base: u32, off: u32) -> u16 {
    unsafe { ((base.wrapping_add(off)) as *const u16).read_unaligned() }
}

#[inline(always)]
unsafe fn wru(base: u32, off: u32, v: u32) {
    unsafe { ((base.wrapping_add(off)) as *mut u32).write_unaligned(v) }
}

#[inline(always)]
unsafe fn wrf(base: u32, off: u32, v: f32) {
    unsafe { ((base.wrapping_add(off)) as *mut f32).write_unaligned(v) }
}

#[inline(always)]
unsafe fn wru8(base: u32, off: u32, v: u8) {
    unsafe { ((base.wrapping_add(off)) as *mut u8).write(v) }
}

#[inline(always)]
unsafe fn g32(va: u32) -> u32 {
    unsafe { global::<u32>(va).read_unaligned() }
}

#[inline(always)]
unsafe fn gf(va: u32) -> f32 {
    unsafe { *global::<f32>(va) }
}

/// Every exit clears the active flag; the epilogue just returns.
#[inline(always)]
unsafe fn exit_slot(esi: u32) {
    unsafe { wru8(esi, B_FLAG151, 0) }
}

/// Voice-handle resolution shared by the two router calls: class 0xff
/// means no voice (null handle), otherwise the handle is the table
/// entry for the bank byte plus the class byte scaled by the stride.
unsafe fn resolve_voice(handle: u32) -> u32 {
    unsafe {
        let class = rdu8(handle, V_CLASS);
        if class == 0xff {
            return 0;
        }
        let bank = rdu8(handle, V_BANK) as u32;
        let factor = g32(G_VOICE_MUL);
        let tab = g32(G_VOICE_TAB);
        let entry = rdu(
            tab.wrapping_add(bank.wrapping_mul(VOICE_ROW)),
            VOICE_OFF,
        );
        entry.wrapping_add((class as u32).wrapping_mul(factor))
    }
}

unsafe fn waves_break_head(esi: u32, arg: u32) {
    unsafe {
        if rdu8(esi, B_MODE152) == 0 {
            return second_half(esi);
        }
        let flag = g32(G_HASH_FLAG);
        if flag & 1 == 0 {
            global::<u32>(G_HASH_FLAG).write_unaligned(flag | 1);
            let h1: u32 = callee_cdecl!(C_JOAAT_A, u32, relocated(S_NAME_A), 0);
            global::<u32>(G_HASH_A).write_unaligned(h1);
            let h2: u32 = callee_cdecl!(C_JOAAT_B, u32, relocated(S_NAME_B), 0);
            global::<u32>(G_HASH_B).write_unaligned(h2);
        }
        let farg = f32::from_bits(arg);
        match rdu(esi, ST_STATE) {
            0 => teardown_init(esi),
            1 => {
                let x0 = rdf(esi, F_BASE) - farg;
                wrf(esi, F_BASE, x0);
                // jb after comiss(0.0, x0): taken unless 0.0 >= x0.
                if !(0.0 >= x0) {
                    return exit_slot(esi);
                }
                wru(esi, ST_STATE, 2);
                wru(esi, F_LEVEL, 0);
                wru(esi, I_CURSOR, 0);
                wru(esi, F_GAIN, 0);
                main_path(esi, farg);
            }
            2 => main_path(esi, farg),
            _ => exit_slot(esi),
        }
    }
}

/// State 0: fold the auxiliary term into the base and arm the slot.
unsafe fn teardown_init(esi: u32) {
    unsafe {
        if rdu16(esi, C_COUNT) == 0 {
            return exit_slot(esi);
        }
        let x0 = rdf(esi, F_AUX) + rdf(esi, 0x0c);
        wru(esi, 0x0c, 0);
        wru(esi, ST_STATE, 1);
        wrf(esi, F_BASE, x0);
        exit_slot(esi);
    }
}

/// Cursor ran past the table: release the voice and park the slot.
unsafe fn teardown(esi: u32, edi: u32) {
    unsafe {
        let voice = rdu(edi, 0);
        if voice != 0 {
            let _: u32 = callee_thiscall!(C_RELEASE, u32, voice, 0);
        }
        wru(esi, ST_STATE, 0);
        exit_slot(esi);
    }
}

unsafe fn main_path(esi: u32, farg: f32) {
    unsafe {
        let edi = esi.wrapping_add(P_VOICE);
        // Level accumulation: target * scale * weight + level.
        let mut x0 = rdf(esi, F_SCALE) * farg;
        let wtab = rdu(esi, P_WEIGHTS);
        let cur = rdu(esi, I_CURSOR);
        x0 *= rdf(wtab, cur.wrapping_mul(4));
        x0 += rdf(esi, F_LEVEL);
        wrf(esi, F_LEVEL, x0);
        // Gain accumulation: target * scale * 100 + gain.
        x0 = rdf(esi, F_SCALE) * farg;
        let one = gf(K_ONE);
        let mut slot_c = one;
        x0 *= gf(K_HUNDRED);
        x0 += rdf(esi, F_GAIN);
        wrf(esi, F_GAIN, x0);
        // Spawn a voice when none is attached yet.
        x0 = rdf(esi, F_LEVEL);
        let above = (x0 > one) as u8;
        if rdu(edi, 0) == 0 {
            let h1 = g32(G_HASH_A);
            let _: u32 = callee_thiscall!(
                C_SPAWN, u32, relocated(O_SPAWN), h1, edi, 0, 0, 0, 0, 0, 0, 0,
                0, 1, 0, 0, 0, 0xffffffff, 0, 0
            );
        }
        if above != 0 {
            x0 = rdf(esi, F_LEVEL);
            wru(esi, I_CURSOR, rdu(esi, I_CURSOR).wrapping_add(1));
            x0 -= one;
            wrf(esi, F_LEVEL, x0);
        }
        let count = rdu16(esi, C_COUNT) as u32;
        let cur = rdu(esi, I_CURSOR);
        // jae on the unsigned compare: park the slot at the end.
        if cur >= count.wrapping_sub(1) {
            return teardown(esi, edi);
        }
        // Interpolate the cursor pair by the level.
        let ttab = rdu(esi, P_PAIRS);
        let o = cur.wrapping_mul(8);
        let mut lerp0 = rdf(ttab, o.wrapping_add(8)) - rdf(ttab, o);
        lerp0 *= rdf(esi, F_LEVEL);
        lerp0 += rdf(ttab, o);
        let mut lerp1 = rdf(ttab, o.wrapping_add(12)) - rdf(ttab, o.wrapping_add(4));
        lerp1 *= rdf(esi, F_LEVEL);
        lerp1 += rdf(ttab, o.wrapping_add(4));
        // Transform the pair; the answers come back through the
        // out-words, followed by a zero word and an untouched fill
        // word the frame commit reads.
        let mut inout = [lerp0.to_bits(), lerp1.to_bits(), 0u32, 0u32, 0u32, 0u32];
        let _: u32 = callee_thiscall!(
            C_XFORM, u32,
            inout.as_mut_ptr().add(2) as u32,
            inout.as_mut_ptr() as u32,
            0
        );
        let out0 = f32::from_bits(inout[2]);
        let out1 = f32::from_bits(inout[3]);
        // Resolve the context row through TLS and measure the
        // squared distance to it.
        let x2 = rdf(esi, F_GAIN) / rdf(esi, F_DIV);
        let slot = g32(G_TLS_SLOT);
        let tls = tls_slot(slot as usize);
        let tidx = rdu(tls, 0x70);
        let base = relocated(G_PAIR_TAB).wrapping_add(tidx.wrapping_mul(64));
        let mut x4 = out0 - rdf(base, 0);
        let mut xx0 = out1 - rdf(base, 4);
        let mut xx1 = -rdf(base, 8);
        xx1 *= xx1;
        x4 *= x4;
        xx0 *= xx0;
        x4 += xx0;
        xx0 = gf(K_SQ_LIMIT);
        x4 += xx1;
        // Shaped falloff, skipped when the distance is already past
        // the limit; the result is clamped at zero from below.
        if xx0 > x4 {
            xx0 = x4.sqrt();
            xx0 -= gf(K_FORTY);
            xx0 *= gf(K_INV60);
            slot_c = xx0;
            if 0.0 > xx0 {
                slot_c = 0.0;
            }
        }
        if rdu(edi, 0) == 0 {
            return exit_slot(esi);
        }
        // Program the voice: filter, shape, route by index, commit.
        let r1: f32 = callee_thiscall!(C_FILT, f32, relocated(O_FILT_A), x2.to_bits());
        xx0 = r1 * slot_c;
        // The shaper's answer overwrites the filter's in the shared
        // slot, so the router below receives r2, not r1.
        let r2: f32 = callee_cdecl!(C_SHAPE, f32, xx0.to_bits());
        let voice = rdu(edi, 0);
        let this1 = resolve_voice(voice);
        let _: u32 = callee_thiscall!(C_ROUTE2, u32, this1, r2.to_bits());
        // Both filter calls below take the gain ratio, which sits in
        // its own slot untouched by the shaper's store.
        let _r3: f32 = callee_thiscall!(C_FILT, f32, relocated(O_FILT_B), x2.to_bits());
        let t: u32 = callee_cdecl!(C_TRUNC, u32,);
        let voice2 = rdu(edi, 0);
        let this2 = resolve_voice(voice2);
        let _: u32 = callee_thiscall!(C_ROUTE1, u32, this2, t);
        let r4: f32 = callee_thiscall!(C_FILT, f32, relocated(O_FILT_C), x2.to_bits());
        let this3 = rdu(edi, 0);
        let _: u32 = callee_thiscall!(C_COMMIT_V, u32, this3, r4.to_bits());
        let this4 = rdu(edi, 0);
        let _: u32 = callee_thiscall!(C_COMMIT_F, u32, this4, inout.as_mut_ptr().add(2) as u32);
        exit_slot(esi);
    }
}

/// Sweep path: probe up to 8 entries, recording one outcome byte each.
unsafe fn second_half(esi: u32) {
    unsafe {
        let count = (rdu16(esi, C_COUNT) as u32).min(SWEEP_MAX);
        if count == 0 {
            return exit_slot(esi);
        }
        let mut ptr = esi.wrapping_add(0x9c);
        let base28 = 0xffffff64u32.wrapping_sub(esi);
        let mut edi = esi.wrapping_add(0x3c);
        // Scratch struct the initialiser zeroes on every call; the
        // submit call then reads its first words.
        let mut e50 = [0u32; 20];
        loop {
            let tidx = rdu(edi, 0x88);
            let ttab = rdu(esi, P_PAIRS);
            let to = tidx.wrapping_mul(8);
            let t0 = rdf(ttab, to);
            let t4 = rdf(ttab, to.wrapping_add(4));
            // Probe the entry. argued-word 3 points at the
            // saved-register slot and argued-word 9 is stack
            // leftovers; both are skipped in the contract.
            let mut edxslot = 0u32;
            let _: u32 = callee_cdecl!(
                C_PROBE, u32, t0.to_bits(), t4.to_bits(), 0,
                &mut edxslot as *mut u32 as u32, 0, 0, 0, 0x40c00000u32,
                0x41a00000u32, 0
            );
            // The probe result slot is zeroed beforehand and the
            // probe cannot reach it, so the outcome is always 0.0.
            let x1 = 0.0f32;
            let fedi = rdf(edi, 0);
            let x2 = x1 - fedi;
            let gt = x1 > fedi;
            let slot18 = x1;
            let slot20 = x2;
            let slotb = gt as u8;
            let run_mid =
                rdu(edi, 0x68) == 0 && !gt && rdu8(ptr, 0) != 0;
            if run_mid {
                let _: u32 =
                    callee_thiscall!(C_ZERO, u32, e50.as_mut_ptr() as u32);
                let gate: u32 =
                    callee_thiscall!(C_GATE, u32, relocated(O_GATE), 0, 0);
                let sel = if (gate as u8) != 0 {
                    0.0
                } else {
                    gf(K_ONE)
                };
                let bf = gf(G_BLEND);
                let r5: f32 =
                    callee_thiscall!(C_FILT, f32, relocated(O_FILT_D), bf.to_bits());
                let x0m = r5 * sel;
                let r6: f32 = callee_cdecl!(C_SHAPE, f32, x0m.to_bits());
                // The store lands in the struct's first word: the
                // cdecl argument is still on the stack above it.
                e50[0] = r6.to_bits();
                if rdu8(esi, B_FLAG151) == 0 {
                    let _: u32 = callee_thiscall!(
                        C_SUBMIT, u32, relocated(O_SPAWN), relocated(S_SUB_A),
                        edi.wrapping_add(0x68), e50.as_mut_ptr() as u32,
                        0xffffffff, 0, 0
                    );
                    wru(edi, 0xa8, g32(G_LIMIT));
                }
            }
            wru8(ptr, 0, slotb);
            let flag151 = rdu8(esi, B_FLAG151);
            let do_store = if flag151 != 0 {
                true
            } else if slotb != 0 {
                slot18 > fedi
            } else {
                fedi > slot18
            };
            if do_store {
                wrf(edi, 0, slot18);
                wrf(edi, 0x20, slot20);
            }
            let sum = rdu(edi, 0xa8).wrapping_add(rdu(edi, 0xc8));
            if g32(G_LIMIT) > sum {
                let r: u32 = callee_cdecl!(C_RAND, u32, RAND_LO, RAND_HI);
                wru(edi, 0xc8, r);
                if rdu(edi, 0x68) == 0 {
                    let _: u32 =
                        callee_thiscall!(C_ZERO, u32, e50.as_mut_ptr() as u32);
                    // The pair reload here feeds dead frame stores,
                    // but the reads themselves are kept so any fault
                    // behaviour matches the original exactly.
                    let ttab2 = rdu(esi, P_PAIRS);
                    let o2 = tidx.wrapping_mul(8);
                    core::hint::black_box(rdf(ttab2, o2));
                    core::hint::black_box(rdf(ttab2, o2.wrapping_add(4)));
                    let bf = gf(G_BLEND);
                    let r7: f32 = callee_thiscall!(
                        C_FILT, f32, relocated(O_FILT_D), bf.to_bits()
                    );
                    let r8: f32 = callee_cdecl!(C_SHAPE, f32, r7.to_bits());
                    let x0s = r8 - gf(K_THREE);
                    e50[0] = x0s.to_bits();
                    let _: u32 = callee_thiscall!(
                        C_SUBMIT, u32, relocated(O_SPAWN), relocated(S_SUB_B),
                        edi.wrapping_add(0x68), e50.as_mut_ptr() as u32,
                        0xffffffff, 0, 0
                    );
                }
                wru(edi, 0xa8, g32(G_LIMIT));
            }
            ptr = ptr.wrapping_add(1);
            edi = edi.wrapping_add(4);
            // Loop while (ptr - esi - 155) < count, i.e. one pass
            // per entry; the addition wraps like the original's.
            let k = ptr.wrapping_add(base28);
            if k < count {
                continue;
            }
            break;
        }
        exit_slot(esi);
    }
}
