// original: 0x00c2d7a0 audfire_param_dispatch (proposed name)
// Validates a fire-audio binding and dispatches a ranged parameter update.
//
// The binding at `arg` is accepted only when the emitter flag bit 18 is set,
// the validator call answers nonzero, the channel byte is positive, the
// outer/inner object chain is complete, and the emitter kind is 0x27 or 0x28.
// On acceptance it stamps a flag word into the channel table, resolves a
// worker entry through two more calls, and invokes the worker's slot with a
// pair of range bounds derived from the emitter's stored limits widened by a
// small kind-specific margin (0.05 for kind 0x27, 0.06 for 0x28): when the
// high limit exceeds the low limit the widened low bound leads, otherwise the
// narrowed low bound trails. Returns nothing meaningful.
export!(thiscall, rw_00c2d7a0(this_ptr: u32, arg: u32) -> u32 {
    unsafe {
        if (lu(this_ptr + 0x10) >> 18) & 1 == 0 {
            return 0;
        }
        let valid = callee_thiscall!(1, u32, this_ptr, arg);
        if valid & 0xff == 0 {
            return 0;
        }
        let channel = load_i8(this_ptr + 5);
        let outer = lu(arg + 0xdc4);
        if channel <= 0 {
            return 0;
        }
        if outer == 0 {
            return 0;
        }
        let inner = lu(outer + 0x64);
        if inner == 0 {
            return 0;
        }
        if inner.wrapping_add(0x100) == 0 {
            return 0;
        }
        if lu(inner + 0x1b0) == 0 {
            return 0;
        }
        callee_thiscall!(2, u32, outer, channel as i32 as u32);
        let row = lu(lu(outer + 4) + 0x0c);
        let table = lu(row + 0x94);
        if table != 0 {
            ((table + (channel as u32) * 8 + 4) as *mut u32).write(0x3fe);
        }
        let kind = lu(this_ptr);
        if kind != 0x27 && kind != 0x28 {
            return 0;
        }
        let inner2 = lu(outer + 0x64);
        let shifted = if inner2 == 0 {
            0
        } else {
            inner2.wrapping_add(0x100)
        };
        if shifted == 0 {
            return 0;
        }
        let worker = lu(shifted + 0x7c);
        if worker == 0 {
            return 0;
        }
        let channel2 = load_i8(this_ptr + 5);
        if channel2 <= 0 {
            return 0;
        }
        let index = callee_thiscall!(3, u32, worker, channel2 as i32 as u32) as i32;
        if index <= 0 {
            return 0;
        }
        let entries = lu(shifted + 0x74);
        let entry = lu(entries + (index as u32) * 4 + 0x80);
        // Stack args are last-pushed-first: the entry word pushed last is arg0.
        let handle = callee_cdecl!(
            4,
            u32,
            entry,
            0,
            relocated(0x0114f848),
            relocated(0x0114f804),
            0
        );
        if handle == 0 {
            return 0;
        }
        let kind2 = lu(this_ptr);
        let hi = load_f(this_ptr + 8);
        let lo = load_f(this_ptr + 0x0c);
        let slot: u32 = lu(lu(handle) + 0x3c);
        let fire: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        if kind2 == 0x27 {
            let margin = load_f(relocated(0x00fe876c));
            if hi > lo {
                fire(handle, (lo + margin).to_bits(), hi.to_bits());
            } else {
                fire(handle, hi.to_bits(), (lo - margin).to_bits());
            }
        } else if kind2 == 0x28 {
            let margin = load_f(relocated(0x00fe8774));
            if hi > lo {
                fire(handle, (lo + margin).to_bits(), hi.to_bits());
            } else {
                fire(handle, hi.to_bits(), (lo - margin).to_bits());
            }
        }
        0
    }
});
