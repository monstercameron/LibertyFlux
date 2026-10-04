// original: 0x00909450 blip_flags_clear
/// Clear flag bits in a blip's flag word at +0x20.
///
/// Uses the record itself when it has own data, else the default blip's
/// record. When any `mask` bit is set in the word, clears all mask bits and
/// returns 1 in AL, else 0; AH keeps the flag word's high byte in both
/// cases.
export!(cdecl, rw_00909450(id: u32, mask: u32) -> u32 {
    unsafe {
        let rec = blip(id);
        let tgt = if *rec.add(0x08) != 0 {
            rec
        } else {
            blip(*global::<u32>(BLIP_DEFAULT))
        };
        let cell = tgt.add(0x20) as *mut u16;
        let flags = *cell;
        // The original tests with setne into AL only, so the return keeps
        // the flag word's high byte in AH.
        let hit = if (mask & flags as u32) != 0 {
            *cell = flags & !(mask as u16);
            1
        } else {
            0
        };
        (flags as u32 & 0xFF00) | hit
    }
});
