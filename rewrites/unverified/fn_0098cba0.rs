// original: 0x0098CBA0 audVehicleAudioEntity::model_config_name

/// Uses the signed model index at `this + 0x820 + 0x2E` to read a 32-bit
/// pointer from the model configuration table, then returns that pointer plus
/// the 0x60-byte name field offset. The pointer table is game-owned data.
lf_checker_rt::export!(thiscall, rw_audvehicleaudioentity_model_config_name(this: u32) -> u32 {
    const CONFIG_OBJECT: u32 = 0x820;
    const MODEL_INDEX: u32 = 0x2E;
    const MODEL_CONFIG_TABLE: u32 = 0x1295_CD8;
    const NAME_OFFSET: u32 = 0x60;

    unsafe {
        let config = (this.wrapping_add(CONFIG_OBJECT) as *const u32).read_unaligned();
        let index = (config.wrapping_add(MODEL_INDEX) as *const i16).read_unaligned() as isize;
        let table = lf_checker_rt::global::<u32>(MODEL_CONFIG_TABLE);
        table.offset(index).read_unaligned().wrapping_add(NAME_OFFSET)
    }
});
