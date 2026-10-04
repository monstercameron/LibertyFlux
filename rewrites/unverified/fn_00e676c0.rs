// original: 0x00e676c0 request_model_vader
/// Requests the "vader" model into its dedicated slot.
///
/// Forwards the model name string and the slot object to the shared
/// registration routine (thiscall/1, intercepted by the checker) and
/// returns its answer.
export!(cdecl, rw_00e676c0() -> u32 {
    unsafe {
        const MODEL_NAME: u32 = 0x00E9E24C;
        const SLOT: u32 = 0x012F9F74;
        callee_thiscall!(1, u32, relocated(SLOT), relocated(MODEL_NAME))
    }
});
