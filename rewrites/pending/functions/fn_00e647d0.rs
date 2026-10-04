// original: 0x00e647d0 AMB_PICKAXE
/// Register the "AMB_PICKAXE" ambient sound with its audio object.
///
/// Hands the name string to the registrar for the object at 0x01231418
/// (thiscall/1, stubbed by the checker) and returns its answer.
export!(cdecl, rw_00e647d0() -> u32 {
    unsafe {
        const NAME: u32 = 0x00E8C358; // "AMB_PICKAXE"
        const OBJ: u32 = 0x01231418;
        callee_thiscall!(1, u32, relocated(OBJ), relocated(NAME))
    }
});
