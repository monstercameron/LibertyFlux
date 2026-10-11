// original: 0x00bd65c0 NativeImpl_TRIGGER_PTFX_ON_OBJ_BONE

/// Hash the effect name, resolve an entity handle in its typed pool when present, then submit a position vector and globally scaled offset vector to the particle trigger manager. A bone index occupies the mode slot for bone variants; other variants pass the all-ones mode word. Return the manager's result. The entry and direct callees use 32-bit stack words.
#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn rw_00bd65c0(
    effect_name: u32,
    entity_handle: u32,
    position_x: u32,
    position_y: u32,
    position_z: u32,
    offset_x: u32,
    offset_y: u32,
    offset_z: u32,
    bone_index: u32,
    effect_parameter: u32
) -> u32 {
    const POOL_LOOKUP: u32 = 1;
    const HASH_NAME: u32 = 2;
    const TRIGGER_EFFECT: u32 = 3;
    const SCALE_GLOBAL: u32 = 0x00fe8728;
    const EFFECT_MANAGER: u32 = 0x01394d60;
    const POOL_GLOBAL: u32 = 0x01632c60;
    unsafe {
        let scale_bits = lf_checker_rt::global::<u32>(SCALE_GLOBAL).read_unaligned();
        let pool = lf_checker_rt::global::<u32>(POOL_GLOBAL).read_unaligned();
        let entity = lf_checker_rt::callee_thiscall!(POOL_LOOKUP, u32, pool, entity_handle);
        let position = [position_x, position_y, position_z];
        let scaled_offset = [
            (core::hint::black_box(f32::from_bits(offset_x)) * core::hint::black_box(f32::from_bits(scale_bits))).to_bits(),
            (core::hint::black_box(f32::from_bits(offset_y)) * core::hint::black_box(f32::from_bits(scale_bits))).to_bits(),
            (core::hint::black_box(f32::from_bits(offset_z)) * core::hint::black_box(f32::from_bits(scale_bits))).to_bits(),
        ];
        let name_hash = lf_checker_rt::callee_cdecl!(HASH_NAME, u32, effect_name);
        lf_checker_rt::callee_thiscall!(
            TRIGGER_EFFECT,
            u32,
            lf_checker_rt::relocated(EFFECT_MANAGER),
            name_hash,
            entity,
            position.as_ptr() as usize as u32,
            scaled_offset.as_ptr() as usize as u32,
            bone_index,
            effect_parameter,
        )
    }
}