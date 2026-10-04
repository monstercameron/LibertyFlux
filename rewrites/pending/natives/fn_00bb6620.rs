// original: 0x00bb6620 REGISTER_ODDJOB_MISSION_PASSED
lf_rn22_rt::export!(cdecl,
    /// Script native `REGISTER_ODDJOB_MISSION_PASSED`.
    /// Sets the latest odd job mission passed. The handler entry jumps here;
    /// this body reads no script arguments: it fetches a float through one
    /// engine call, feeds its bits to a second call, makes a third call with
    /// constant arguments, then copies one engine global to another.
    rw_00bb6620(ctx: u32) -> u32 {
    unsafe {
        let _ = ctx;
        let fetched: f32 = lf_rn22_rt::callee_cdecl!(1, f32, 0x1B0);
        let _: u32 = lf_rn22_rt::callee_cdecl!(2, u32, 0x1A5, fetched.to_bits());
        let _: u32 = lf_rn22_rt::callee_cdecl!(3, u32, 0x1B0, 0);
        let value = *lf_rn22_rt::global::<u32>(0x011735B4);
        *lf_rn22_rt::global::<u32>(0x011D78F4) = value;
        value
    }
});
