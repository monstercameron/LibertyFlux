// original: 0x0098CD60 audVehicleAudioEntity::bind_vehicle

/// Stores the vehicle configuration pointer and initializes three shaper
/// selections. Configuration mode one uses fixed selections; other modes
/// draw two distinct selections and a third selection from read-only tables.
/// It also stores one scripted random range result, sets the model threshold
/// byte from the 16-bit model id, and returns the slot-acquisition helper's
/// result. The model id is converted to f32 only after zero extension, so all
/// possible 16-bit values are exactly representable.
lf_checker_rt::export!(thiscall, rw_audvehicleaudioentity_bind_vehicle(
    entity: u32,
    vehicle_config: u32
) -> u32 {
    const CONFIG_POINTER: u32 = 0x820;
    const VEHICLE_HANDLE: u32 = 0x814;
    const SHAPER_A: u32 = 0x8A0;
    const SHAPER_B: u32 = 0x8A4;
    const SHAPER_C: u32 = 0x8A8;
    const RANDOM_VALUE: u32 = 0xA84;
    const INITIALIZED_FLAG: u32 = 0xB26;
    const MODEL_THRESHOLD_FLAG: u32 = 0xB2C;
    const CONFIG_MODE: u32 = 0x1304;
    const MODEL_ID: u32 = 0x2C;
    const MODE_WITH_FIXED_SHAPERS: u32 = 1;
    const SHAPER_TABLE: u32 = 0x00E8_FEC0;
    const FINAL_SHAPER_TABLE: u32 = 0x00E8_FECC;
    const MODEL_THRESHOLD: f32 = 22_936.9;

    unsafe {
        (entity.wrapping_add(CONFIG_POINTER) as *mut u32).write_unaligned(vehicle_config);

        if (vehicle_config.wrapping_add(CONFIG_MODE) as *const u32).read_unaligned()
            == MODE_WITH_FIXED_SHAPERS
        {
            (entity.wrapping_add(SHAPER_A) as *mut u32).write_unaligned(5);
            (entity.wrapping_add(SHAPER_B) as *mut u32).write_unaligned(u32::MAX);
            (entity.wrapping_add(SHAPER_C) as *mut u32).write_unaligned(2);
        } else {
            let random_first: unsafe extern "cdecl" fn(i32, i32) -> u32 =
                core::mem::transmute(lf_checker_rt::callee_addr(1));
            let random_second: unsafe extern "cdecl" fn(i32, i32) -> u32 =
                core::mem::transmute(lf_checker_rt::callee_addr(2));
            let shaper_table = lf_checker_rt::global::<u32>(SHAPER_TABLE);

            let first_index = random_first(0, 2) as usize;
            let first_shaper = shaper_table.add(first_index).read_unaligned();
            (entity.wrapping_add(SHAPER_A) as *mut u32).write_unaligned(first_shaper);
            loop {
                let second_index = random_second(0, 2) as usize;
                let second_shaper = shaper_table.add(second_index).read_unaligned();
                (entity.wrapping_add(SHAPER_B) as *mut u32).write_unaligned(second_shaper);
                if first_shaper == second_shaper {
                    break;
                }
            }

            let random_third: unsafe extern "cdecl" fn(i32, i32) -> u32 =
                core::mem::transmute(lf_checker_rt::callee_addr(3));
            let third_index = random_third(0, 2) as usize;
            let third_shaper = lf_checker_rt::global::<u32>(FINAL_SHAPER_TABLE)
                .add(third_index)
                .read_unaligned();
            (entity.wrapping_add(SHAPER_C) as *mut u32).write_unaligned(third_shaper);
        }

        (entity.wrapping_add(VEHICLE_HANDLE) as *mut u32).write_unaligned(vehicle_config);
        (entity.wrapping_add(INITIALIZED_FLAG) as *mut u8).write(0);

        let random_range: unsafe extern "cdecl" fn(i32, i32) -> u32 =
            core::mem::transmute(lf_checker_rt::callee_addr(4));
        let random_offset = random_range(-400, 400);
        (entity.wrapping_add(RANDOM_VALUE) as *mut u32).write_unaligned(random_offset);

        let model_id = (vehicle_config.wrapping_add(MODEL_ID) as *const u16)
            .read_unaligned();
        let is_below_threshold = f32::from(model_id) <= MODEL_THRESHOLD;
        (entity.wrapping_add(MODEL_THRESHOLD_FLAG) as *mut u8)
            .write(u8::from(is_below_threshold));

        let acquire_slot: unsafe extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(lf_checker_rt::callee_addr(5));
        acquire_slot(entity)
    }
});
