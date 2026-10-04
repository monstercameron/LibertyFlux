// original: 0x00e654e0 audio_slot_bind_54e0
/// Initialise one static audio voice object, then register its default callback.
///
/// The original first calls the tiny voice initializer (a thiscall taking only
/// the object address: it zeroes the object's first word and sets its flag byte)
/// with the static object at `0x01283E68`, then registers the default callback
/// stub at `0x00E71D00` with the audio registrar and returns the registrar's
/// answer. Neither call reads anything the rewrite owns: both callees are
/// intercepted and answered by the checker, so this function is fully described
/// by the two outgoing calls and the returned answer.
export!(cdecl, rw_00e654e0() -> u32 {
    unsafe {
        /// Static voice object this instance initialises (file VA).
        const VOICE_OBJ: u32 = 0x01283E68;
        /// Default callback stub registered for it (file VA).
        const CALLBACK: u32 = 0x00E71D00;
        let _: u32 = callee_thiscall!(1, u32, relocated(VOICE_OBJ));
        callee_cdecl!(2, u32, relocated(CALLBACK))
    }
});
