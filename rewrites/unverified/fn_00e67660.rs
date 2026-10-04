// original: 0x00e67660 request_model_gb_bikehelmet01
/// Requests the "GB_BIKEHELMET01" model into its dedicated slot.
///
/// Forwards the model name string and the slot object to the shared
/// registration routine (thiscall/1, intercepted by the checker) and
/// returns its answer.
export!(cdecl, rw_00e67660() -> u32 {
    unsafe {
        const MODEL_NAME: u32 = 0x00E9DE30;
        const SLOT: u32 = 0x012F9D4C;
        callee_thiscall!(1, u32, relocated(SLOT), relocated(MODEL_NAME))
    }
});
