// original: 0x0089a130 audio_voice_start (proposed name)
/// Start (or restart) one audio voice, returning nonzero on success.
///
/// `obj` is the voice object, `arg1` an opaque start parameter forwarded to
/// the handlers. The voice's bank/slot bytes select a bank record; a missing
/// record, a failed prepare call, or a failed gate check fails the start.
/// A pending-restart flag runs a prepare/commit pair first, then a record
/// flag decides between a retrigger call and a full stop-plus-start.
///
/// The original pushes one word too many for the retrigger call on the
/// record-flag path (the callee pops a single word), so that path returns
/// with the stack one word lower; the call below passes the same two words
/// so the stack adjustment matches exactly.
export!(thiscall, rw_0089a130(obj: *mut u8, arg1: u32) -> u32 {
    unsafe {
        let gtable = *global::<u32>(0x0115d988);
        let stride = *global::<u32>(0x0115d964);
        // Bank-record lookup: row for the bank byte plus stride times the
        // slot byte, or null when the slot byte is the 0xff sentinel.
        let lookup = |slot: u32, bank: u32| -> u32 {
            if slot == 0xff {
                0
            } else {
                let row = *((gtable as *const u8)
                    .add(bank.wrapping_mul(0x6f40).wrapping_add(0x6f10) as usize)
                    as *const u32);
                stride.wrapping_mul(slot).wrapping_add(row)
            }
        };
        let bank = *obj.add(0x40) as u32;
        let slot0 = *obj.add(0x48) as u32;
        if slot0 == 0xff {
            return 0;
        }
        if lookup(slot0, bank) == 0 {
            return 0;
        }
        if *obj.add(0xd8) != 0 {
            let bit = ((*obj.add(0x39) >> 5) & 1) as u32;
            let idx = *(obj.add(0x3c) as *const i16) as i32 as u32;
            let prepared = callee_cdecl!(1, u32, idx);
            let slot1 = *obj.add(0x48) as u32;
            let target = lookup(slot1, *obj.add(0x40) as u32);
            // The flag bit is stored over the low byte of the caller's saved
            // `this`, so the second word carries the object address with the
            // bit spliced into its low byte.
            let stamped = ((obj as u32) & 0xffffff00) | bit;
            let a2 = callee_thiscall!(2, u32, target, prepared, stamped, 0);
            if a2 != 1 {
                return (a2 == 0) as u32;
            }
            callee_thiscall!(3, u32, obj as u32, arg1);
            *obj.add(0xd8) = 0;
        }
        let slot = *obj.add(0x48) as u32;
        if slot != 0xff {
            let rec = lookup(slot, *obj.add(0x40) as u32);
            if rec != 0 {
                let kind = *((rec as *const u8).add(6) as *const u16);
                if kind == 2 {
                    // Two words pushed, one popped: matches the original's
                    // stack adjustment on this path exactly.
                    let a4 = callee_thiscall!(4, u32, obj as u32, 0, arg1);
                    let a5 = callee_thiscall!(5, u32, a4);
                    if a5 as u8 != 0 {
                        return 1;
                    }
                }
            }
        }
        let c8 = *(obj.add(0xc8) as *const u32);
        if c8 != 0xffffffff {
            let d0 = *(obj.add(0xd0) as *const u32);
            if (d0 as i32) >= (c8 as i32) {
                return 0;
            }
        }
        if *obj.add(0x39) & 8 != 0 {
            return 0;
        }
        let a4b = callee_thiscall!(4, u32, obj as u32, 0);
        if a4b == 0 {
            return 1;
        }
        callee_thiscall!(6, u32, obj as u32, 0);
        callee_thiscall!(
            7,
            u32,
            obj as u32,
            *(obj.add(0xd4) as *const u32),
            0,
            (obj as u32).wrapping_add(0xb0)
        );
        1
    }
});
