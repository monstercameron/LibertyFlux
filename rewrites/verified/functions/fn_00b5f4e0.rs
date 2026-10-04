// original: 0x00b5f4e0 STAY_DOWN
//! Audio level mix for one entity/voice pair.
//!
//! NOTE: `STAY_DOWN` is the inventory's placeholder token for this address
//! (low confidence, marked meaningless); the descriptive name used below is
//! `audio_mix_level`. The function is a thiscall taking
//! (this, voice, param_triple, param_block, scalar) and returning an f32
//! level in ST0: it resolves an entity through a lookup helper, runs the
//! entity's level virtual (slot 0xFC), mixes the parameter triple with the
//! scalar through a 9-argument mixer, and returns the mixer's output. It
//! returns 0.0 early for unrecognized info records, drives a voice
//! allocator/setup pair for live voices, and commits a level delta through
//! a 2-argument stdcall when the recomputed level is strictly positive
//! (NaN falls through, matching comiss/jbe).
//!
//! Contract: out/contracts/fn_00b5f4e0.json (10 callees, 12 heap segments).
//! Verified: 1000/1000 trials, fp bit-exact, orig_ok 1000. Callee 9's taken
//! branch is unreachable under per-trial callee scripts (both virtual calls
//! share one script slot, so the delta is always exactly zero); the skip is
//! verified on every trial and the taken branch was reviewed by hand.
#[inline(always)]
fn r32(addr: u32) -> u32 {
    unsafe { *(addr as *const u32) }
}

#[inline(always)]
fn r8(addr: u32) -> u8 {
    unsafe { *(addr as *const u8) }
}

#[inline(always)]
fn w32(addr: u32, val: u32) {
    unsafe { *(addr as *mut u32) = val }
}

/// Entity virtual slot 0xFC: loads the slot through the object's table and
/// calls it exactly like the original, so both sides land on the same
/// planted recorder stub.
#[inline(always)]
fn entity_level(entity: u32) -> f32 {
    let table = r32(entity);
    let slot = r32(table.wrapping_add(0xfc));
    let f: extern "thiscall" fn(u32) -> f32 =
        unsafe { core::mem::transmute(slot as usize) };
    f(entity)
}

/// Resolve the entity for the parameter block at `s10`, run its level
/// virtual, mix the parameter triple from `s_c` with the scalar `s14`,
/// and return the mixer's output sample.
///
/// Returns 0.0 early when the info record for this object reports an
/// unrecognized kind. Otherwise drives the voice allocator/setup pair for
/// live voices, runs the post step, and commits a level delta when the
/// recomputed level is strictly positive.
export!(thiscall, rw_rb64_f3(this_: u32, s8: u32, s_c: u32, s10: u32, s14: u32) -> f32 {
    let esi = s8;
    let edi = callee_cdecl!(0, u32, r32(s10));

    // First dispatch: an entity already carrying the live flag goes
    // straight to the info-record path; otherwise the voice selects it.
    let e6c = r32(edi.wrapping_add(0x6c));
    let entity_live = e6c != 0 && r8(e6c.wrapping_add(0xe)) != 0;
    let mut via_info = entity_live;
    if !entity_live {
        if esi != 0 {
            let s6c = r32(esi.wrapping_add(0x6c));
            if s6c != 0 && r8(s6c.wrapping_add(0xe)) != 0 {
                via_info = r8(this_.wrapping_add(0x58)) == 0;
            }
        }
    }
    if via_info {
        let rec = callee_cdecl!(1, u32, r32(this_.wrapping_add(0x18)));
        if r32(rec.wrapping_add(0xc)) != 1 {
            // Unrecognized record kind: run the sibling mix for a live
            // entity, then return silence.
            let e6c2 = r32(edi.wrapping_add(0x6c));
            if e6c2 != 0 && r8(e6c2.wrapping_add(0xe)) != 0 {
                callee_thiscall!(2, u32, this_, esi, s_c, s10);
            }
            return 0.0;
        }
    }

    // Level path: virtual level plus the entity's stored base. The first
    // virtual result stays live in its frame slot and is reused below as
    // the minuend of the level delta (it is NOT the mixer's output).
    let f1 = entity_level(edi);
    let f2 = f32::from_bits(r32(edi.wrapping_add(0xb84)));

    if esi != 0 {
        let rec = callee_cdecl!(1, u32, r32(this_.wrapping_add(0x18)));
        let kind = (r32(esi.wrapping_add(0x28)) >> 6) & 0xf;
        let target = if kind == 3 {
            esi
        } else if kind == 2 {
            r32(esi.wrapping_add(0xf50))
        } else {
            0
        };
        // Count one use of the record when every gate agrees.
        if rec != 0
            && target != 0
            && r8(target.wrapping_add(0x218)) == 0
            && r8(target.wrapping_add(0x219)) != 0
            && r8(edi.wrapping_add(0xa60)) == 2
            && r8(edi.wrapping_add(0x210)) == 0
        {
            let cell = rec.wrapping_add(0x104);
            w32(cell, r32(cell).wrapping_add(1));
        }
    }

    // Parameter mix. The frame slot carrying the first triple element is
    // passed by address (compared by content, not address); the scalar
    // arrives as the fourth argument.
    let t0 = f32::from_bits(r32(s_c.wrapping_add(0x30)));
    let _t1 = f32::from_bits(r32(s_c.wrapping_add(0x34)));
    let _t2 = f32::from_bits(r32(s_c.wrapping_add(0x38)));
    let mut first_slot = t0;
    let mix = callee_cdecl!(
        3, f32,
        esi,
        edi,
        r32(this_.wrapping_add(0x18)),
        s14,
        (&mut first_slot as *mut f32) as u32,
        s10,
        this_,
        0,
        0
    );
    let _ = (_t1, _t2);

    // Voice bring-up for live voices.
    if esi != 0
        && (r32(esi.wrapping_add(0x28)) & 0x3c0) == 0xc0
        && r8(esi.wrapping_add(0x218)) == 0
        && r8(esi.wrapping_add(0x219)) != 0
        && r8(edi.wrapping_add(0x211)) != 0
        && r8(edi.wrapping_add(0x212)) == 0
    {
        let bus = r32(edi.wrapping_add(0x21c));
        let tag = if r32(bus.wrapping_add(0x12c)) == 2 {
            let got = callee_cdecl!(
                4, u32,
                esi,
                r32(esi.wrapping_add(0x20)).wrapping_add(0x30),
                0x4270_0000u32,
                0xffff_ffffu32,
                0xffff_ffffu32,
                edi,
                0
            );
            if got == 0 {
                relocated(0x00eb1074)
            } else {
                relocated(0x00eb1080)
            }
        } else {
            relocated(0x00eb1080)
        };
        callee_thiscall!(
            5, u32,
            esi.wrapping_add(0x570),
            tag,
            0,
            0,
            0,
            0xffff_ffffu32,
            0,
            0,
            0x3f80_0000u32,
            0,
            0
        );
    }

    callee_thiscall!(6, u32, this_, edi, s10);
    let gate = callee_cdecl!(7, u32,);
    if (gate & 0xff) == 0 {
        return mix;
    }

    // Recompute the level and commit a positive delta.
    let f3 = entity_level(edi);
    // Same operations in the same order as the original: x0 starts from
    // the FIRST virtual result (still in its slot), the middle term
    // cancels only for finite values, and NaN must stay NaN.
    let mut x0 = f1;
    let mut x1 = f2;
    x0 -= f3;
    x1 -= f32::from_bits(r32(edi.wrapping_add(0xb84)));
    x1 += x0;
    // comiss/jbe: continue only for strictly greater (NaN falls through).
    if esi != 0 && x1 > 0.0 {
        let kind = (r32(esi.wrapping_add(0x28)) >> 6) & 0xf;
        let sel = if kind <= 1 || kind >= 5 {
            0
        } else {
            r32(esi.wrapping_add(0x6c))
        };
        let host = r32(edi.wrapping_add(0x6c));
        if host != 0 && sel != 0 {
            callee_stdcall!(9, u32, sel, x1.to_bits());
        }
    }
    mix
});
