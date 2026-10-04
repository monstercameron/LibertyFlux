// original: 0x00B9E2A0 ATTACH_PED_TO_WORLD_PHYSICALLY
// Rewrite of native handler ATTACH_PED_TO_WORLD_PHYSICALLY.
//
// Passes constant callback address plus the call context to the attach engine helper.
// The call context holds the return-slot pointer at +0 and the
// argument-array pointer at +8.
// Returns the engine's answer, matching the value the original leaves in EAX.
export!(cdecl, rw_00b9e2a0(ctx: u32) -> u32 {
    unsafe {
        let engine: extern "cdecl" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        engine(relocated(0xBA4510), ctx)
    }
});
