// original: 0x00e67680 request_model_innovation
/// Requests the "innovation" model into its dedicated slot.
///
/// Forwards the model name string and the slot object to the shared
/// registration routine (thiscall/1, intercepted by the checker) and
/// returns its answer.
export!(cdecl, rw_00e67680() -> u32 {
    unsafe {
        const MODEL_NAME: u32 = 0x00E9E210;
        const SLOT: u32 = 0x012FA154;
        callee_thiscall!(1, u32, relocated(SLOT), relocated(MODEL_NAME))
    }
});
