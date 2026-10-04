// original: 0x00BD41B0 REMOVE_PTFX_FROM_VEHICLE
// Rewrite of native handler REMOVE_PTFX_FROM_VEHICLE.
//
// Passes vehicle handle to the particle-effect engine helper.
// The call context holds the return-slot pointer at +0 and the
// argument-array pointer at +8.
// Returns the engine's answer, matching the value the original leaves in EAX.
export!(cdecl, rw_00bd41b0(ctx: u32) -> u32 {
    unsafe {
        let argv = *((ctx + 8) as *const u32) as *const u32;
        let a0 = *argv.add(0);
        let engine: extern "cdecl" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        engine(a0)
    }
});
