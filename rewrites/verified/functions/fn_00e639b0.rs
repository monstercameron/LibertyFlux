// original: 0x00e639b0 register_object
/// Register the object at 0x11D6FE4 with two name pointers.
///
/// Single thiscall/4 to the registrar (stubbed by the checker): the object
/// pointer in ECX and stack arguments `(0, 0xE886FC, 0, 0xE88595)`. Returns
/// the registrar's answer.
export!(cdecl, rw_00e639b0() -> u32 {
    unsafe {
        let register: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        register(
            relocated(0x11D6FE4),
            0,
            relocated(0xE886FC),
            0,
            relocated(0xE88595),
        )
    }
});
