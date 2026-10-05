// original: 0x00B642F0 veh_add_clamp_store_04
/// Add an argument to the u16 at `[this+4]`, clamped into `[0, 0x61A8]`, store back.
///
/// Reads `cur = [this+4]` as a 16-bit word. If `cur` or the argument's low word
/// is at or above 0x61A8 (signed 16-bit comparison), the result is 0x61A8. Else
/// `s = cur + a0` (full 32-bit add); if bit 15 of `s` is set the result is 0,
/// otherwise the low word of `s` capped at 0x61A8. The result is sign-extended
/// (`cwde`, a no-op since it is below 0x8000) into `[this+4]` and returned.
/// Thiscall, one stack word; entry registers except ECX are ignored.
export!(thiscall, rw_00b642f0(this: u32, a0: u32) -> u32 {
    unsafe {
        const LIMIT: i32 = 0x61A8;
        const SLOT: u32 = 4;
        let cur = ((this + SLOT) as *const u16).read_unaligned();
        let v: u32 = if (cur as i16) as i32 >= LIMIT
            || ((a0 & 0xFFFF) as u16 as i16) as i32 >= LIMIT
        {
            LIMIT as u32
        } else {
            let s = (cur as u32).wrapping_add(a0);
            if (((s & 0xFFFF) as u16 as i16) as i32) < 0 {
                0
            } else {
                let w = s & 0xFFFF;
                if ((w as u16 as i16) as i32) > LIMIT { LIMIT as u32 } else { w }
            }
        };
        let out = (((v & 0xFFFF) as u16) as i16) as i32 as u32;
        ((this + SLOT) as *mut u32).write_unaligned(out);
        out
    }
});
