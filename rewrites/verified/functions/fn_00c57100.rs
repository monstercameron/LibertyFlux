// original: 0x00c57100 task_complex_gate_check (proposed)

/// Decide whether a gated complex task may proceed, returning 0 or 1.
///
/// The object is rejected at once when the flag byte at `+0x210` is set with
/// mode 1 or 2 at `+0xa74`, or when its link at `+0x224` is null. Otherwise a
/// gate bit is formed: 1 unless the byte at `+0xa60` is 2 while the owner's
/// byte at `+0x18` is clear, in which case callee 1 is consulted and the bit
/// comes from the owner's bytes at `+0x19`/`+0x1a` (a clear bit returns 0
/// here). The link's virtual slot at `+0x1c` (callee 2, planted) yields a
/// base that callee 3 resolves; a null resolution or a type answer other than
/// 0x24 from virtual slot `+4` (callee 4, planted) returns the gate bit.
/// Then the link word at `+0xe8` must be nonzero, callee 5 must answer
/// nonzero for (`obj`, 0x6a4), and the final result is the gate bit exactly
/// when callee 6 answers nonzero. Stale upper return bytes on early exits are
/// reproduced from the scripted answers (the contract pins entry `eax` to 0
/// for the filter path).
///
/// Original: 0x00c57100 (thiscall: `this` in ecx, one stack word).
lf_checker_rt::export!(thiscall, rw_00c57100(this: u32, obj: u32) -> u32 {
    unsafe {
        const GATE: u32 = 1;
        const RESOLVE: u32 = 3;
        const VALIDATE: u32 = 5;
        const CONFIRM: u32 = 6;
        const WANT_TYPE: u32 = 0x24;
        const MODE: u32 = 0x6a4;
        if ((obj + 0x210) as *const u8).read() != 0 {
            let mode = ((obj + 0xa74) as *const u32).read_unaligned();
            if mode == 1 || mode == 2 {
                return 0;
            }
        }
        if ((obj + 0x224) as *const u32).read_unaligned() == 0 {
            return 0;
        }
        let gate: u32;
        if ((obj + 0xa60) as *const u8).read() == 2
            && ((this + 0x18) as *const u8).read() == 0
        {
            let mut bit: u32 = 0;
            if ((obj + 0x26c) as *const u8).read() & 4 != 0 {
                bit = u32::from(((this + 0x19) as *const u8).read() != 0);
            }
            let r1: u32 = lf_checker_rt::callee_cdecl!(GATE, u32, obj);
            if r1 != 0 {
                if ((this + 0x1a) as *const u8).read() != 0 {
                    bit = 1;
                } else if bit == 0 {
                    return r1 & 0xffff_ff00;
                }
            } else if bit == 0 {
                return r1 & 0xffff_ff00;
            }
            gate = bit;
        } else {
            gate = 1;
        }
        let inner = ((obj + 0x224) as *const u32).read_unaligned();
        let vt = (inner as *const u32).read_unaligned();
        let tgt = ((vt + 0x1c) as *const u32).read_unaligned();
        let f2: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(tgt as usize);
        let base = f2(inner);
        let held: u32 = lf_checker_rt::callee_thiscall!(RESOLVE, u32, base.wrapping_add(0x20));
        if held == 0 {
            return gate;
        }
        let vt4 = (held as *const u32).read_unaligned();
        let tgt4 = ((vt4 + 4) as *const u32).read_unaligned();
        let f4: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(tgt4 as usize);
        let ty = f4(held);
        if ty != WANT_TYPE {
            return (ty & 0xffff_ff00) | gate;
        }
        let inner2 = ((obj + 0x224) as *const u32).read_unaligned();
        if ((inner2 + 0xe8) as *const u32).read_unaligned() == 0 {
            // eax was reloaded with the link just above, not the type answer.
            return inner2 & 0xffff_ff00;
        }
        let r5: u32 = lf_checker_rt::callee_thiscall!(VALIDATE, u32, held, obj, MODE);
        if (r5 & 0xff) == 0 {
            return r5 & 0xffff_ff00;
        }
        let r6: u32 = lf_checker_rt::callee_thiscall!(CONFIRM, u32, inner2);
        (r6 & 0xffff_ff00) | u32::from((r6 & 0xff) != 0) & gate
    }
});

