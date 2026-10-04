// original: 0x00BD4450 START_PTFX_ON_PED_BONE
// Rewrite of native handler START_PTFX_ON_PED_BONE.
//
// Passes constant callback address plus the call context to the particle engine helper.
// The call context holds the return-slot pointer at +0 and the
// argument-array pointer at +8.
// Returns the engine's answer, matching the value the original leaves in EAX.
export!(cdecl, rw_00bd4450(ctx: u32) -> u32 {
    unsafe {
        let engine: extern "cdecl" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        engine(relocated(0xBD6280), ctx)
    }
});
