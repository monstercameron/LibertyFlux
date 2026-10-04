// original: 0x00e65320 audio_init_register_19
/// Audio init/register stub: initialise the global object at
/// `0x01284168` through the object helper, then register the handler slot at
/// `0x00E71C30` through the registrar and return its answer.
///
/// Takes no arguments and reads no registers; the only observables are
/// the two outgoing calls (in order) and the returned value.
export!(cdecl, rw_00e65320() -> u32 {
    callee_thiscall!(1, u32, relocated(0x01284168));
    callee_cdecl!(2, u32, relocated(0x00E71C30))
});
