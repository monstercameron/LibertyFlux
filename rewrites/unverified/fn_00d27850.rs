// original: 0x00D27850 targeting_bulk_update
/// Check the three runtime eligibility gates, hash the steam-effect name,
/// open its stream slot, and set the strength parameter from the target object.
/// When the stream slot reports the additional update path, query the first
/// position, update the stream object, and initialize its payload. Then query
/// the final position and submit the replay effect with the original scalar,
/// flags, and output buffer.
///
/// The contract intercepts all eight outgoing calls. It compares their
/// arguments and scripted results, snapshots the four frame-pointer arguments
/// by value, and checks the four seeded global words. Helper bodies and their
/// internal streaming, position, and replay behavior are outside the proof.
lf_checker_rt::export!(thiscall, rw_00d27850(
    this: u32,
    entity: u32,
) -> u32 {
    unsafe {
        const GATE_ACTIVE_VA: u32 = 0x011F_7060;
        const BUILD_TAG_VA: u32 = 0x0120_88B4;
        const BUILD_EXPECTED_VA: u32 = 0x00F1_C040;
        const TYPE_GATE_VA: u32 = 0x0103_7720;
        const STREAM_CONTEXT_VA: u32 = 0x0139_4D60;
        const STEAM_NAME_VA: u32 = 0x00EE_19A8;
        const STRENGTH_NAME_VA: u32 = 0x00EE_19B4;

        if *lf_checker_rt::global::<u32>(GATE_ACTIVE_VA) == 1 {
            return 0;
        }
        let build_tag = *lf_checker_rt::global::<u32>(BUILD_TAG_VA);
        let expected_tag = *lf_checker_rt::global::<u32>(BUILD_EXPECTED_VA);
        if build_tag != expected_tag {
            return 0;
        }
        if *lf_checker_rt::global::<u32>(TYPE_GATE_VA) == 0x12 {
            return 0;
        }

        let steam_hash = lf_checker_rt::callee_cdecl!(
            1,
            u32,
            lf_checker_rt::relocated(STEAM_NAME_VA),
            0,
        );

        // The output byte occupies the byte immediately before the saved hash.
        // The helper writes one word there, so its upper three bytes replace
        // the hash's low three bytes while the fifth byte retains the hash's
        // original high byte.
        let mut flag_and_hash = [0u8; 5];
        flag_and_hash[1..5].copy_from_slice(&steam_hash.to_le_bytes());
        let effect_flag_ptr = flag_and_hash.as_mut_ptr() as u32;
        let effect = lf_checker_rt::callee_thiscall!(
            2,
            u32,
            lf_checker_rt::relocated(STREAM_CONTEXT_VA),
            entity,
            steam_hash,
            effect_flag_ptr,
            0x4000_0000,
            0,
        );
        // The callee writes through effect_flag_ptr. The low byte is the
        // controlling output; reading that byte retains the original test.
        let effect_flag = (effect_flag_ptr as *const u8).read_volatile();
        let final_hash = u32::from_le_bytes([
            flag_and_hash[1],
            flag_and_hash[2],
            flag_and_hash[3],
            flag_and_hash[4],
        ]);
        if effect == 0 {
            return 0;
        }

        let strength_bits = (entity.wrapping_add(0x54) as *const u32).read_unaligned();
        let _ = lf_checker_rt::callee_thiscall!(
            3,
            u32,
            effect,
            lf_checker_rt::relocated(STRENGTH_NAME_VA),
            strength_bits,
        );

        if effect_flag != 0 {
            let mut first_world_output = core::mem::MaybeUninit::<[u32; 3]>::uninit();
            let first_world_ptr = first_world_output.as_mut_ptr() as u32;
            let first_world_result = lf_checker_rt::callee_thiscall!(
                4,
                u32,
                entity,
                first_world_ptr,
            );
            let _ = lf_checker_rt::callee_thiscall!(5, u32, effect, first_world_result);
            let _ = lf_checker_rt::callee_thiscall!(6, u32, effect);
        }

        let mut final_world_output = core::mem::MaybeUninit::<[u32; 3]>::uninit();
        let final_world_ptr = final_world_output.as_mut_ptr() as u32;
        let final_world_result = lf_checker_rt::callee_thiscall!(
            4,
            u32,
            entity,
            final_world_ptr,
        );
        let strength_bits = (entity.wrapping_add(0x54) as *const u32).read_unaligned();
        let mut replay_output = core::mem::MaybeUninit::<u32>::uninit();
        let replay_output_ptr = replay_output.as_mut_ptr() as u32;
        let _ = lf_checker_rt::callee_cdecl!(
            7,
            u32,
            final_hash,
            entity,
            strength_bits,
            0,
            replay_output_ptr,
            0,
            0,
            u32::MAX,
            u32::MAX,
            0,
            0,
        );
        let _ = final_world_result;
        0
    }
});
