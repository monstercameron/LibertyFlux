// original: 0x00B60BD0 update_player_weapon_fire_audio

use lf_checker_rt::{callee_cdecl, callee_thiscall, relocated};

/// Update the audio position and weapon-fire state for one ped event.
///
/// This thiscall routine takes five stack words: a ped-info pointer, two
/// opaque values forwarded to audio helpers, a pointer to three consecutive
/// position floats, and an unused word. It sends the position to the audio
/// position helper and the audio event helper, using the source object's
/// word at `+0x18` as an event value. If the ped lookup returns the original
/// non-null ped-info pointer, it queries the camera manager and sets byte
/// `+0x16D` on a returned camera object. The camera manager is addressed
/// relative to the relocated image. The routine returns true in AL and pops
/// all five stack arguments.
lf_checker_rt::export!(thiscall, rw_00B60BD0(
    source: u32,
    ped_info: u32,
    related_value: u32,
    source_owner: u32,
    position_address: u32,
    unused: u32
) -> u32 {
    unsafe {
        const SOURCE_EVENT_WORD: u32 = 0x18;
        const POSITION_X: u32 = 0x00;
        const POSITION_Y: u32 = 0x04;
        const POSITION_Z: u32 = 0x08;
        const CAMERA_ACTIVE_FLAG: u32 = 0x16D;
        const CAMERA_FLAG_VALUE: u8 = 1;
        const CAMERA_MANAGER: u32 = 0x0103_E498;
        const FAR_CLIP_BITS: u32 = 0x447A_0000;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_byte(address: u32, value: u8) {
            unsafe { (address as *mut u8).write_unaligned(value) }
        }

        let position = [
            read_word(position_address.wrapping_add(POSITION_X)),
            read_word(position_address.wrapping_add(POSITION_Y)),
            read_word(position_address.wrapping_add(POSITION_Z)),
        ];
        let position_ptr = position.as_ptr() as u32;

        let _ = callee_thiscall!(1, u32, source, ped_info, source_owner, position_ptr);

        let event_word = read_word(source.wrapping_add(SOURCE_EVENT_WORD));
        let _ = callee_cdecl!(
            2,
            u32,
            ped_info,
            event_word,
            source_owner,
            position_ptr,
            FAR_CLIP_BITS
        );

        let _ = callee_thiscall!(3, u32, source, ped_info, related_value, 0, position_address, 0);

        let resolved_ped = callee_cdecl!(4, u32, ped_info);
        if ped_info != 0 && resolved_ped == ped_info {
            let camera = callee_thiscall!(5, u32, relocated(CAMERA_MANAGER));
            if camera != 0 {
                write_byte(camera.wrapping_add(CAMERA_ACTIVE_FLAG), CAMERA_FLAG_VALUE);
            }
        }

        let _ = unused;
        1
    }
});
