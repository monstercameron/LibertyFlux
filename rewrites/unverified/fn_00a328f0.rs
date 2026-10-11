// original: 0x00A328F0 entity_dispatch_audio_parts_2

/// Visit the selected model's audio parts and attach qualifying animation
/// effects. `this` is an entity whose 16-bit model id is at byte offset
/// `0x2e`. The id is sign-extended before indexing the model-info pointer
/// table. Models without flag `0x20` are skipped. The signed model count is
/// polled before and after each entry; each entry's virtual slot `+4` must
/// return exactly one, and its slot `+0x1c` result must describe kind 5 before
/// the slot is called again and the effect manager receives the entity and
/// second result. The game uses 32-bit pointers and thiscall conventions.
lf_checker_rt::export!(thiscall, rw_00a328f0(this: u32) -> () {
    const MODEL_ID: u32 = 0x2e;
    const INVALID_MODEL_ID: u16 = u16::MAX;
    const MODEL_INFO_TABLE_VA: u32 = 0x1295_cd8;
    const MODEL_AUDIO_FLAG: u8 = 0x20;
    const MODEL_FLAGS: u32 = 0x5b;
    const ENTRY_VTABLE: u32 = 0;
    const ENTRY_ENABLED_SLOT: u32 = 0x04;
    const ENTRY_AUDIO_SLOT: u32 = 0x1c;
    const EFFECT_KIND: u32 = 0x24;
    const ANIMATION_EFFECT_KIND: u32 = 5;
    const EFFECT_MANAGER_VA: u32 = 0x171d_ee8;
    const MODEL_COUNT: u32 = 1;
    const MODEL_ENTRY: u32 = 2;
    const SPAWN_EFFECT: u32 = 5;
    const ENTRY_ENABLED: u32 = 3;
    const GET_AUDIO_PART: u32 = 4;

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }

    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        let model_id = read_u16(this.wrapping_add(MODEL_ID));
        if model_id == INVALID_MODEL_ID {
            return;
        }

        let signed_model_id = i32::from(model_id as i16);
        let table = lf_checker_rt::relocated(MODEL_INFO_TABLE_VA);
        let model_info_address = table
            .wrapping_add(signed_model_id.wrapping_mul(4) as u32);
        let model_info = read_u32(model_info_address);
        if read_u8(model_info.wrapping_add(MODEL_FLAGS)) & MODEL_AUDIO_FLAG == 0 {
            return;
        }

        let mut entry_index = 0i32;
        let mut model_count =
            lf_checker_rt::callee_thiscall!(MODEL_COUNT, u32, model_info) as i32;
        if model_count <= 0 {
            return;
        }

        loop {
            let entry = lf_checker_rt::callee_thiscall!(
                MODEL_ENTRY,
                u32,
                model_info,
                entry_index as u32,
            );
            let vtable = read_u32(entry.wrapping_add(ENTRY_VTABLE));
            let enabled_address = read_u32(vtable.wrapping_add(ENTRY_ENABLED_SLOT));
            let is_enabled: extern "thiscall" fn(u32) -> u8 =
                core::mem::transmute(enabled_address as usize);
            if is_enabled(entry) == 1 {
                let audio_address = read_u32(vtable.wrapping_add(ENTRY_AUDIO_SLOT));
                let get_audio_part: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(audio_address as usize);
                let first_audio_part = get_audio_part(entry);
                if read_u32(first_audio_part.wrapping_add(EFFECT_KIND))
                    == ANIMATION_EFFECT_KIND
                {
                    let effect = get_audio_part(entry);
                    let manager = lf_checker_rt::relocated(EFFECT_MANAGER_VA);
                    let _ = lf_checker_rt::callee_thiscall!(
                        SPAWN_EFFECT,
                        u32,
                        manager,
                        this,
                        effect,
                    );
                }
            }

            entry_index = entry_index.wrapping_add(1);
            model_count =
                lf_checker_rt::callee_thiscall!(MODEL_COUNT, u32, model_info) as i32;
            if entry_index >= model_count {
                break;
            }
        }
    }
});
