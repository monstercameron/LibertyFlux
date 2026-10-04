// original: 0x00e5d8a0 init_slots_register_dtor_e5d8a0
/// Initializes a 25-entry slot table (each entry: two `0xFFFFFFFF` marker
/// words followed by a zero word), then registers a teardown routine with
/// the runtime registrar and returns the registrar's answer.
export!(cdecl, rw_e5d8a0() -> u32 {
    const COUNT: usize = 25;
    unsafe {
        let table = global::<u32>(0x019D2898);
        for i in 0..COUNT {
            *table.add(i * 3) = 0xFFFF_FFFF;
            *table.add(i * 3 + 1) = 0xFFFF_FFFF;
            *table.add(i * 3 + 2) = 0;
        }
    }
    callee_cdecl!(2, u32, relocated(0x00E6EA50))
});
