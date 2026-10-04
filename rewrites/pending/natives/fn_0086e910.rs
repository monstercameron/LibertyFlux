// original: 0x0086e910 TIMESTEP
/// `TIMESTEP` (native hash `0x35694DDC`): copy the engine's current frame
/// timestep, in seconds as an f32 bit pattern, into the context return
/// slot. Reads one game-global float; makes no calls.
export!(cdecl, rw_0086e910(ctx: *const crate::NativeCtx) -> () {
    unsafe {
        let bits: u32 = *lf_rn08_rt::global(0x110b730);
        *(*ctx).ret_slot = bits;
    }
});
