// original: 0x00bd5f50 NativeImpl_START_PTFX

/// Hash the effect name and submit the resulting identifier, two local
/// three-word vectors, a mode, and the final input word to the effect manager.
/// The first vector copies three input float words; the second multiplies its
/// three float words by the global particle scale in source-then-scale order.
/// If the manager returns a nonzero signed or unsigned identifier, request the
/// current script thread and register the effect with the script stub. Return
/// the manager's identifier either way. The function uses cdecl with eight
/// stack arguments; helper calls are scripted by the checker.
#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn rw_00bd5f50(
    effect_name: u32,
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
    const PARTICLE_SCALE: u32 = 0x00fe_8728;
    const HASH_NAME: u32 = 1;
    const CREATE_EFFECT: u32 = 2;
    const CURRENT_THREAD: u32 = 3;
    const REGISTER_EFFECT: u32 = 4;

    #[inline(always)]
    fn scale(bits: u32, scale_bits: u32) -> u32 {
        (core::hint::black_box(f32::from_bits(bits))
            * core::hint::black_box(f32::from_bits(scale_bits)))
        .to_bits()
    }

    unsafe {
        let scale_bits = lf_checker_rt::global::<u32>(PARTICLE_SCALE).read_unaligned();
        let position = [position_x, position_y, position_z];
        let offset = [
            scale(offset_x, scale_bits),
            scale(offset_y, scale_bits),
            scale(offset_z, scale_bits),
        ];
        let name_hash = lf_checker_rt::callee_cdecl!(HASH_NAME, u32, effect_name);
        let effect_id = lf_checker_rt::callee_thiscall!(
            CREATE_EFFECT,
            u32,
            lf_checker_rt::relocated(EFFECT_MANAGER),
            name_hash,
            0,
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
