// original: 0x00b9f360 GET_MODEL_PED_IS_HOLDING
/// Return the model a ped is holding, if any.
///
/// Forwards the ped handle (argument 0) to the engine and stores its full
/// 32-bit answer in the return slot. Returns the engine answer.
export!(cdecl, rw_00b9f360(ctx: u32) -> u32 {
    unsafe {
        let base = ctx as *const u32;
        let ret_slot = *base as *mut u32;
        let args = *base.add(2) as *const u32;
        let answer: u32 = callee_cdecl!(1, u32, *args);
        *ret_slot = answer;
        answer
    }
});
