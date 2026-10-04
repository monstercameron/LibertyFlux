// original: 0x00dd9340 UIClip::vf97
/// `UIClip::vf97`: re-tint a clip when its identity check passes.
///
/// Runs the `field_1f0` member's slot `0x124` check and requires that same
/// member's slot `0x4c` answer equals the argument, otherwise returns at once.
/// Then toggles `flag_320`, derives a colour word (through a helper call when
/// the flag was clear, else the constant `0xff242424`), stamps it into the
/// member's word `+0x1e0`, runs an engine helper chain, and finishes through
/// one of two member slots depending on the toggled flag. Engine helpers are
/// stubbed by the checker; only their answers and the call shapes matter.
/// Returns only the low byte meaningfully on the earliest exit (the original
/// leaves entry-register residue there), so the contract compares `al`.
export!(thiscall, rw_00dd9340(this_ptr: u32, arg0: u32) -> u32 {
    unsafe {
        let m = ((this_ptr as *const u32).add(0x1f0 / 4)).read();
        let gate_slot = (((m as *const u32).read() + 0x124) as *const u32).read();
        let gate: extern "thiscall" fn(u32) -> u8 =
            core::mem::transmute(gate_slot as usize);
        if gate(m) == 0 {
            return 0;
        }
        let ident_slot = (((m as *const u32).read() + 0x4c) as *const u32).read();
        let ident_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(ident_slot as usize);
        let ident = ident_of(m);
        if ident != arg0 {
            return ident;
        }
        callee_thiscall!(3, u32, relocated(0x1176888), relocated(0xefb8b0));
        let flagp = (this_ptr as *mut u8).add(0x320);
        let was_clear = flagp.read() == 0;
        flagp.write(u8::from(was_clear));
        // The helper takes the address of a frame word plus a constant and
        // answers a pointer to the colour; the constant path uses 0xff242424.
        // The frame address is skipped by the contract on both sides.
        let scratch = arg0;
        let colour = if was_clear {
            let p = callee_cdecl!(4, u32, &scratch as *const u32 as u32, 0x3e);
            (p as *const u32).read()
        } else {
            0xff242424
        };
        ((m as *mut u32).add(0x1e0 / 4)).write(colour);
        let step = callee_cdecl!(
            5,
            u32,
            relocated(0xefb8c8),
            0,
            relocated(0x114e384),
            relocated(0x1057b1c),
            0
        );
        let engine = relocated(0x1981a4c);
        let mid = callee_thiscall!(6, u32, engine, step);
        // Cdecl: no register setup precedes this call, and only zero cleanup
        // keeps the original's stack balanced (verified against its crash).
        let edi = callee_cdecl!(7, u32, mid);
        if edi == 0 {
            return edi;
        }
        if flagp.read() != 0 {
            let tag_slot =
                ((((this_ptr as *const u32).read() + 0x4c)) as *const u32).read();
            let tag_of: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(tag_slot as usize);
            let tag = tag_of(this_ptr);
            callee_thiscall!(8, u32, edi, tag);
            let inner = (this_ptr + 0x1f8) as u32;
            return callee_thiscall!(9, u32, edi, inner);
        }
        let inner = (this_ptr + 0x1f8) as u32;
        callee_thiscall!(10, u32, edi, inner)
    }
});
