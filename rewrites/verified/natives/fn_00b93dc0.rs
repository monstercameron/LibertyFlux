// original: 0x00b93dc0 ADD_STUNT_JUMP
/// Script native `ADD_STUNT_JUMP` (hash 0x422E7AC3).
///
/// Unlike the other handlers this one forwards no script arguments itself:
/// it passes an engine-function address and the whole call context to a
/// shared unpacker routine, which unpacks the stunt-jump definition from
/// the argument array. The pushed address is a relocated immediate in the
/// original (its RVA is in the relocation table), so it is derived with
/// `relocated()` here, never hard-coded as a mapped address.
export!(cdecl, rw_00b93dc0(ctx: *const u8) -> u32 {
    callee_cdecl!(1, u32, relocated(0x00B95630), ctx as u32)
});
