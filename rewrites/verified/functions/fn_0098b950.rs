// original: 0x0098b950 audVehicleAudioEntity::reset_siren_damage
/// Clears the siren damage indicator, sound identifier, and related timer.
///
/// The receiver contains a byte flag, an unaligned 32-bit sound identifier,
/// and an unaligned 16-bit timer. All are set to zero; the method returns
/// integer zero.
const DAMAGE_FLAG: u32 = 0xB2F;
const SOUND_ID: u32 = 0xB19;
const DAMAGE_TIMER: u32 = 0xB1D;

export!(thiscall, rw_0098b950(this_: u32) -> u32 {
    unsafe {
        (this_.wrapping_add(DAMAGE_FLAG) as *mut u8).write(0);
        (this_.wrapping_add(SOUND_ID) as *mut u32).write_unaligned(0);
        (this_.wrapping_add(DAMAGE_TIMER) as *mut u16).write_unaligned(0);
    }
    0
});
