// original: 0x00B5FD50 ped_damage_direction_angle

use lf_checker_rt::{callee_cdecl, export, relocated};

/// Write the last damage bone and the signed angle from a character's facing
/// direction to a target point.
///
/// The cdecl arguments are: a character record, a two-float target point, a
/// source entity, a four-byte bone output, and a four-byte angle output.
/// Null character or source pointers return false without touching outputs.
/// Otherwise the function calls the bone helper with the source and the
/// character's 16-bit damage-bone index, computes a bearing with two double
/// precision math calls, subtracts either the source entity's target bearing
/// or its stored heading, then passes that difference to the angle wrapper.
/// The bone and angle outputs are written through their pointers; success is
/// returned in AL. Floating operations retain the source instruction order,
/// and the two double helper calls use their low 64-bit XMM arguments.
lf_checker_rt::export!(cdecl, rw_00B5FD50(character: u32, target_point: u32, source_entity: u32, out_bone: u32, out_angle: u32) -> u32 {
    unsafe {
        use core::arch::x86::{_mm_cvtss_sd, _mm_cvtsd_f64, _mm_set_ss, _mm_setzero_pd};

        const CHAR_BONE_INDEX: u32 = 0x52;
        const CHAR_X: u32 = 0x10;
        const CHAR_Y: u32 = 0x14;
        const SOURCE_TARGET: u32 = 0x20;
        const SOURCE_HEADING: u32 = 0x1C;
        const TARGET_X: u32 = 0x10;
        const TARGET_Y: u32 = 0x14;
        const SIGN_MASK_DATA: u32 = 0x00FE_8FA0;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        #[inline(always)]
        fn sub_f32(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) - core::hint::black_box(right)
        }

        #[inline(always)]
        fn as_f64_sse(value: f32) -> f64 {
            unsafe { _mm_cvtsd_f64(_mm_cvtss_sd(_mm_setzero_pd(), _mm_set_ss(value))) }
        }

        #[inline(always)]
        unsafe fn bearing(x: f32, y: f32) -> f64 {
            let sign_mask = read_word(relocated(SIGN_MASK_DATA));
            let x_bits = x.to_bits() ^ sign_mask;
            let negative_x = f32::from_bits(x_bits);
            let x64 = as_f64_sse(negative_x).to_bits();
            let y64 = as_f64_sse(y).to_bits();
            let result = callee_cdecl!(
                2,
                u64,
                x64 as u32,
                (x64 >> 32) as u32,
                y64 as u32,
                (y64 >> 32) as u32
            );
            f64::from_bits(result)
        }

        #[inline(always)]
        fn round_f64_to_f32(value: f64) -> f32 {
            use core::arch::x86::{_mm_cvtss_f32, _mm_cvtsd_ss, _mm_set_sd, _mm_setzero_ps};
            unsafe { _mm_cvtss_f32(_mm_cvtsd_ss(_mm_setzero_ps(), _mm_set_sd(value))) }
        }

        if character == 0 {
            return 0;
        }
        if source_entity == 0 {
            return 0;
        }

        let bone_index = (character.wrapping_add(CHAR_BONE_INDEX) as *const u16).read_unaligned() as u32;
        let bone = callee_cdecl!(1, u32, source_entity, bone_index);
        write_word(out_bone, bone);

        let target_x = f32::from_bits(read_word(target_point));
        let target_y = f32::from_bits(read_word(target_point.wrapping_add(4)));
        let character_x = f32::from_bits(read_word(character.wrapping_add(CHAR_X)));
        let character_y = f32::from_bits(read_word(character.wrapping_add(CHAR_Y)));
        let delta_x = sub_f32(target_x, character_x);
        let delta_y = sub_f32(target_y, character_y);
        let facing = round_f64_to_f32(bearing(delta_x, delta_y));
        write_word(out_angle, facing.to_bits());

        let target_object = read_word(source_entity.wrapping_add(SOURCE_TARGET));
        let target_facing = if target_object != 0 {
            let x = f32::from_bits(read_word(target_object.wrapping_add(TARGET_X)));
            let y = f32::from_bits(read_word(target_object.wrapping_add(TARGET_Y)));
            round_f64_to_f32(bearing(x, y))
        } else {
            f32::from_bits(read_word(source_entity.wrapping_add(SOURCE_HEADING)))
        };

        let difference = sub_f32(facing, target_facing);
        let normalized: f32 = callee_cdecl!(3, f32, difference.to_bits());
        write_word(out_angle, normalized.to_bits());
        1
    }
});
