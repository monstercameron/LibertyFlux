// original: 0x00e647e0 AMB_SLEDGEHAMMER
/// Register the "AMB_SLEDGEHAMMER" ambient sound with its audio object.
///
/// Same shape as [`rw_00e647d0`]: name plus owning object through the
/// registrar (thiscall/1, stubbed), returning its answer.
export!(cdecl, rw_00e647e0() -> u32 {
    unsafe {
        const NAME: u32 = 0x00E8C374; // "AMB_SLEDGEHAMMER"
        const OBJ: u32 = 0x012315E0;
        let register: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        register(relocated(OBJ), relocated(NAME))
    }
});
