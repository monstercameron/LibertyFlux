// original: 0x00B8C480 DOES_BLIP_EXIST
// Rewrite of native handler DOES_BLIP_EXIST.
//
// Passes blip handle; stores the engine answer's low byte.
// The call context holds the return-slot pointer at +0 and the
// argument-array pointer at +8.
// Returns the return-slot pointer, matching the value the original leaves in EAX.
export!(cdecl, rw_00b8c480(ctx: u32) -> u32 {
    unsafe {
        let argv = *((ctx + 8) as *const u32) as *const u32;
        let a0 = *argv.add(0);
        let engine: extern "cdecl" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let ret = *(ctx as *const u32) as *mut u32;
        let ans = engine(a0);
        *ret = ans & 0xFF;
        ret as u32
    }
});
