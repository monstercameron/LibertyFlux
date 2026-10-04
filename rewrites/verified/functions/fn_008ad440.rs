// original: 0x008ad440 audio_set_gains_copy_param
/// Audio flag gains plus parameter copy: the twin that also takes a dword.
///
/// Same flag-driven gains as the sibling, then copies the table dword at
/// `+0x16` to `this+0x0c`, marks the state valid and reports success.
export!(thiscall, rw_008ad440(this_: *mut u8) -> u8 {
    unsafe {
        const MUTE: f32 = -1.0; // 0xBF800000
        let table = *(this_ as *const u32) as *const u8;
        let flags = *table.add(0x15);
        *(this_.add(4) as *mut f32) = if (flags & 0x80) != 0 { MUTE } else { 0.0 };
        *(this_.add(8) as *mut f32) = if (flags & 0x40) != 0 { MUTE } else { 0.0 };
        *(this_.add(0x0c) as *mut u32) =
            core::ptr::read_unaligned(table.add(0x16) as *const u32);
        *this_.add(0x26) = 1;
        1
    }
});
