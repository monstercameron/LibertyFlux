// original: 0x00b86bf0 GET_CAM_STATE
/// Script native `GET_CAM_STATE` (hash 0x22AA0984).
///
/// Camera-singleton method; returns the int camera state.
///
/// Script arguments: cam: int.
///
/// Forwards arg0 (integer/handle) to the engine routine.
/// The engine's full-word answer is stored into the return slot.
export!(cdecl, rw_00b86bf0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let slot = *(ctx as *const *mut u32);
        let arg0 = *args.add(0);
        let answer: u32 = callee_cdecl!(1, u32, arg0, );
        *slot = answer;
        answer
    }
});
