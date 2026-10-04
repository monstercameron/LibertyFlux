// original: 0x008ad660 audio_copy_voice_params
/// Audio voice parameter copy: fetch four table dwords into the state.
///
/// Copies the table words at `+0x15/0x19/0x1d/0x21` to `this+4/8/0xc/0x10`,
/// marks the state valid and reports success.
export!(thiscall, rw_008ad660(this_: *mut u8) -> u8 {
    unsafe {
        let table = *(this_ as *const u32) as *const u8;
        *(this_.add(0x04) as *mut u32) =
            core::ptr::read_unaligned(table.add(0x15) as *const u32);
        *(this_.add(0x08) as *mut u32) =
            core::ptr::read_unaligned(table.add(0x19) as *const u32);
        *(this_.add(0x0c) as *mut u32) =
            core::ptr::read_unaligned(table.add(0x1d) as *const u32);
        *(this_.add(0x10) as *mut u32) =
            core::ptr::read_unaligned(table.add(0x21) as *const u32);
        *this_.add(0x26) = 1;
        1
    }
});
