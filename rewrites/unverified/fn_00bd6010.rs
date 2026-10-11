// original: 0x00bd6010 NativeImpl_START_PTFX_ON_OBJ

/// Resolve an entity handle in its typed pool, hash the effect name, and submit the entity, position vector, scaled offset vector, and effect parameters to the particle manager. Each offset component is multiplied by the global particle scale in source-then-scale order. The optional bone index is forwarded between the vectors and the mode word. If the manager returns a nonzero effect id, register that id with the current script thread; return the manager's id. The entry is cdecl with raw words on the stack, and every direct callee is intercepted.
#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn rw_00bd6010(
    effect_name: u32,
    entity_handle: u32,
    position_x: u32,
    position_y: u32,
    position_z: u32,
    offset_x: u32,
    offset_y: u32,
    offset_z: u32,
    effect_parameter: u32,
) -> u32 {
    const EFFECT_MANAGER: u32 = 0x0139_4d60;
    const SCRIPT_MANAGER: u32 = 0x012b_d0c4;
    const POOL_GLOBAL: u32 = 0x01632c60;
    const PARTICLE_SCALE: u32 = 0x00fe_8728;
    const HASH_NAME: u32 = 1;
    const POOL_LOOKUP: u32 = 2;
    const CREATE_EFFECT: u32 = 3;
    const CURRENT_THREAD: u32 = 4;
    const REGISTER_EFFECT: u32 = 5;

    unsafe {
        let scale_bits = lf_checker_rt::global::<u32>(PARTICLE_SCALE).read_unaligned();
        let pool = lf_checker_rt::global::<u32>(POOL_GLOBAL).read_unaligned();
        let entity = lf_checker_rt::callee_thiscall!(POOL_LOOKUP, u32, pool, entity_handle);
        let position = [position_x, position_y, position_z];
        let offset = [
            (core::hint::black_box(f32::from_bits(offset_x)) * core::hint::black_box(f32::from_bits(scale_bits))).to_bits(),
            (core::hint::black_box(f32::from_bits(offset_y)) * core::hint::black_box(f32::from_bits(scale_bits))).to_bits(),
            (core::hint::black_box(f32::from_bits(offset_z)) * core::hint::black_box(f32::from_bits(scale_bits))).to_bits(),
        ];
        let name_hash = lf_checker_rt::callee_cdecl!(HASH_NAME, u32, effect_name);
        let effect_id = lf_checker_rt::callee_thiscall!(
            CREATE_EFFECT,
            u32,
            lf_checker_rt::relocated(EFFECT_MANAGER),
            name_hash,
            entity,
            position.as_ptr() as usize as u32,
            offset.as_ptr() as usize as u32,
            u32::MAX,
            effect_parameter,
        );
        if effect_id != 0 {
            let thread = lf_checker_rt::callee_stdcall!(CURRENT_THREAD, u32, 0, 0);
            let script_manager = lf_checker_rt::global::<u32>(SCRIPT_MANAGER).read_unaligned();
            let _ = lf_checker_rt::callee_thiscall!(
                REGISTER_EFFECT,
                u32,
                script_manager,
                effect_id,
                5,
                thread,
            );
        }
        effect_id
    }
}
