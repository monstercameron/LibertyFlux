// original: 0x008ad770 audio_set_gain_by_mode
/// Audio gain by mode: copy one parameter, set the gain from a mode nibble.
///
/// Copies the table dword at `+0x15` to `this+4`, sets `this+8` to -1.0 when
/// any of bits 0x0c of the table byte at `+5` is set and to 0.0 otherwise,
/// marks the state valid and reports success.
export!(thiscall, rw_008ad770(this_: *mut u8) -> u8 {
    unsafe {
        let table = *(this_ as *const u32) as *const u8;
        *(this_.add(4) as *mut u32) =
            core::ptr::read_unaligned(table.add(0x15) as *const u32);
        *(this_.add(8) as *mut f32) = if (*table.add(5) & 0x0C) != 0 { -1.0 } else { 0.0 };
        *this_.add(0x26) = 1;
        1
    }
});
