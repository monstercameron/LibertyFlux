// original: 0x00e65500 audio_slot_bind_x6_5500
/// Initialise six consecutive static audio voice objects, then register one default callback.
///
/// The original walks six 0x28-byte voice objects starting at `0x01283548`,
/// calling the tiny voice initializer (a thiscall taking only the object
/// address) on each, then registers the default callback stub at `0x00E71D10` with
/// the audio registrar and returns the registrar's answer. The callees are
/// intercepted and answered by the checker, so this function is fully described
/// by the seven outgoing calls and the returned answer.
export!(cdecl, rw_00e65500() -> u32 {
    unsafe {
        /// First of the six consecutive static voice objects (file VA).
        const VOICE_OBJ_BASE: u32 = 0x01283548;
        /// Bytes from one voice object to the next.
        const VOICE_STRIDE: u32 = 0x28;
        /// How many voice objects are initialised.
        const VOICE_COUNT: u32 = 6;
        /// Default callback stub registered afterwards (file VA).
        const CALLBACK: u32 = 0x00E71D10;
        let mut obj = relocated(VOICE_OBJ_BASE);
        let mut left = VOICE_COUNT;
        while left > 0 {
            let _: u32 = callee_thiscall!(1, u32, obj);
            obj = obj.wrapping_add(VOICE_STRIDE);
            left -= 1;
        }
        callee_cdecl!(2, u32, relocated(CALLBACK))
    }
});
