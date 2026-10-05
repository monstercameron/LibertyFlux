// original: 0x008AA230 aud_toggle_slot_flag (proposed)

/// Tear down or set up a sound's slot flag, selected by the low byte of
/// the argument.
///
/// `this` is a sound, `a0` a mode word. A zero low byte selects the
/// clear path, anything else the set path. On the clear path with a
/// sub-index other than 0xFF at sound `+4` and a non-null slot pointer
/// `P = table2[byte40] + STRIDE2 * sub` (`table2` = bank offset 0x6F14
/// in the table at global `AUD_TABLE` = 0x115D988, banks of 0x6F40,
/// stride from `AUD_STRIDE2` = 0x115D968, bytes from sound `+0x40` and
/// `+4`), helper 1 (callee id 1, thiscall on `P`, no arguments) runs,
/// and a nonzero answer runs helper 2 (id 2, thiscall on `P` with 0).
///
/// Then a marker byte other than 0xFF at sound `+5` runs helper 3
/// (id 3, thiscall on the sound). On the clear path a nonzero low byte
/// from it runs helper 6 (id 6); on the set path a zero low byte runs
/// helper 4 (id 4) instead. The set path also runs helper 5 (id 5) for
/// a sub-index of 0xFF. A sub-index of 0xFF then returns; otherwise the
/// set path sets bit 0x40 at `P + 0xE8` and the clear path clears it.
/// (Two null-pointer fault branches exist in the original where the
/// sub-index was just proven non-0xFF; they are mirrored but dead.)
///
/// Original: 0x008AA230 (thiscall, `this` in ECX, one stack word,
/// callee pops 4, no return value).
lf_checker_rt::export!(thiscall, rw_008AA230(this: u32, a0: u32) -> u32 {
    unsafe {
        const AUD_STRIDE2: u32 = 0x115D968;
        const AUD_TABLE: u32 = 0x115D988;
        const BANK_STRIDE: u32 = 0x6F40;
        const BANK_ENTRY2: u32 = 0x6F14;
        const FLAG_OFF: u32 = 0xE8;
        const FLAG_BIT: u8 = 0x40;
        const NONE8: u8 = 0xFF;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn slot_ptr(snd: u32, sub: u8) -> u32 {
            unsafe {
                let base = lf_checker_rt::global::<u32>(AUD_TABLE).read();
                let stride2 = lf_checker_rt::global::<u32>(AUD_STRIDE2).read();
                let bank = (rd8(snd + 0x40) as u32).wrapping_mul(BANK_STRIDE);
                rd32(base.wrapping_add(bank).wrapping_add(BANK_ENTRY2))
                    .wrapping_add(stride2.wrapping_mul(sub as u32))
            }
        }

        let set_mode = (a0 as u8) != 0;
        if !set_mode {
            let sub = rd8(this + 4);
            if sub != NONE8 {
                let p = slot_ptr(this, sub);
                if p != 0 {
                    let r1: u32 = lf_checker_rt::callee_thiscall!(1, u32, p);
                    if r1 != 0 {
                        let p2 = slot_ptr(this, rd8(this + 4));
                        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, p2, 0);
                    }
                }
            }
        }
        if rd8(this + 5) != NONE8 {
            let r3: u32 = lf_checker_rt::callee_thiscall!(3, u32, this);
            if !set_mode {
                if r3 & 0xFF != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(6, u32, this);
                }
            } else if r3 & 0xFF == 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, this);
            }
        }
        if set_mode {
            if rd8(this + 4) == NONE8 {
                let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, this);
            }
            if rd8(this + 4) == NONE8 {
                return 0;
            }
            let sub = rd8(this + 4);
            if sub == NONE8 {
                // Dead: sub proven non-0xFF above; the original faults
                // on a null write here.
                let fp = FLAG_OFF as *mut u8;
                fp.write(fp.read() | FLAG_BIT);
                return 0;
            }
            let fp = slot_ptr(this, sub).wrapping_add(FLAG_OFF) as *mut u8;
            fp.write(fp.read() | FLAG_BIT);
            return 0;
        }
        if rd8(this + 4) == NONE8 {
            return 0;
        }
        let sub = rd8(this + 4);
        if sub == NONE8 {
            // Dead: see above; the original clears the bit at null.
            let fp = FLAG_OFF as *mut u8;
            fp.write(fp.read() & !FLAG_BIT);
            return 0;
        }
        let fp = slot_ptr(this, sub).wrapping_add(FLAG_OFF) as *mut u8;
        fp.write(fp.read() & !FLAG_BIT);
        0
    }
});
