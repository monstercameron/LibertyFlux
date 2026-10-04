// original: 0x008ad390 audio_set_flag_gains
/// Audio flag gains: set two gains to -1.0 or 0.0 from table flag bits.
///
/// Bit 0x80 of the table byte at `+0x15` enables the first gain at `this+4`,
/// bit 0x40 the second at `this+8`. Marks the state valid at `this+0x26` and
/// reports success in the low byte.
export!(thiscall, rw_008ad390(this_: *mut u8) -> u8 {
    unsafe {
        const MUTE: f32 = -1.0; // 0xBF800000
        let table = *(this_ as *const u32) as *const u8;
        let flags = *table.add(0x15);
        *(this_.add(4) as *mut f32) = if (flags & 0x80) != 0 { MUTE } else { 0.0 };
        *(this_.add(8) as *mut f32) = if (flags & 0x40) != 0 { MUTE } else { 0.0 };
        *this_.add(0x26) = 1;
        1
    }
});
