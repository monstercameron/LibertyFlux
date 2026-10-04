// original: 0x00e647e0 AMB_SLEDGEHAMMER
/// Register the "AMB_SLEDGEHAMMER" ambient sound with its audio object.
///
/// Same shape as [`rw_00e647d0`]: name plus owning object through the
/// registrar (thiscall/1, stubbed), returning its answer.
export!(cdecl, rw_00e647e0() -> u32 {
    unsafe {
        const NAME: u32 = 0x00E8C374; // "AMB_SLEDGEHAMMER"
        const OBJ: u32 = 0x012315E0;
        callee_thiscall!(1, u32, relocated(OBJ), relocated(NAME))
    }
});
