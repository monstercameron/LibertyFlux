// original: 0x00b4f400 CDummyPed::vf51

/// Refresh a dummy ped's movement driver and mark it awake.
///
/// Runs the base refresh (callee 1, thiscall on `this` with no arguments),
/// then the embedded driver's virtual slot at `+0x08` (thiscall on the
/// driver at `this + 0x350`), then sets the awake byte at `this + 0x390`.
/// No meaningful return value.
///
/// Original: 0x00b4f400 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00b4f400(this: u32) -> u32 {
    unsafe {
        const BASE_REFRESH: u32 = 1;
        const DRIVER: u32 = 0x350;
        const STEP_SLOT: u32 = 0x08;
        const AWAKE: u32 = 0x390;
        lf_checker_rt::callee_thiscall!(BASE_REFRESH, u32, this);
        let driver = this + DRIVER;
        let vt = (driver as *const u32).read_unaligned();
        let step: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((vt + STEP_SLOT) as *const u32).read_unaligned() as usize);
        step(driver);
        ((this + AWAKE) as *mut u8).write(1);
        0
    }
});
