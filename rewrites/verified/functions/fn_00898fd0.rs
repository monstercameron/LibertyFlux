// original: 0x00898fd0 audio_voice_update (proposed name)
// Correction (lane a-S23, second round). The second gate byte at ftable+o+0xd was pinned to zero by the
// previous contract, so the path where both gate bytes are nonzero was missing: the original leaves the x87
// control-word residue in eax and returns through 0x89920b without calling the handler (0x899037-0x899055,
// 0x89920b). That path now returns without a call. The residue value itself is not reproduced (it needs inline
// assembly, which is forbidden); the dispatcher that invokes this function discards eax, so the value is not
// observed by any caller (see the dossier in scratch). The band index is widened in the contracts to 0..4.
/// Voice/bank update dispatcher for one audio object.
///
/// `ftable` points at the per-band float/flag table selected by the caller,
/// `obj` at the voice object. A flag-gated pre-call runs first, then a band
/// float is scaled by 1000 and truncated to an integer voice parameter; a
/// gate byte either routes to a table-driven handler (whose answer is the
/// result) or falls through to two rounds of bank-table flag checks and a
/// four-way state switch that ends in handler calls or tail calls.
///
/// Gate set (first gate byte nonzero): when the second gate byte is zero, the table handler is
/// called and its answer returned; when it is nonzero, no call is made and the return value is
/// not compared (see the correction note below).
export!(cdecl, rw_00898fd0(ftable: *const u8, obj: *mut u8) -> u32 {
    unsafe {
        // Row-pointer table base and column stride (both filled in by the
        // game at start-up; fabricated by the checker contract). Read once:
        // no intercepted callee writes globals, so the reloads in the
        // original observe the same values.
        let gtable = *global::<u32>(0x0115d988);
        let stride = *global::<u32>(0x0115d968);
        // Bank-record lookup: row for the bank byte plus stride times the
        // slot byte, or null when the slot byte is the 0xff sentinel.
        let lookup = |slot: u32, bank: u32| -> u32 {
            if slot == 0xff {
                0
            } else {
                let row = *((gtable as *const u8)
                    .add(bank.wrapping_mul(0x6f40).wrapping_add(0x6f14) as usize)
                    as *const u32);
                stride.wrapping_mul(slot).wrapping_add(row)
            }
        };

        if *obj.add(0x38) & 8 != 0 {
            callee_thiscall!(1, u32, obj as u32);
        }
        let band = (*obj.add(0x41) & 0x3f) as u32;
        let o = (band * 3) as usize * 4;
        let scaled = *(ftable.add(o + 4) as *const f32) * *global::<f32>(0x00fe8c58);
        // The original truncates via fistp with the control word forced to
        // truncate mode; `as` casts truncate the same way for finite
        // in-range values, which the contract guarantees with pinned floats.
        let scaled64 = scaled as f64;
        let voice_param = (if scaled64.is_nan() || scaled64 >= 9223372036854775808.0 || scaled64 < -9223372036854775808.0 { i64::MIN } else { scaled64 as i64 }) as i32 as u32;
        if *ftable.add(o + 0xc) != 0 {
            if *ftable.add(o + 0xd) != 0 {
                // Both gate bytes set. The original does not call the handler here: it leaves the x87
                // control-word residue (fnstcw, or 0xc00) in eax and returns through 0x89920b, with no call
                // and no store. The residue is a leftover of the control-word store, not a result, and the
                // only invoker of this function (the dispatcher at 0x88a3e0, loop call at 0x88a4d9) discards
                // eax (it reads eax again only after reloading it at 0x88a440). So this path returns 0
                // without a call; the return register is not compared on this path (contract gateset).
                return 0;
            }
            // Second gate byte zero: call the table handler and return its answer.
            let idx = *obj.add(0x3b) as u32;
            let target = *global::<u32>(0x0115d6b4).add(idx as usize);
            let handler: extern "cdecl" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            return handler(obj as u32, voice_param);
        }

        let slot = *obj.add(4) as u32;
        let bank = *obj.add(0x40) as u32;
        let rec = lookup(slot, bank);
        let tag = *(((rec as usize) + 0xe7) as *const u8) & 7;
        let zone = *global::<u32>(0x0115f80c).add(tag as usize * 2);
        let flag = ((zone << 5).wrapping_add(rec).wrapping_add(0x5f)) as *const u8;
        if *flag & 0x40 != 0 {
            *obj.add(0x38) |= 2;
        }
        if *flag & 0x80 != 0 {
            callee_thiscall!(2, u32, obj as u32);
        }

        let switch = *(obj.add(6) as *const i16) as i32;
        match switch {
            0 => {
                let g = global::<u32>(0x0115dc64);
                *g = (*g).wrapping_add(1);
                let rec2 = lookup(slot, bank);
                // When the probe bit is clear the original returns the
                // bank-scaled row index left in eax by the lookup above.
                let mut last = bank.wrapping_mul(0x6f40);
                if *(((rec2 as usize) + 0xe7) as *const u8) & 0x40 != 0 {
                    last = callee_thiscall!(3, u32, obj as u32);
                    if last != 1 {
                        return last;
                    }
                }
                if *obj.add(0x38) & 2 == 0 {
                    return last;
                }
                callee_thiscall!(4, u32, obj as u32, voice_param)
            }
            1 => {
                let g = global::<u32>(0x0115dc68);
                *g = (*g).wrapping_add(1);
                let a = callee_thiscall!(3, u32, obj as u32);
                // The original walks the answer down with two decs and
                // returns whatever is left, so only 0/1/2 take named paths.
                if a == 0 {
                    let level = *(ftable as *const u32);
                    if level <= *(obj.add(0x80) as *const u32) {
                        return level;
                    }
                    callee_thiscall!(1, u32, obj as u32)
                } else if a == 1 {
                    // Falls through with eax already decremented to 0.
                    if *obj.add(0x38) & 2 == 0 {
                        return 0;
                    }
                    callee_thiscall!(4, u32, obj as u32, voice_param)
                } else if a == 2 {
                    callee_thiscall!(1, u32, obj as u32)
                } else {
                    a.wrapping_sub(2)
                }
            }
            2 => {
                let g = global::<u32>(0x0115dc70);
                *g = (*g).wrapping_add(1);
                if *obj.add(0x38) & 4 == 0 {
                    callee_thiscall!(6, u32, obj as u32, voice_param)
                } else {
                    callee_thiscall!(5, u32, obj as u32)
                }
            }
            3 => {
                let g = global::<u32>(0x0115dc6c);
                *g = (*g).wrapping_add(1);
                let a = callee_thiscall!(7, u32, obj as u32);
                if a as u8 != 0 {
                    return a;
                }
                let vtable = *(obj as *const u32);
                let target = *((vtable as *const u8).add(0x14) as *const u32);
                let finish: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(target as usize);
                finish(obj as u32, 1)
            }
            // Default: the switch value itself is still in eax.
            _ => switch as u32,
        }
    }
});
