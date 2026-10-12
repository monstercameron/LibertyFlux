// original: 0x00B5E4A0 emit_bullet_impact_audio

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, relocated};

/// Route a bullet impact through the weapon audio path selected by the
/// weapon information record.
///
/// The this pointer supplies the weapon field at `+0x18` and the fallback
/// value at `+0x10`; four stack words provide two forwarded values and a
/// single precision scale. The weapon lookup is a cdecl call with one
/// argument and preserves ECX. When the returned record's word at `+0x0c`
/// equals one, the function calls the collision voice updater with the
/// process audio object in ECX and four stack arguments. Otherwise it calls
/// the impact emitter with the same object and three stack arguments, adding
/// `0x30` to the second forwarded value. Both helper results are returned in
/// EAX. The function uses the thiscall convention and removes four argument
/// words from the caller's stack.
lf_checker_rt::export!(thiscall, rw_00B5E4A0(this: u32, first: u32, offset_value: u32, third: u32, scale_bits: u32) -> u32 {
    unsafe {
        const WEAPON_FIELD: u32 = 0x18;
        const FALLBACK_FIELD: u32 = 0x10;
        const INFO_FLAG: u32 = 0x0C;
        const AUDIO_OBJECT: u32 = 0x0122_02E0;
        const FALLBACK_BIAS: u32 = 0x30;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let weapon_info = callee_cdecl!(1, u32, read_word(this.wrapping_add(WEAPON_FIELD)));
        if read_word(weapon_info.wrapping_add(INFO_FLAG)) == 1 {
            callee_thiscall!(
                2,
                u32,
                relocated(AUDIO_OBJECT),
                third,
                scale_bits,
                first,
                0
            )
        } else {
            callee_thiscall!(
                3,
                u32,
                relocated(AUDIO_OBJECT),
                third,
                offset_value.wrapping_add(FALLBACK_BIAS),
                read_word(this.wrapping_add(FALLBACK_FIELD))
            )
        }
    }
});
