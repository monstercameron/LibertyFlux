// original: 0x009799D0 audio_voice_start_gated
/// Start a voice for a parameter block when the audio gates allow it.
///
/// Returns nothing meaningful (exit eax is entry garbage on the early paths,
/// so the return channel is unchecked). thiscall(this, params).
export!(thiscall, rw_s103_9799d0(this: *mut u8, arg: *const u8) -> u32 {
    unsafe {
        let live = *global::<u32>(0x11F7060) != 1
            && *global::<u32>(0x12088B4) == *global::<u32>(0xF1C040)
            && *global::<u32>(0x1037720) != 0x12;
        if live {
            let key = *(arg as *const u32);
            let pass = key == 0 || callee_cdecl!(1, u32, key) != 0;
            if pass {
                let h = callee_thiscall!(2, u32, this as usize as u32);
                if h != 0 {
                    callee_thiscall!(3, u32, this as usize as u32, h, arg as usize as u32);
                    *(((h as usize) + 0x40) as *mut u32) = 0x40400000;
                    callee_thiscall!(4, u32, this as usize as u32, h, 0);
                    *(((h as usize) + 0x60) as *mut u32) = 1;
                }
            }
        }
        0
    }
});
