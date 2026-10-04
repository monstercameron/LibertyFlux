// original: 0x00979850 COLLISIONS
/// Audio-collision event setup: validates the two input handles through a
/// chain of subsystem lookups, then either reports a rejected event or builds
/// the parameter block for a new collision sound.
///
/// Returns nothing; all observable effects are the outgoing calls (the hash
/// of a constant name, the parameter-block submission, and the rejection
/// report on the failing path).
export!(thiscall, rw_00979850(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        if arg0 == 0 {
            return 0;
        }
        if *global::<u32>(0x11f7060) == 1 {
            return 0;
        }
        if *global::<u32>(0x12088b4) != *global::<u32>(0xf1c040) {
            return 0;
        }
        if *global::<u32>(0x1037720) == 0x12 {
            return 0;
        }
        // Resolve the collision record for this event.
        let edi = callee_thiscall!(1, u32, this, arg0, arg1);
        if edi == 0 {
            return 0;
        }
        // Kind tag at an unaligned offset; zero means "no record".
        let edi_tag = ((edi.wrapping_add(0x12)) as *const u32).read_unaligned();
        if edi_tag == 0 {
            return 0;
        }
        // Resolve the sounding object.
        let esi = callee_cdecl!(2, u32, arg0);
        if esi == 0 {
            return 0;
        }
        // Scratch parameter area, passed by address to the subsystem calls.
        // The original leaves it zero-filled here; the contract snapshots it.
        let mut scratch = [0u32; 8];
        callee_thiscall!(3, u32, scratch.as_mut_ptr() as u32);
        let _gain = callee_thiscall!(4, u32, this, esi);
        let _tune = *global::<u32>(0x12202b4);
        // Three source coordinates: inline at +0x10, or +0x30 past the link.
        let link = *((esi.wrapping_add(0x20)) as *const u32);
        let src = if link == 0 {
            esi.wrapping_add(0x10)
        } else {
            link.wrapping_add(0x30)
        };
        let f0 = *(src as *const u32);
        let f1 = *(src.wrapping_add(4) as *const u32);
        let f2 = *(src.wrapping_add(8) as *const u32);
        let tok = callee_cdecl!(5, u32,);
        let h3 = callee_cdecl!(6, u32, tok);
        let ok = callee_thiscall!(7, u32, this, edi_tag, scratch.as_mut_ptr() as u32, tok, h3, 0);
        if (ok & 0xff) == 0 {
            // Rejected: report and stop.
            callee_cdecl!(10, u32, tok);
            return 0;
        }
        // Accepted: object class decides whether the id slot is filled.
        let typ = (*((esi.wrapping_add(0x28)) as *const u32) >> 6) & 0xf;
        let id_slot = if typ == 2 || typ == 3 { esi } else { 0 };
        // Loader-adjusted address constant (verified: original side passes img+rva).
        let hash = callee_cdecl!(8, u32, relocated(0xe8bfa4), 0);
        // Parameter block: [hash, -1, 8, f0, f1, f2].
        let params = [hash, 0xffffffffu32, 8, f0, f1, f2];
        callee_cdecl!(
            9, u32, edi_tag, 0, 0, 1,
            scratch.as_mut_ptr() as u32, params.as_ptr() as u32, id_slot, tok
        );
        0
    }
});
