// original: 0x00bd6880 NativeImpl_UPDATE_PTFX_OFFSETS

/// Forward an entity handle and two three-float vectors to the particle-effect
/// offset updater. The seven cdecl stack words are one handle followed by six
/// raw IEEE-754 single-precision values. The first vector is forwarded bit for
/// bit; each component of the second is multiplied, in source-then-scale order,
/// by the global particle scale. Both vector pointers refer to temporary local
/// arrays, and the function returns the updater's EAX result unchanged.
#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn rw_00bd6880(
    handle: u32,
    first_x: u32,
    first_y: u32,
    first_z: u32,
    second_x: u32,
    second_y: u32,
    second_z: u32,
) -> u32 {
    const UPDATER_THIS: u32 = 0x0139_4d60;
    const PARTICLE_SCALE: u32 = 0x00fe_8728;
    const UPDATER_CALLEE: u32 = 1;

    #[inline(always)]
    fn scaled_component(component_bits: u32, scale_bits: u32) -> u32 {
        let component = f32::from_bits(component_bits);
        let scale = f32::from_bits(scale_bits);
        (core::hint::black_box(component) * core::hint::black_box(scale)).to_bits()
    }

    unsafe {
        let scale_bits = lf_checker_rt::global::<u32>(PARTICLE_SCALE).read_unaligned();
        let first_vector = [first_x, first_y, first_z];
        let scaled_vector = [
            scaled_component(second_x, scale_bits),
            scaled_component(second_y, scale_bits),
            scaled_component(second_z, scale_bits),
        ];
        lf_checker_rt::callee_thiscall!(
            UPDATER_CALLEE,
            u32,
            lf_checker_rt::relocated(UPDATER_THIS),
            handle,
            first_vector.as_ptr() as usize as u32,
            scaled_vector.as_ptr() as usize as u32,
        )
    }
}
