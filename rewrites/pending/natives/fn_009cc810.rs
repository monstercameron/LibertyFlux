// original: 0x009cc810 START_MOBILE_PHONE_RINGING
lf_rn22_rt::export!(cdecl,
    /// Script native `START_MOBILE_PHONE_RINGING`.
    /// The handler entry jumps here; this body reads no script arguments: it
    /// invokes one engine object with two zero arguments to start the phone
    /// ringing.
    rw_009cc810(ctx: u32) -> u32 {
    unsafe {
        let _ = ctx;
        let ring: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(lf_rn22_rt::callee_addr(1) as usize);
        ring(lf_rn22_rt::relocated(0x01284A60), 0, 0)
    }
});
