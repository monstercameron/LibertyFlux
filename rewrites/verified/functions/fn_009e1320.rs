// original: 0x009e1320 audio_voice_release_and_park
/// Release the held voice (via the helper while its count is
/// positive), clear the voice slots, then park the linked record. Returns the
/// linked record. (thiscall/0)
export!(thiscall, rw_009e1320(this: *mut u8) -> u32 {
    unsafe {
        if *((this.add(0x30)) as *const u32) != 0 {
            if *((this.add(0x38)) as *const i32) > 0 {
                // ECX at this call holds the voice word (observed register
                // state reproduced so the call comparison pins it too).
                let voice = *((this.add(0x30)) as *const u32);
                callee_thiscall!(1, u32, voice, this as u32);
                *(this.add(0x38) as *mut u32) = 0;
            }
            *(this.add(0x30) as *mut u32) = 0;
        }
        let link = *((this.add(0x34)) as *const u32);
        if link != 0 {
            *((link.wrapping_add(0x40)) as *mut u8) = 0xFF;
            *((link.wrapping_add(0x48)) as *mut u32) = 0xFFFF_FFFF;
        }
        link
    }
});
