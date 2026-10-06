// original: 0x00c8c710 audio_zones_commit (proposed)
///
/// Commits every pending voice row. The SIGNED dword at `this+0x240` is
/// the row count (zero or negative commits nothing and returns 1); row
/// `i` starts at `this + i * 0x30`. Each row is committed through callee
/// 1 (cdecl, nine arguments: 1, 2 when the count is not 1 else 1, the
/// stack argument, the row address, the dwords at row+0x14/+0x1c, the
/// `this` pointer with its low byte replaced by row+0x10's byte, and the
/// dwords at row+0x18, then 0; esi in the original is row+0x14). The
/// callee's low half-word answers the slot: 0xFFFF means unplaced and
/// clears the sticky ok flag, anything else `r` sets slot row 0x16fc990
/// + r * 0x24's bit 0x400 to row+0x20's byte's low bit (reached rows
/// always have a low-three-bits-set first word in a passing trial).
/// Returns the sticky flag: 1 unless some row went unplaced. Thiscall,
/// one stack argument; returns a byte in al.

lf_checker_rt::export!(thiscall, rw_00c8c710(this: u32, arg: u32) -> u8 {
    unsafe {
    #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
    #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        const COUNT_OFF: u32 = 0x240;
        const PITCH: u32 = 0x30;
        const SLOTS: u32 = 0x16fc990;
        const SLOT_PITCH: u32 = 0x24;
        const PLACED_BIT: u32 = 0x400;
        const UNPLACED: u16 = 0xffff;
        let count = rd32(this.wrapping_add(COUNT_OFF)) as i32;
        if count <= 0 {
            return 1;
        }
        let mut ok: u8 = 1;
        let mut i: i32 = 0;
        while i < count {
            let row = this.wrapping_add((i as u32).wrapping_mul(PITCH));
            let esi = row.wrapping_add(0x14);
            let tweak = ((esi.wrapping_sub(4)) as *const u8).read();
            let thismod = (this & 0xffff_ff00) | (tweak as u32);
            let two = if count != 1 { 2u32 } else { 1u32 };
            let ans = lf_checker_rt::callee_cdecl!(
                1, u32, 1, two, arg, row, rd32(esi), rd32(esi.wrapping_add(8)),
                thismod, rd32(esi.wrapping_add(4)), 0
            );
            let r = ans as u16;
            if r == UNPLACED {
                ok = 0;
            } else {
                // (the original sign-extends the half-word answer)
                let idx = r as i16 as i32 as u32;
                let slot = lf_checker_rt::relocated(SLOTS)
                    .wrapping_add(idx.wrapping_mul(SLOT_PITCH));
                if rd32(slot) & 7 != 0 {
                    let bit = ((((esi.wrapping_add(0xc)) as *const u8).read() & 1) as u32) << 10;
                    let w = rd32(slot);
                    wr32(slot, (w & !PLACED_BIT) | bit);
                } else {
                    // The original reads through a null pointer here (its
                    // slot test left ecx at 0); the game never commits such
                    // a row. Hidden behind black_box so the fault is real
                    // on both sides if it ever happens.
                    let z = core::hint::black_box(0u32);
                    core::hint::black_box((z as *const u32).read_unaligned());
                }
            }
            i += 1;
            if i >= rd32(this.wrapping_add(COUNT_OFF)) as i32 {
                break;
            }
        }
        ok
    }
});
