// original: 0x008AA610 aud_retire_slot_chain (proposed)

/// Retire every entry of a slot chain: flip each sound's flag bits and
/// move the entries to slot 0, then clear the kind's bitmap bit.
///
/// `this` is the audio manager and `a0` a descriptor whose word at `+4`
/// is the slot kind, or null (which releases the lock token and returns
/// at once). A lock token spans the call (callee id 1 takes it with
/// `this+0x3210`, id 2 releases it). Each u16-linked entry of the kind's
/// chain at `this+0xFA4+kind*8` (next at `+0`, bank bytes at `+2`/`+3`)
/// resolves its sound `P = table1[byte2] + STRIDE1 * byte3` (`table1` =
/// bank offset 0x6F10 in the table at global `AUD_TABLE` = 0x115D988,
/// banks of 0x6F40, stride from `AUD_STRIDE1` = 0x115D964) and, with the
/// sub-index at sound `+4` and byte `+0x40`, the flag word
/// `Q = table2[byte40] + STRIDE2 * sub` (`table2` = bank offset 0x6F14,
/// stride from `AUD_STRIDE2` = 0x115D968): bit 3 is set at `Q+0xE8`,
/// bit 1 cleared, helper 3 (id 3, thiscall on the sound) runs and a
/// non-null answer is zeroed, and bit 4 is set. The entry is then
/// unlinked from its chain and pushed at the head of slot 0's chain.
/// (A sub-index of 0xFF nulls `Q` and faults on the flag write on both
/// sides identically; the contract covers it through fault parity.)
/// Finally bit `kind` is cleared in the bitmap
/// at `this+0x28A0` (word `kind/32`, bit `kind%32).
///
/// The original reuses its incoming `a0` argument slot as scratch for
/// the entry pointer, so this contract runs with the stack check off.
///
/// Original: 0x008AA610 (thiscall, `this` in ECX, one stack word,
/// callee pops 4, no return value).
lf_checker_rt::export!(thiscall, rw_008AA610(this: u32, a0: u32) -> u32 {
    unsafe {
        const AUD_STRIDE1: u32 = 0x115D964;
        const AUD_STRIDE2: u32 = 0x115D968;
        const AUD_TABLE: u32 = 0x115D988;
        const LOCK_ARG: u32 = 0x3210;
        const BITS: u32 = 0x28A0;
        const SLOT_BASE: u32 = 0xFA4;
        const SLOT_STRIDE: u32 = 8;
        const BANK_STRIDE: u32 = 0x6F40;
        const BANK_ENTRY1: u32 = 0x6F10;
        const BANK_ENTRY2: u32 = 0x6F14;
        const FLAG_OFF: u32 = 0xE8;
        const NONE16: u32 = 0xFFFF;
        const NONE8: u8 = 0xFF;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }

        let mut tok: u32 = 0;
        let _: u32 = lf_checker_rt::callee_thiscall!(
            1,
            u32,
            &mut tok as *mut u32 as u32,
            this.wrapping_add(LOCK_ARG)
        );
        if a0 == 0 {
            let _: u32 =
                lf_checker_rt::callee_thiscall!(2, u32, &mut tok as *mut u32 as u32);
            return 0;
        }
        let kind = rd16(a0 + 4);
        let mut cur = rd16(
            this.wrapping_add(SLOT_BASE)
                .wrapping_add((kind as u32).wrapping_mul(SLOT_STRIDE)),
        );
        if cur as u32 != NONE16 {
            let stride1 = lf_checker_rt::global::<u32>(AUD_STRIDE1).read();
            let stride2 = lf_checker_rt::global::<u32>(AUD_STRIDE2).read();
            let base = lf_checker_rt::global::<u32>(AUD_TABLE).read();
            loop {
                let e = this.wrapping_add((cur as u32).wrapping_mul(4));
                let next = rd16(e);
                let b1 = rd8(e + 2);
                let b2 = rd8(e + 3);
                let bank1 = (b1 as u32).wrapping_mul(BANK_STRIDE);
                let snd = rd32(base.wrapping_add(bank1).wrapping_add(BANK_ENTRY1))
                    .wrapping_add(stride1.wrapping_mul(b2 as u32));
                let s4 = rd8(snd + 4);
                let i40 = rd8(snd + 0x40);
                let q = if s4 == NONE8 {
                    0
                } else {
                    let bank2 = (i40 as u32).wrapping_mul(BANK_STRIDE);
                    rd32(base.wrapping_add(bank2).wrapping_add(BANK_ENTRY2))
                        .wrapping_add(stride2.wrapping_mul(s4 as u32))
                };
                let fp = q.wrapping_add(FLAG_OFF) as *mut u8;
                fp.write(fp.read() | 8);
                let fp2 = q.wrapping_add(FLAG_OFF) as *mut u8;
                fp2.write(fp2.read() & 0xFD);
                let r: u32 = lf_checker_rt::callee_thiscall!(3, u32, snd);
                if r != 0 {
                    (r as *mut u32).write_unaligned(0);
                }
                let q3 = if rd8(snd + 4) == NONE8 {
                    0
                } else {
                    let bank2 = (rd8(snd + 0x40) as u32).wrapping_mul(BANK_STRIDE);
                    rd32(base.wrapping_add(bank2).wrapping_add(BANK_ENTRY2))
                        .wrapping_add(stride2.wrapping_mul(rd8(snd + 4) as u32))
                };
                let fp3 = q3.wrapping_add(FLAG_OFF) as *mut u8;
                fp3.write(fp3.read() | 0x10);
                let old0 = rd16(this + SLOT_BASE);
                (e as *mut u16).write_unaligned(old0);
                (this.wrapping_add(SLOT_BASE) as *mut u16).write_unaligned(cur);
                cur = next;
                if cur as u32 == NONE16 {
                    break;
                }
            }
        }
        let bits = rd32(this + BITS);
        let w = (kind as u32) >> 5;
        let b = (kind as u32) & 0x1F;
        let wp = bits.wrapping_add(w.wrapping_mul(4)) as *mut u32;
        wp.write_unaligned(wp.read_unaligned() & !(1u32 << b));
        let _: u32 =
            lf_checker_rt::callee_thiscall!(2, u32, &mut tok as *mut u32 as u32);
        0
    }
});
