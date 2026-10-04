// original: 0x00cfecc0 task_cover_flag_two_phase (proposed)

/// Two-phase cover-flag update: marks the task, clears stale cover-mode
/// bits, and asks the worker to run up to twice.
///
/// `this` (ECX) is the task with a flag byte at `+0x35`; `obj` is the cover
/// record. On entry bit 0x10 of the flag byte is cleared and bit 8 set. The
/// function returns unless the byte at `obj + 0x218` is zero, the byte at
/// `obj + 0x219` is nonzero, and the pointer at `obj + 0xd68` is non-null.
/// Otherwise it clears bits 0x18 of the dword at that pointer (unless the
/// pointer equals `obj + 0xe70`, in which case the word is left alone, and
/// unless the bits read back as exactly 0x18) and runs the worker (thiscall,
/// four stack words: `obj`, first arg, second arg, phase 0). When the flag
/// byte still has bit 8 set, or the pointed-to dword reads 0x18 in bits
/// 0x18, it stops. Else it sets bit 8 again, repeats the bit clear, runs the
/// worker with phase 1, and when bit 8 survived that call it also sets bit
/// 0x10. No return value. The two call sites use separate scripted answers
/// so both phases' flag outcomes are exercised.
///
/// Original: 0x00cfecc0 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00cfecc0(this: u32, obj: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0x35;
        const GATE_ZERO: u32 = 0x218;
        const GATE_NONZERO: u32 = 0x219;
        const SLOT: u32 = 0xD68;
        const SLOT_SELF: u32 = 0xE70;
        const MODE_BITS: u32 = 0x18;
        const PHASE_CALLEE_0: u32 = 1;
        const PHASE_CALLEE_1: u32 = 2;
        let fb = this.wrapping_add(FLAGS) as *mut u8;
        fb.write(fb.read() & !0x10 | 0x08);
        if (obj.wrapping_add(GATE_ZERO) as *const u8).read() != 0 {
            return 0;
        }
        if (obj.wrapping_add(GATE_NONZERO) as *const u8).read() == 0 {
            return 0;
        }
        let slot = (obj.wrapping_add(SLOT) as *const u32).read_unaligned();
        if slot == 0 {
            return 0;
        }
        if slot != obj.wrapping_add(SLOT_SELF) {
            let v = (slot as *const u32).read_unaligned();
            if v & MODE_BITS != MODE_BITS {
                (slot as *mut u32).write_unaligned(v & !MODE_BITS);
            }
        }
        lf_checker_rt::callee_thiscall!(PHASE_CALLEE_0, u32, this, obj, a1, a2, 0);
        if fb.read() & 0x08 != 0 {
            return 0;
        }
        let cur = ((slot as *const u32).read_unaligned()) & MODE_BITS;
        if cur == MODE_BITS {
            return 0;
        }
        fb.write(fb.read() | 0x08);
        let v2 = (slot as *const u32).read_unaligned();
        if v2 & MODE_BITS != MODE_BITS {
            (slot as *mut u32).write_unaligned(v2 & !MODE_BITS);
        }
        lf_checker_rt::callee_thiscall!(PHASE_CALLEE_1, u32, this, obj, a1, a2, 1);
        let done = fb.read();
        if done & 0x08 == 0 {
            return 0;
        }
        fb.write(done | 0x10);
        0
    }
});
