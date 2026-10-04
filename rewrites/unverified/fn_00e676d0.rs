// original: 0x00e676d0 request_model_bm_turnstyle_set
/// Requests the "BM_turnstyle_set" model into its dedicated slot.
///
/// Forwards the model name string and the slot object to the shared
/// registration routine (thiscall/1, intercepted by the checker) and
/// returns its answer.
export!(cdecl, rw_00e676d0() -> u32 {
    unsafe {
        const MODEL_NAME: u32 = 0x00E9E908;
        const SLOT: u32 = 0x012FA580;
        callee_thiscall!(1, u32, relocated(SLOT), relocated(MODEL_NAME))
    }
});
