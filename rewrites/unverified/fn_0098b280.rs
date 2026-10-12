// original: 0x0098b280 audVehicleAudioEntity::heli_damage_voices_active
/// Reports whether any of seven vehicle voice handles is active.
///
/// Each handle is a 32-bit word in the receiver. The method tests the full
/// word at offsets `0xA88`, `0xA8C`, `0xA94`, `0xA90`, `0xA98`, `0xA9C`, and
/// `0xAA0`, returning a zero-extended byte boolean for the first nonzero one.
const VOICE_HANDLE_OFFSETS: [u32; 7] = [
    0xA88, 0xA8C, 0xA94, 0xA90, 0xA98, 0xA9C, 0xAA0,
];

export!(thiscall, rw_0098b280(this_: u32) -> u8 {
    for offset in VOICE_HANDLE_OFFSETS {
        let handle = unsafe {
            (this_.wrapping_add(offset) as *const u32).read()
        };
        if handle != 0 {
            return 1;
        }
    }
    0
});
