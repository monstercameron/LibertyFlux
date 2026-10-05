// original: 0x00e69670 veh_square_init_e69670 (proposed)
/// Initialise one derived vehicle tuning constant as the square of a global.
///
/// Reads one `f32` global and stores its SSE square (`mulss` of the value with
/// itself) into a second global. Takes no arguments and makes no calls (cdecl, no
/// stack words); the return register is untouched. Operand order is pinned with
/// `black_box`.
///
/// Original: 0x00e69670 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_00e69670() -> u32 {
    unsafe {
        const SOURCE: u32 = 0x010459B0;
        const DESTINATION: u32 = 0x016624C8;
        let a = f32::from_bits(core::ptr::read_unaligned(lf_checker_rt::global::<u32>(SOURCE) as *const u32));
        let q = core::hint::black_box(a) * core::hint::black_box(a);
        core::ptr::write_unaligned(lf_checker_rt::global::<u32>(DESTINATION), q.to_bits());
        0
    }
});
