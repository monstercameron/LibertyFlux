// original: 0x0089ab30 audio_voice_route (proposed name)
/// Route one audio voice to its output, returning a status code.
///
/// `obj` is the voice object; `arg1`/`arg2` are opaque route parameters.
/// When the voice's bank record carries no output id the route and commit
/// calls run and the commit answer is the result (2 when the record itself
/// is missing). Otherwise, unless the low byte of `arg2` aborts the route,
/// an optional stop call runs and the stored output id is installed through
/// the final call, returning 0.
export!(thiscall, rw_0089ab30(obj: *mut u8, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        let gtable = *global::<u32>(0x0115d988);
        let stride14 = *global::<u32>(0x0115d968);
        let stride10 = *global::<u32>(0x0115d964);
        let bank = *obj.add(0x40) as u32;
        let row14 = *((gtable as *const u8)
            .add(bank.wrapping_mul(0x6f40).wrapping_add(0x6f14) as usize)
            as *const u32);
        let rec14 = stride14
            .wrapping_mul(*obj.add(0xf7) as u32)
            .wrapping_add(row14);
        let row10 = *((gtable as *const u8)
            .add(bank.wrapping_mul(0x6f40).wrapping_add(0x6f10) as usize)
            as *const u32);
        let lookup10 = |slot: u32| -> u32 {
            if slot == 0xff {
                0
            } else {
                stride10.wrapping_mul(slot).wrapping_add(row10)
            }
        };
        if *((rec14 as *const u8).add(0x98) as *const u32) == 0xffffffff {
            let slot = *obj.add(0x48) as u32;
            if slot == 0xff {
                return 2;
            }
            let rec10 = lookup10(slot);
            if rec10 == 0 {
                return 2;
            }
            callee_thiscall!(1, u32, rec10, *(obj.add(0x54) as *const u32), 0);
            // The original re-reads the bank bytes for this lookup; nothing
            // between the reads can change them, so it is the same record.
            // The flag bit is stored over the low byte of the caller's saved
            // `this`, so the middle word carries the object address with the
            // bit spliced into its low byte.
            let bit = ((*obj.add(0x39) >> 5) & 1) as u32;
            let stamped = ((obj as u32) & 0xffffff00) | bit;
            callee_thiscall!(2, u32, rec10, arg1, stamped, arg2)
        } else {
            if (arg2 & 0xff) != 0 {
                return 0;
            }
            let slot = *obj.add(0x48) as u32;
            if slot != 0xff && lookup10(slot) != 0 {
                callee_thiscall!(3, u32, obj as u32, 0);
            }
            callee_thiscall!(
                4,
                u32,
                obj as u32,
                *((rec14 as *const u8).add(0x98) as *const u32),
                0,
                (rec14 as u32).wrapping_add(0x78)
            );
            0
        }
    }
});
