// original: 0x00E5CBB0 timer_pool_init_a_register
/// Initialise one mainloop-timing pool object, then register its callback.
///
/// Calls the pool initialiser as thiscall/3 (callee id 1): the object in
/// ECX, then three constant stack words. Discards its answer, registers
/// the callback's code address with the registrar (cdecl/1, callee id 2)
/// and returns the registrar's answer. Takes no arguments.
export!(cdecl, rw_00e5cbb0() -> u32 {
    unsafe {
        let _init_answer: u32 = callee_thiscall!(1, u32, relocated(0x01904988),
            relocated(0x01835FE0), 0x80000u32, 1u32);
        callee_cdecl!(2, u32, relocated(0x00E6E520))
    }
});
