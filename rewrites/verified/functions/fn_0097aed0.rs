// original: 0x0097AED0 audGtaAudioEntity::audGtaAudioEntity_2
/// Tear down the audio entity's owned slots, then chain to the base teardown.
///
/// Releases the three owned references (the first only when the audio gates
/// allow the bank lookup), stamps the intermediate and final type tags, and
/// tail-calls the shared base teardown. thiscall(obj).
export!(thiscall, rw_s103_97aed0(obj: *mut u8) -> u32 {
    unsafe {
        let rd = |p: *const u8, off: usize| *(((p as usize) + off) as *const u32);
        let wr = |p: *mut u8, off: usize, v: u32| *(((p as usize) + off) as *mut u32) = v;
        let slot = rd(obj, 0x0C);
        wr(obj, 0, relocated(0xE8D6F4));
        if slot != 0 {
            let live = *global::<u32>(0x11F7060) != 1
                && *global::<u32>(0x12088B4) == *global::<u32>(0xF1C040)
                && *global::<u32>(0x1037720) != 0x12;
            if live {
                callee_cdecl!(1, u32, rd(slot as usize as *const u8, 0xA4));
            }
            callee_thiscall!(2, u32, rd(obj, 0x0C), 0);
            wr(obj, 0x0C, 0);
        }
        let slot_b = rd(obj, 0xB4);
        if slot_b != 0 {
            callee_thiscall!(3, u32, slot_b, (obj as usize as u32).wrapping_add(0xB4));
        }
        wr(obj, 0xB4, 0);
        let slot_c = rd(obj, 8);
        if slot_c != 0 {
            callee_thiscall!(4, u32, slot_c, 0);
        }
        wr(obj, 0, relocated(0xE83134));
        callee_thiscall!(5, u32, obj as usize as u32)
    }
});
