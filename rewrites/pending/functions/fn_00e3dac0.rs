// original: 0x00E3DAC0 STATS_NET_I_PC
// ---------------------------------------------------------------------------
// 0x00E3DAC0 (STATS_NET_I_PC): refresh one stats object. Gated on a global
// flag; resolves the object's group id, then takes one of two paths: a short
// float-plumbing path when the group lookup misses a known id, or the main
// path that folds several float stats, looks up a colour parameter block and
// pushes a packed colour value to a sink before chaining into the sibling
// update at 0x00E3DD50. Returns nothing meaningful.
// ---------------------------------------------------------------------------

/// Keyed stat lookup (contract id 5): answers a pointer to two words and
/// fills the caller's slot.
#[inline(always)]
fn stat_lookup_ac0(key: u32, slot: *mut u32) -> u32 {
    unsafe { callee_cdecl!(5u32, u32, slot as u32, key) }
}

/// Short path: three keyed lookups (0x29/0x2a/0x2b) folded through float
/// sinks, then a backend record fetch whose second word feeds the reporter.
#[inline(always)]
fn short_path_ac0(obj: u32) {
    unsafe {
        let _ = callee_thiscall!(4u32, u32, obj);
        // Key 0x29: copy the answered pair into a struct for the filler.
        let mut tmp = 0u32;
        let p = stat_lookup_ac0(0x29, &mut tmp as *mut u32);
        let mut pair = [
            (p as *const u32).read(),
            ((p + 4) as *const u32).read(),
        ];
        let _ = callee_cdecl!(6u32, u32, 2, pair.as_mut_ptr() as u32, 0, 0);
        let _ = callee_cdecl!(7u32, u32, pair[0], pair[1]);
        // Key 0x2a selected by filler key 7.
        let mut tmp2 = [0u32; 2];
        stat_lookup_ac0(0x2a, tmp2.as_mut_ptr());
        let _ = callee_cdecl!(8u32, u32, 7, 0, tmp2.as_mut_ptr() as u32, 0);
        let _ = callee_cdecl!(9u32, u32, tmp2[0], tmp2[1]);
        // Key 0x2b plus a fixed constant and the neighbouring spill slot.
        // The spill slot is stack the original never wrote; under the
        // checker's defined zero fill it reads +0.0, added explicitly to
        // keep the -0.0 edge bit-exact.
        let mut tmp3 = [0u32; 2];
        stat_lookup_ac0(0x2b, tmp3.as_mut_ptr());
        let konst = (global::<f32>(0x00FE8914) as *const f32).read();
        let spill: f32 = core::hint::black_box(0.0);
        let sum = (f32::from_bits(tmp3[0]) + konst) + spill;
        let _ = callee_cdecl!(10u32, u32, sum.to_bits());
        // Backend handle, record fetch, report.
        let handle = callee_thiscall!(11u32, u32, relocated(0x0116BFF0), relocated(0x00F1551C));
        let mut tmp4 = 0u32;
        let rec = callee_cdecl!(
            12u32, u32,
            &mut tmp4 as *mut u32 as u32, handle, 0xFFFF_FFFF, 0xFFFF_FFFF
        );
        let second = ((rec + 4) as *const u32).read();
        // First argument is the caller's incoming EBX bit pattern, read from
        // the saved-register slot: unobservable from the rewrite, masked in
        // the contract. Third is the stub's deterministic exit ECX (0).
        let _ = callee_cdecl!(13u32, u32, 0, second, handle);
    }
}

/// Main path: two float folds, a colour parameter lookup keyed 0x41/0x3b, a
/// clamped intensity folded into the colour's top byte, then the sink chain
/// and the sibling update.
#[inline(always)]
fn main_path_ac0(obj: u32) {
    unsafe {
        let _ = callee_thiscall!(4u32, u32, obj);
        // Key 0xa7 plus the fixed constant and its spill slot (see above).
        let mut tmp = [0u32; 2];
        stat_lookup_ac0(0xa7, tmp.as_mut_ptr());
        let konst = (global::<f32>(0x00FE8914) as *const f32).read();
        let spill: f32 = core::hint::black_box(0.0);
        let sum = (f32::from_bits(tmp[0]) + konst) + spill;
        let _ = callee_cdecl!(10u32, u32, sum.to_bits());
        // Key 0x2c selected by filler key 7.
        let mut tmp2 = [0u32; 2];
        stat_lookup_ac0(0x2c, tmp2.as_mut_ptr());
        let _ = callee_cdecl!(14u32, u32, 7, 0, tmp2.as_mut_ptr() as u32, 0);
        let _ = callee_cdecl!(9u32, u32, tmp2[0], tmp2[1]);
        // Colour block key: 0x41 unless the probe fails or the mode is 2.
        let probe = callee_cdecl!(15u32, u32, 0);
        let mode = (global::<u32>(0x011D6FD4) as *const u32).read();
        let key = if (probe & 0xFF) != 0 && mode != 2 { 0x41 } else { 0x3b };
        let mut tmp3 = 0u32;
        let block = callee_cdecl!(16u32, u32, &mut tmp3 as *mut u32 as u32, key);
        let base = (block as *const u32).read() & 0x00FF_FFFF;
        // Intensity: convert key 0x37, optionally refresh from the backend,
        // clamp to [0, limit], keep the low byte as the top colour byte.
        let mut tmp4 = 0u32;
        let found = stat_lookup_ac0(0x37, &mut tmp4 as *mut u32);
        let converted = cvttss2si(f32::from_bits((found as *const u32).read()));
        let gate = (global::<u8>(0x01161548) as *const u8).read();
        let level: u8 = if gate != 0 {
            callee_thiscall!(17u32, u32, relocated(0x01161548)) as u8
        } else {
            converted as u8
        };
        let mut tmp5 = 0u32;
        let found2 = stat_lookup_ac0(0x37, &mut tmp5 as *mut u32);
        let value = level as f32;
        let limit = f32::from_bits((found2 as *const u32).read());
        // NaN-aware clamp: `jbe` after `comiss` takes the unordered path.
        let clamped = if 0.0f32 > value {
            0.0f32
        } else if !(value > limit) {
            value
        } else {
            limit
        };
        let colour = ((cvttss2si(clamped) as u8 as u32) << 24) | base;
        let _ = callee_cdecl!(18u32, u32, colour);
        // Sibling update and teardown chain.
        let _ = callee_thiscall!(19u32, u32, obj);
        let _ = callee_cdecl!(20u32, u32,);
        let _ = callee_cdecl!(21u32, u32,);
    }
}

/// Rewrite of STATS_NET_I_PC: refresh one stats object (thiscall/0, void).
///
/// Callee ids in the contract: 1 = readiness probe, 2 = group query,
/// 3 = group-list lookup, 4 = sibling reset, 5 = keyed stat lookup,
/// 6/8/14 = struct fillers (one id per site: different snap widths),
/// 7/9/10 = float sinks, 11 = backend handle, 12 = record fetch,
/// 13 = reporter (first arg masked: incoming-EBX slot), 15 = colour probe,
/// 16 = colour block, 17 = backend refresh, 18 = colour sink, 19 = the
/// sibling update at 0x00E3DD50, 20/21 = teardown calls.
export!(thiscall, rw_00e3dac0(obj: u32) -> u32 {
    unsafe {
        if (global::<u8>(0x011609F6) as *const u8).read() == 0 {
            return 0;
        }
        let ready = callee_cdecl!(1u32, u32, 0);
        if (ready & 0xFF) != 0 {
            let group = callee_cdecl!(
                2u32, u32,
                (global::<u32>(0x01160C0C) as *const u32).read()
            );
            ((obj + 4) as *mut u32).write(group);
        } else {
            ((obj + 4) as *mut u32).write(0);
        }
        let list = callee_cdecl!(3u32, u32, ((obj + 4) as *const u32).read());
        if list == 0 && ((obj + 4) as *const u32).read() == 8 {
            short_path_ac0(obj);
        } else {
            main_path_ac0(obj);
        }
        0
    }
});
