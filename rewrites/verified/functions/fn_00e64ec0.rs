// original: 0x00e64ec0 audio_register_04
/// Audio registration stub `audio_register_04`: initialize one static audio object,
/// then register its static callback with the audio manager.
///
/// Calls the object initializer (thiscall/0, intercepted as callee 1)
/// with the fixed object address `0x01283848`, then the registrar
/// (cdecl/1, intercepted as callee 2) with the fixed callback address
/// `0x00E71A00`. Returns the registrar's answer, as the original leaves
/// it in EAX. Takes no inputs; both addresses are relocated.
export!(cdecl, rw_00e64ec0() -> u32 {
    const OBJ: u32 = 0x01283848;
    const CALLBACK: u32 = 0x00E71A00;
    let _ = callee_thiscall!(1, u32, relocated(OBJ));
    callee_cdecl!(2, u32, relocated(CALLBACK))
});
