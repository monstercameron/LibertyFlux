// original: 0x0097b390 audio_voice_dispatch
/// Dispatch a voice request by selector, or snapshot the live voice.
///
/// When the voice bank is banked (flag bit 2 at +0xf4) the request is
/// forwarded with the code for `sel` (0 selects 0x1a4, 2 selects 0x4c3,
/// 3 selects 0x4d0, 4 selects 0x4b5, anything else 0x1a9). Otherwise the
/// 16-byte live record is copied to `out`.
export!(thiscall, rw_0097b390(this: *const u8, sel: u32, out: *mut u8) -> u32 {
    unsafe {
        let inner = *((this.add(0x120)) as *const u32);
        if *((inner as *const u8).add(0xf4)) & 4 != 0 {
            let code = match sel {
                0 => 0x1a4,
                2 => 0x4c3,
                3 => 0x4d0,
                4 => 0x4b5,
                _ => 0x1a9,
            };
            callee_thiscall!(1, u32, inner, out as u32, code);
        } else {
            let rec = *(((inner as *const u8).add(0x20)) as *const u32) as *const u8;
            *((out.add(0)) as *mut u32) = *((rec.add(0x30)) as *const u32);
            *((out.add(4)) as *mut u32) = *((rec.add(0x34)) as *const u32);
            *((out.add(8)) as *mut u32) = *((rec.add(0x38)) as *const u32);
            *((out.add(0xc)) as *mut u32) = *((rec.add(0x3c)) as *const u32);
        }
        0
    }
});
