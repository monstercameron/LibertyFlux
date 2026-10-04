// original: 0x00e65200 audio_init_register_10
/// Audio init/register stub: initialise the global object at
/// `0x0128381C` through the object helper, then register the handler slot at
/// `0x00E71BA0` through the registrar and return its answer.
///
/// Takes no arguments and reads no registers; the only observables are
/// the two outgoing calls (in order) and the returned value.
export!(cdecl, rw_00e65200() -> u32 {
    callee_thiscall!(1, u32, relocated(0x0128381C));
    callee_cdecl!(2, u32, relocated(0x00E71BA0))
});
