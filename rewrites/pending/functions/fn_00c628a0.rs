// original: 0x00c628a0 anim_player_gate_check
/// Gate check: whether the player takes the active branch.
///
/// Combines flag bits of the word at `+0x4` with the bound animation's marker
/// byte and one registry query over the low nibble (a null tag faults on the
/// marker read). Returns 1 when any acceptance bit is set or the query accepts,
/// else 0.
export!(thiscall, rw_00c628a0(this: u32) -> u32 {
    unsafe {
        let flags = *((this + 4) as *const u32);
        if flags & 0x0020_0000 == 0 {
            let tag = *((this + 0x44) as *const u16);
            let mark = if tag == 1 {
                let anim = *((this + 0x40) as *const u32);
                *((anim + 6) as *const u8)
            } else {
                core::ptr::read_volatile(6 as *const u8)
            };
            if mark & 0x10 != 0 && flags & 0x0002_0000 == 0 {
                let low = flags & 0xF;
                if low == 0 {
                    return 1;
                }
                let answer: u32 = callee_cdecl!(0, u32, low, 0);
                if answer & 0xFF != 0 {
                    return 1;
                }
            }
        }
        if flags & 0x0000_1E00 != 0 { 1 } else { 0 }
    }
});
