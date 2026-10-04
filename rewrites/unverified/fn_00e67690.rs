// original: 0x00e67690 request_model_nrg900
/// Requests the "nrg900" model into its dedicated slot.
///
/// Forwards the model name string and the slot object to the shared
/// registration routine (thiscall/1, intercepted by the checker) and
/// returns its answer.
export!(cdecl, rw_00e67690() -> u32 {
    unsafe {
        const MODEL_NAME: u32 = 0x00E9E264;
        const SLOT: u32 = 0x012F9E54;
        callee_thiscall!(1, u32, relocated(SLOT), relocated(MODEL_NAME))
    }
});
