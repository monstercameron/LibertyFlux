// original: 0x00E3DD50 stats_group_table_update
// ---------------------------------------------------------------------------
// 0x00E3DD50: large stats-object update. Resolves the object's group table,
// folds float stats through sink stages, packs colour values, and iterates
// an outer counter loop with two bounded inner loops. thiscall/0, void.
// Slot comments name the original's frame words ([E+..]) each local mirrors.
// ---------------------------------------------------------------------------

/// Words filled once before the outer loop (regions 2-4). Iteration-varying
/// words live in explicit persistent locals instead.
#[allow(dead_code)]
struct Setup350 {
    /// Filler words [E+0x24..0x34] (id 10; words 2 and 4 die to overwrites).
    s10w: [u32; 5],
    /// Filler words [E+0x10,0x14,0x18] (id 11, overwrites id 3's word).
    s11w: [u32; 3],
    w44: [u32; 2],
    w58: [u32; 2],
    w6c: [u32; 2],
    w78: [u32; 3],
    w88: u32,
    w50: u32,
    w5c: u32,
}

/// Regions 1-4: group resolution, temp setup, scaling branch, one-shot
/// queries. Returns setup plus table, count, group, s19, s20, s18, w30.
#[inline(always)]
#[allow(clippy::type_complexity)]
fn setup350(obj: u32) -> Option<(Setup350, u32, u32, u32, u32, u32, u32, u32)> {
    unsafe {
        let g4 = ((obj + 4) as *const u32).read();
        let table = callee_cdecl!(1u32, u32, g4);
        if table == 0 {
            return None;
        }
        let count = callee_thiscall!(2u32, u32, obj);
        if (count as i32) <= 0 {
            return None;
        }
        let mut t3 = 0u32;
        let _ = callee_cdecl!(3u32, u32, &mut t3 as *mut u32 as u32);
        let mut t44 = 0u32;
        let _ = callee_cdecl!(4u32, u32, &mut t44 as *mut u32 as u32);
        let mut t5 = [0u32; 2];
        let _ = callee_cdecl!(5u32, u32, t5.as_mut_ptr() as u32, 0x38);
        let mut t6c = 0u32;
        let _ = callee_cdecl!(6u32, u32, &mut t6c as *mut u32 as u32);
        let mut t58 = [0u32; 2];
        let _ = callee_cdecl!(7u32, u32, t58.as_mut_ptr() as u32, 0x30);
        let mut t4c = [0u32; 2];
        let _ = callee_cdecl!(8u32, u32, t4c.as_mut_ptr() as u32);
        let mut t78 = 0u32;
        let _ = callee_cdecl!(9u32, u32, &mut t78 as *mut u32 as u32, 0x35);
        let mut t10w = [0u32; 5];
        let _ = callee_cdecl!(10u32, u32, 1, t10w.as_mut_ptr() as u32, 0, 0);
        let mut t11 = [0u32; 3];
        let _ = callee_cdecl!(11u32, u32, 2, t11.as_mut_ptr() as u32, 0, 0);
        let mut f44 = [0u32; 2];
        let _ = callee_cdecl!(12u32, u32, 3, f44.as_mut_ptr() as u32, 0, 0);
        let mut f6c = [0u32; 2];
        let _ = callee_cdecl!(13u32, u32, 2, 0, f6c.as_mut_ptr() as u32, 0);
        let mut f78 = [0u32; 3];
        let _ = callee_cdecl!(14u32, u32, 2, 0, f78.as_mut_ptr() as u32, 0);
        // Scaling branch.
        let a15 = callee_cdecl!(15u32, u32,) & 0xFF;
        let g884a = (global::<u32>(0x0105C884) as *const u32).read();
        let g888b = (global::<u32>(0x0105C888) as *const u32).read();
        let num = if a15 != 0 { g888b } else { g884a };
        let a16 = callee_cdecl!(16u32, u32,) & 0xFF;
        let g880a = (global::<u32>(0x0105C880) as *const u32).read();
        let g87cb = (global::<u32>(0x0105C87C) as *const u32).read();
        let den = if a16 != 0 { g87cb } else { g880a };
        let ratio = (num as i32) as f32 / (den as i32) as f32;
        let c88e8 = (global::<f32>(0x00FE88E8) as *const f32).read();
        let (w50, w5c) = if c88e8 > ratio {
            let c888c = (global::<f32>(0x00FE888C) as *const f32).read();
            let v5c = f32::from_bits(t58[1]) * ratio;
            let v50 = (c88e8 - ratio * c888c) * f32::from_bits(t4c[1]);
            (v50.to_bits(), v5c.to_bits())
        } else {
            let mut t17 = [t4c[0], t4c[1]];
            let _ = callee_cdecl!(17u32, u32, 2, 0, t17.as_mut_ptr() as u32, 0);
            (t17[1], t58[1])
        };
        let w88 = (f32::from_bits(t11[0]) - f32::from_bits(f78[0])).to_bits();
        let w30 = (f32::from_bits(t11[1]) - f32::from_bits(f78[1])).to_bits();
        // Region 4 head (one-shot queries).
        let s18 = callee_cdecl!(18u32, u32, g4);
        let s19 = callee_thiscall!(19u32, u32, obj);
        let s20 = callee_thiscall!(20u32, u32, obj);
        let _ = t3;
        let _ = t44;
        let _ = t5;
        let _ = t6c;
        let _ = t78;
        Some((
            Setup350 { s10w: t10w, s11w: t11, w44: f44, w58: t58,
                       w6c: f6c, w78: f78, w88, w50, w5c },
            table, count, g4, s19, s20, s18, w30,
        ))
    }
}

/// Region 8: polled selector quad (runs when the flag is set). Returns the
/// colour-base word and the snapped combine word [E+0x8C]; the other three
/// combined words are dead in the original and not reproduced.
#[inline(always)]
fn region8_350(st: &Setup350) -> (u32, u32) {
    unsafe {
        let g880a = (global::<u32>(0x0105C880) as *const u32).read();
        let g87cb = (global::<u32>(0x0105C87C) as *const u32).read();
        let g884a = (global::<u32>(0x0105C884) as *const u32).read();
        let g888b = (global::<u32>(0x0105C888) as *const u32).read();
        let _b = if callee_cdecl!(32u32, u32,) & 0xFF != 0 { g87cb } else { g880a };
        let _d = if callee_cdecl!(33u32, u32,) & 0xFF != 0 { g888b } else { g884a };
        let _s = if callee_cdecl!(34u32, u32,) & 0xFF != 0 { g87cb } else { g880a };
        let c = if callee_cdecl!(35u32, u32,) & 0xFF != 0 { g888b } else { g884a };
        let w8c = ((c as i32) as f32 * f32::from_bits(st.w88)).to_bits();
        let mut t9c = 0u32;
        let found = callee_cdecl!(36u32, u32, &mut t9c as *mut u32 as u32, 0x42);
        ((found as *const u32).read(), w8c)
    }
}

/// Persistent iteration state (frame words the outer loop carries).
struct Iter350 {
    ebx: u32,
    w14: u32,
    w30: u32,
    w34: u32,
    w8c: u32,
    bound3c: u32,
}

/// One outer-loop iteration (regions 4-tail through 13).
#[inline(always)]
fn iter350(
    obj: u32, st: &Setup350, table: u32, g4: u32, n: i32, flag: bool,
    s19: u32, s20: u32, s18: u32, it: &mut Iter350,
) -> bool {
    unsafe {
        // Divisor stepping loop (answer sequence bounds it; terminates).
        let first = callee_cdecl!(21u32, u32, g4, it.ebx, 0) & 0xFF;
        if first == 0 {
            loop {
                let next = (it.ebx.wrapping_add(1) as i32) % (s18 as i32);
                it.ebx = next as u32;
                let more = callee_cdecl!(21u32, u32, g4, it.ebx, 0) & 0xFF;
                if more != 0 {
                    break;
                }
            }
        }
        // Table word, backend formatter, record fetch.
        let entry = ((table + it.ebx.wrapping_mul(8)) as *const u32).read();
        let word = (entry & 0xFFFF) as u16 as i16 as i32;
        let str_addr = if word <= 0x1D8 {
            relocated(0x00F15394)
        } else {
            relocated(0x00F15388)
        };
        let mut tfmt = 0u32;
        let _ = callee_cdecl!(
            22u32, u32, &mut tfmt as *mut u32 as u32, 0x100, str_addr, word as u32
        );
        let mut trec = 0u32;
        let handle = callee_thiscall!(
            23u32, u32, relocated(0x0116BFF0), &mut trec as *mut u32 as u32
        );
        let _ = callee_cdecl!(24u32, u32, handle);
        let _ = callee_cdecl!(25u32, u32, 1);
        // Shared plain-form condition (constant within an iteration).
        let w54 = (entry & 0xFFFF).wrapping_sub(0x1D9);
        let ax = (w54 & 0xFFFF) as u16;
        let tag = (((table + it.ebx.wrapping_mul(8)) as *const u8).add(2)).read();
        let plain = ax > 0xAF || tag == 0x0D;
        let c8734 = (global::<f32>(0x00FE8734) as *const f32).read();
        let w24f = f32::from_bits(st.s10w[0]);
        let w10f = f32::from_bits(st.s11w[0]);
        if !plain {
            let _ = callee_cdecl!(26u32, u32, (w10f + c8734).to_bits(), w24f.to_bits());
        } else {
            let _ = callee_cdecl!(26u32, u32, w10f.to_bits(), w24f.to_bits());
        }
        let s28 = if !plain {
            callee_cdecl!(27u32, u32, (w10f + c8734).to_bits(), w24f.to_bits(), handle, 0)
        } else {
            callee_cdecl!(27u32, u32, w10f.to_bits(), w24f.to_bits(), handle, 0)
        };
        // Wide record, combined floats, new intensity bound.
        let mut wide = [0u32; 2];
        let _ = callee_cdecl!(28u32, u32, wide.as_mut_ptr() as u32, g4, it.ebx);
        let _ = callee_cdecl!(29u32, u32, 2);
        let sum_a = f32::from_bits(st.w6c[0]) + f32::from_bits(st.w44[0]);
        let _ = callee_cdecl!(30u32, u32, st.s10w[1], sum_a.to_bits());
        it.bound3c = s28;
        let sum_b = f32::from_bits(st.w6c[0]) + f32::from_bits(st.w44[0]);
        let mut wide2 = [0u32; 2];
        let s32 = callee_cdecl!(
            31u32, u32, st.s10w[1], sum_b.to_bits(), wide2.as_mut_ptr() as u32, 0
        );
        let maxval = if (s28 as i32) > (s32 as i32) { s28 } else { s32 };
        if flag {
            let (base, w) = region8_350(st);
            it.w34 = base;
            it.w8c = w;
            // Intensity from key 0x36, clamped, packed with the base.
            let mut t37 = 0u32;
            let f37 = callee_cdecl!(37u32, u32, &mut t37 as *mut u32 as u32, 0x36);
            let converted = cvttss2si(f32::from_bits((f37 as *const u32).read()));
            let gate = (global::<u8>(0x01161548) as *const u8).read();
            let level: u8 = if gate != 0 {
                callee_thiscall!(38u32, u32, relocated(0x01161548)) as u8
            } else {
                converted as u8
            };
            let mut t37b = 0u32;
            let f37b = callee_cdecl!(37u32, u32, &mut t37b as *mut u32 as u32, 0x36);
            let limit = f32::from_bits((f37b as *const u32).read());
            let value = level as f32;
            let clamped = if !(0.0f32 <= value) {
                0.0f32
            } else if value > limit {
                limit
            } else {
                value
            };
            let colour = ((cvttss2si(clamped) as u8 as u32) << 24) | (it.w34 & 0x00FF_FFFF);
            it.w34 = colour;
            let mut fslot = it.w8c;
            let _ = callee_cdecl!(
                39u32, u32, &mut fslot as *mut u32 as u32, &mut it.w34 as *mut u32 as u32
            );
            let _ = callee_cdecl!(40u32, u32,);
        }
        // Conditional object writes.
        let gc = ((obj + 0x0C) as *const u32).read();
        let gc_f = f32::from_bits(gc);
        if s19 == it.ebx {
            let sum = gc_f + f32::from_bits(it.w14);
            ((obj + 0x24) as *mut u32).write(sum.to_bits());
        }
        if s20 == it.ebx {
            let sum = (gc_f + f32::from_bits(it.w30))
                + (maxval as i32) as f32 * f32::from_bits(st.w50);
            ((obj + 0x20) as *mut u32).write(sum.to_bits());
        }
        // Small sink, second float pair, then the bounded inner loop.
        let _ = callee_cdecl!(41u32, u32, 1);
        if !plain {
            let _ = callee_cdecl!(
                42u32, u32,
                (f32::from_bits(st.s11w[0]) + c8734).to_bits(),
                w24f.to_bits()
            );
        } else {
            let _ = callee_cdecl!(42u32, u32, st.s11w[0], w24f.to_bits());
        }
        // Threaded float: starts at w14, accumulates w5c after every
        // inner or tail iteration (reloaded fresh for the tail).
        let step = f32::from_bits(st.w5c);
        if (it.bound3c as i32) > 0 {
            let mut x1 = f32::from_bits(it.w14);
            let mut esi: u32 = 0;
            loop {
                let acc = gc_f + x1;
                if !plain {
                    let _ = callee_cdecl!(
                        43u32, u32, (w10f + c8734).to_bits(), acc.to_bits(),
                        handle, esi, esi
                    );
                } else {
                    let _ = callee_cdecl!(
                        43u32, u32, w10f.to_bits(), acc.to_bits(), handle, esi,
                        esi
                    );
                }
                x1 = x1 + step;
                esi = esi.wrapping_add(1);
                if !((esi as i32) < (it.bound3c as i32)) {
                    break;
                }
            }
        }
        // Small sinks, then the tag-0xE fork.
        let _ = callee_cdecl!(44u32, u32, 2);
        let sum_c = f32::from_bits(st.w6c[0]) + f32::from_bits(st.w44[0]);
        let _ = callee_cdecl!(45u32, u32, st.s10w[1], sum_c.to_bits());
        let tag2 = (((table + it.ebx.wrapping_mul(8)) as *const u8).add(2)).read();
        if tag2 == 0x0E {
            let polled: f32 = callee_cdecl!(46u32, f32, word as u32);
            let c8628 = (global::<f32>(0x00FE8628) as *const f32).read();
            // `lahf` bit: set unless the polled value ordered-equals.
            let cl = u32::from(polled != c8628);
            let f0 = f32::from_bits(st.w6c[0]) + f32::from_bits(st.w44[0]);
            let f1 = gc_f + f32::from_bits(it.w14);
            let _ = callee_thiscall!(
                47u32, u32, obj, cl, f0.to_bits(), f1.to_bits()
            );
        } else if (s32 as i32) > 0 {
            let mut x1 = f32::from_bits(it.w14);
            let mut esi: u32 = 0;
            loop {
                let acc = gc_f + x1;
                let mut ttail = 0u32;
                let _ = callee_cdecl!(
                    48u32, u32, 0, acc.to_bits(),
                    &mut ttail as *mut u32 as u32, esi, esi
                );
                x1 = x1 + step;
                esi = esi.wrapping_add(1);
                if !((esi as i32) < (s32 as i32)) {
                    break;
                }
            }
        }
        // Latch: toggle flag, step the divisor, decay the accumulators.
        let x1n = (maxval as i32) as f32 * f32::from_bits(st.w5c);
        it.w14 = (x1n + f32::from_bits(it.w14)).to_bits();
        it.w30 = (x1n + f32::from_bits(it.w30)).to_bits();
        let next = (it.ebx.wrapping_add(1) as i32) % (s18 as i32);
        it.ebx = next as u32;
        let _ = n;
        !flag
    }
}

/// Rewrite of the large stats update at 0x00E3DD50 (thiscall/0, void).
///
/// Callee ids in the contract: 1 = group-table lookup, 2 = count query,
/// 3/4/6/8 = one-word fillers, 5/7/9/37 = keyed lookups, 10-14/17 = struct
/// fillers (one id per site: different slots and widths), 15/16/32-35 = the
/// encrypted poller (one id per site: independent branches), 18 = divisor
/// query, 19/20 = sibling queries, 21 = stepping probe (sequenced answers),
/// 22 = backend formatter (encrypted), 23 = record fetch, 24/25/29/41/44 =
/// small sinks, 26/30/42/45 = float-pair sinks, 27/31 = quad sinks,
/// 28 = wide record, 36 = colour block, 38 = backend refresh, 39 = colour
/// sink (snapped), 40 = teardown, 43/48 = reporters, 46 = polled float
/// (x87 ST0 answer), 47 = combine sink. The cookie check is compiler
/// scaffolding and intentionally not reproduced.
export!(thiscall, rw_00e3dd50(obj: u32) -> u32 {
    unsafe {
        let setup = match setup350(obj) {
            Some(s) => s,
            None => return 0,
        };
        let (st, table, count, g4, s19, s20, s18, w30) = setup;
        let mut it = Iter350 {
            ebx: ((obj + 8) as *const u32).read(),
            w14: st.s11w[1],
            w30,
            w34: 0,
            w8c: 0,
            bound3c: 0,
        };
        // The loop-back lands on the region-4 `je`, so the counter runs
        // count..1: the n=0 iteration never executes.
        let mut n = count as i32;
        let mut flag = true;
        while n >= 1 {
            flag = iter350(obj, &st, table, g4, n, flag, s19, s20, s18, &mut it);
            n -= 1;
        }
        0
    }
});
