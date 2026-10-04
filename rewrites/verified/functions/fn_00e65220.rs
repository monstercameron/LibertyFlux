// original: 0x00e65220 audio_init_register_11
/// Audio init/register stub: initialise the global object at
/// `0x01283C0C` through the object helper, then register the handler slot at
/// `0x00E71BB0` through the registrar and return its answer.
///
/// Takes no arguments and reads no registers; the only observables are
/// the two outgoing calls (in order) and the returned value.
export!(cdecl, rw_00e65220() -> u32 {
    callee_thiscall!(1, u32, relocated(0x01283C0C));
    callee_cdecl!(2, u32, relocated(0x00E71BB0))
});
