// original: 0x00e67630 request_model_faggio
/// Requests the "faggio" model into its dedicated slot.
///
/// Forwards the model name string and the slot object to the shared
/// registration routine (thiscall/1, intercepted by the checker) and
/// returns its answer.
export!(cdecl, rw_00e67630() -> u32 {
    unsafe {
        const MODEL_NAME: u32 = 0x00E9E274;
        const SLOT: u32 = 0x012FA5EC;
        callee_thiscall!(1, u32, relocated(SLOT), relocated(MODEL_NAME))
    }
});
