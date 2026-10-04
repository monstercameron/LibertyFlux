// original: 0x00e65140 audio_init_register_04
/// Audio init/register stub: initialise the global object at
/// `0x012838E8` through the object helper, then register the handler slot at
/// `0x00E71B40` through the registrar and return its answer.
///
/// Takes no arguments and reads no registers; the only observables are
/// the two outgoing calls (in order) and the returned value.
export!(cdecl, rw_00e65140() -> u32 {
    callee_thiscall!(1, u32, relocated(0x012838E8));
    callee_cdecl!(2, u32, relocated(0x00E71B40))
});
