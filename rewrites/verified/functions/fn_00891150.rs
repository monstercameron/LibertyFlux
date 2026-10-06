// original: 0x00891150 audsound_notify_then_emit_sample
/// Notifies the slot of a change, then emits a sample into the voice.
///
/// Unless bit 0 of `this+0x39` is set, resolves the slot pointer (null when
/// `[this+4]` is 0xff, else `stride * byte + table[idx * 0x6f40 + 0x6f14]`),
/// and when the voice at `this+0x74` is non-null calls its detach slot
/// (virtual +4, thiscall: 0), feeds the answer to the attach callee
/// (thiscall: slot, answer), then calls the sync callee (thiscall: slot, 1).
/// While bit 7 of `this+0x38` is set it returns here (the sync answer, or
/// entry EAX when the notify was skipped, so the contract pins entry EAX).
/// Otherwise samples the input callee (cdecl: `arg1`), keeps its low word,
/// and unless `arg3` is -1 adds `arg3` to the lookup callee's (thiscall on
/// the global at 0x115d9a0, no stack words) answer as the emit value (-1
/// when skipped); then latches into the resolved slot like the sibling
/// dispatcher (word at +0xe4, flag at +0xe8 from `arg2`'s low bits, bit 0x40
/// at +0xe7, emit value at +0xdc) and returns EAX as the original leaves it
/// (deterministic on every non-fault path; a 0xff slot byte faults
/// identically on both sides).
/// Original: 0x00891150 (thiscall, three stack words: arg1, arg2, arg3).
export!(thiscall, rw_00891150(this: *mut u8, arg1: u32, arg2: u32, arg3: u32) -> u32 {
    unsafe {
        const DETACH: u32 = 1;
        const ATTACH: u32 = 2;
        const SYNC: u32 = 3;
        const SAMPLE: u32 = 4;
        const LOOKUP: u32 = 5;
        const LOOKUP_THIS: u32 = 0x115d9a0;
        const SLOT_BYTE: usize = 4;
        const INDEX: usize = 0x40;
        const VOICE: usize = 0x74;
        const ROW: u32 = 0x6f40;
        const COL: u32 = 0x6f14;
        const EMPTY: u8 = 0xff;
        const STRIDE_G: u32 = 0x115d968;
        const TABLE_G: u32 = 0x115d988;
        let resolve = |this: *mut u8| {
            let b = *this.add(SLOT_BYTE);
            if b == EMPTY {
                0
            } else {
                let stride = *global::<u32>(STRIDE_G);
                let table = *global::<u32>(TABLE_G);
                let idx = *this.add(INDEX) as u32;
                let base = *((table.wrapping_add(idx.wrapping_mul(ROW)).wrapping_add(COL))
                    as *const u32);
                stride.wrapping_mul(b as u32).wrapping_add(base)
            }
        };
        if *this.add(0x39) & 1 == 0 {
            let slot = resolve(this);
            let voice = *(this.add(VOICE) as *const u32);
            if voice != 0 {
                let vt = (voice as *const u32).read_unaligned();
                let target = ((vt.wrapping_add(4)) as *const u32).read_unaligned();
                let detach: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(target as usize);
                let ans = detach(voice, 0);
                let _: u32 = callee_thiscall!(ATTACH, u32, slot, ans);
            }
            let r: u32 = callee_thiscall!(SYNC, u32, slot, 1);
            if *this.add(0x38) & 0x80 != 0 {
                return r;
            }
        } else if *this.add(0x38) & 0x80 != 0 {
            return 0;
        }
        let s: u32 = callee_cdecl!(SAMPLE, u32, arg1);
        let bp = s as u16;
        let mut edi = 0xffff_ffffu32;
        if arg3 != 0xffff_ffff {
            let base: u32 =
                callee_thiscall!(LOOKUP, u32, lf_checker_rt::relocated(LOOKUP_THIS));
            edi = base.wrapping_add(arg3);
        }
        let edx = resolve(this);
        let mut al = (arg2 as u8).wrapping_shl(2);
        al ^= *((edx.wrapping_add(0xe8)) as *const u8);
        ((edx.wrapping_add(0xe4)) as *mut u16).write_unaligned(bp);
        al &= 4;
        let e8 = (edx.wrapping_add(0xe8)) as *mut u8;
        *e8 ^= al;
        *((edx.wrapping_add(0xe7)) as *mut u8) |= 0x40;
        ((edx.wrapping_add(0xdc)) as *mut u32).write_unaligned(edi);
        // EAX as the original leaves it: the table pointer with its low
        // byte replaced by the final flag byte (or the slot byte 0xff with
        // its low byte replaced on the null path, which faults first).
        let table = *global::<u32>(TABLE_G);
        (table & 0xffff_ff00) | al as u32
    }
});
