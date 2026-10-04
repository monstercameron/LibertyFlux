// original: 0x00e67bb0 CJ_EXT_DOOR_16

/// Register the per-asset slot for "CJ_EXT_DOOR_16" with the asset registry.
///
/// Calls the shared slot-registration routine (thiscall: the slot
/// pointer in ECX, the name pointer pushed) with this wrapper's two
/// constants: `SLOT_PTR`, the slot record in the data section, and
/// `NAME_PTR`, the asset name string in the read-only section. The
/// routine stores the name into the slot, links the slot into the
/// registry list, and returns the slot pointer, which this wrapper
/// returns unchanged.
///
/// Takes no arguments and reads no caller state (the incoming ECX
/// is overwritten); stack effect is zero (plain `ret`: the callee
/// pops its one word). Original: cdecl/0, one outgoing call.
lf_checker_rt::export!(cdecl, rw_00e67bb0() -> u32 {
    const NAME_PTR: u32 = 0x00E9E8C4;
    const SLOT_PTR: u32 = 0x012F9FA4;
    const SLOT_REGISTER: u32 = 1;
    lf_checker_rt::callee_thiscall!(
        SLOT_REGISTER,
        u32,
        lf_checker_rt::relocated(SLOT_PTR),
        lf_checker_rt::relocated(NAME_PTR)
    )
});
