// original: 0x00e67640 request_model_n_faggio
/// Requests the "n_faggio" model into its dedicated slot.
///
/// Forwards the model name string and the slot object to the shared
/// registration routine (thiscall/1, intercepted by the checker) and
/// returns its answer.
export!(cdecl, rw_00e67640() -> u32 {
    unsafe {
        const MODEL_NAME: u32 = 0x00E9E648;
        const SLOT: u32 = 0x012FA2A4;
        callee_thiscall!(1, u32, relocated(SLOT), relocated(MODEL_NAME))
    }
});
