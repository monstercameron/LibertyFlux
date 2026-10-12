// original: 0x0086e910 TIMESTEP
/// `TIMESTEP` (native hash `0x35694DDC`): copy the engine's current frame
/// timestep, in seconds as an f32 bit pattern, into the context return
/// slot. Reads one game-global float; makes no calls.
export!(cdecl, rw_0086e910(ctx: *const NativeCtx08) -> () {
    unsafe {
        let bits: u32 = *global(0x110b730);
        *(*ctx).ret_slot = bits;
    }
});

/// Native call context, r-n08 shape: return-slot pointer, reserved word,
/// argument-array pointer, argument count. (Renamed from r-n08's NativeCtx.)
#[repr(C)]
pub struct NativeCtx08 {
    pub ret_slot: *mut u32,
    pub reserved: u32,
    pub args: *const u32,
    pub argc: u32,
}
