// original: 0x0097B670 audio_slot_release_gated
/// Release the owned slot when present, looking the bank up first if live.
///
/// Returns nothing meaningful (exit eax is entry garbage on the early paths,
/// unchecked). thiscall(obj).
export!(thiscall, rw_s103_97b670(obj: *mut u8) -> u32 {
    unsafe {
        let slot = *(((obj as usize) + 0x14) as *const u32);
        if slot != 0 {
            let live = *global::<u32>(0x11F7060) != 1
                && *global::<u32>(0x12088B4) == *global::<u32>(0xF1C040)
                && *global::<u32>(0x1037720) != 0x12;
            if live {
                let param = *(((slot as usize) + 0xA4) as *const u32);
                callee_cdecl!(1, u32, param);
            }
            let slot_again = *(((obj as usize) + 0x14) as *const u32);
            callee_thiscall!(2, u32, slot_again, 0);
        }
        0
    }
});
