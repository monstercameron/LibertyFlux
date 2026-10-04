// original: 0x00b86b70 GET_CAM_NEAR_CLIP
/// Script native `GET_CAM_NEAR_CLIP` (hash 0x2EF477FD).
///
/// Camera-singleton method; writes the near-clip float to the out-pointer.
///
/// Script arguments: self: Camera.
///
/// Forwards arg0 (integer/handle), arg1 (integer/handle) to the engine routine.
/// No return slot is written.
export!(cdecl, rw_00b86b70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let arg0 = *args.add(0);
        let arg1 = *args.add(1);
        callee_cdecl!(1, u32, arg0, arg1, )
    }
});
