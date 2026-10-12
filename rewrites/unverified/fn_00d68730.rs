// original: 0x00D68730 CRenderPhaseDeferredLighting_LightsToScreen::vf7

/// Tail-calls the base render method when either global gate byte is nonzero: the byte at global 0x0103e488 or the byte at offset two of the word beginning at 0x011609f4. Otherwise returns without calling it.
lf_checker_rt::export!(thiscall, rw_00d68730(this: u32) -> () {
    unsafe {
        const FIRST_GATE: u32 = 0x0103e488;
        const SECOND_GATE: u32 = 0x011609f6;
        if lf_checker_rt::global::<u8>(FIRST_GATE).read() != 0
            || lf_checker_rt::global::<u8>(SECOND_GATE).read() != 0
        {
            lf_checker_rt::callee_thiscall!(1, (), this);
        }
    }
});
