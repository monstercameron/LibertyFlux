// original: 0x00b4f420 CDummyPed::vf33

/// Refresh a dummy ped unless a mode gate or flag blocks it.
///
/// A parameter word from game address `PARAM` is offered to the fetch hook
/// (callee 1, thiscall on `this` through the data table at `FETCH_TAB`).
/// The gate passes 1 when the fetch succeeds, else 0 unless mode byte
/// `MODE_A` is set while mode byte `MODE_B` is clear... precisely: 1 when
/// the fetch is non-zero; otherwise 0 when `MODE_A` is clear, else the
/// `MODE_B`-non-zero test. The gate is then ORed with flag bytes `FLAG0`
/// and `FLAG1`; a non-zero result returns 0 at once. Otherwise the driver
/// resolves through virtual slot `+0xD0` (thiscall on `this` with `this`),
/// adapts (callee 3, thiscall on the resolution) and probes (callee 4,
/// thiscall on the auxiliary block at `+0x78` with kind 0x200000 and slot
/// 1): on success the same slot is invoked a second time with NO stack
/// argument (ecx only), after which four pose floats (`POSE`) would be
/// copied to `this + 0x370`. Against a one-word callee the second call
/// unbalances the stack, so neither the copy nor the calls below run
/// on this path; the stepper and finish run only when the probe fails.
/// Finally the stepper
/// runs (virtual slot `+0x04` on the block at `+0x350`) and control passes
/// tail-wise to the finish hook (callee 6, thiscall on `this`), whose
/// result is returned.
///
/// Original: 0x00b4f420 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00b4f420(this: u32) -> u32 {
    unsafe {
        const FETCH: u32 = 1;
        const DRIVER_SLOT: u32 = 0xd0;
        const ADAPT: u32 = 3;
        const PROBE: u32 = 4;
        const STEPPER_SLOT: u32 = 0x04;
        const FINISH: u32 = 6;
        const PARAM: u32 = 0x017accd8;
        const MODE_A: u32 = 0x0105b48f;
        const MODE_B: u32 = 0x017ed8d1;
        const FLAG0: u32 = 0x01173590;
        const FLAG1: u32 = 0x01173591;
        const POSE: u32 = 0x01050d30;
        const OUT: u32 = 0x370;
        const AUX: u32 = 0x78;
        const DRIVER: u32 = 0x350;
        const PROBE_KIND: u32 = 0x200000;
        const PROBE_SLOT: u32 = 1;
        let p = lf_checker_rt::global::<u32>(PARAM).read_unaligned();
        let r = lf_checker_rt::callee_thiscall!(FETCH, u32, this, p);
        let gate: u8 = if r != 0 {
            1
        } else if lf_checker_rt::global::<u8>(MODE_A).read() == 0 {
            0
        } else {
            u8::from(lf_checker_rt::global::<u8>(MODE_B).read() != 0)
        };
        let gate = gate | lf_checker_rt::global::<u8>(FLAG0).read()
            | lf_checker_rt::global::<u8>(FLAG1).read();
        if gate != 0 {
            return 0;
        }
        let vt = (this as *const u32).read_unaligned();
        let driver: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
            ((vt + DRIVER_SLOT) as *const u32).read_unaligned() as usize,
        );
        let h = driver(this, this);
        let aux = ((this + AUX) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(ADAPT, u32, h);
        let ok = lf_checker_rt::callee_thiscall!(PROBE, u32, aux, PROBE_KIND, PROBE_SLOT);
        if ok != 0 {
            // The original re-invokes the same driver slot WITHOUT pushing
            // the argument this time (ecx only). Against the one-word
            // callee both sides unbalance the stack here; the pose copy
            // below is the original's own unreachable-aftermath on this
            // path and is reproduced for fidelity.
            let vt = (this as *const u32).read_unaligned();
            let driver0: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                ((vt + DRIVER_SLOT) as *const u32).read_unaligned() as usize,
            );
            driver0(this);
            let mut k: u32 = 0;
            while k < 4 {
                let w = lf_checker_rt::global::<u32>(POSE + k * 4).read_unaligned();
                ((this + OUT + k * 4) as *mut u32).write_unaligned(w);
                k += 1;
            }
        }
        let field = this + DRIVER;
        let slot = (field as *const u32).read_unaligned();
        let step: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            ((slot + STEPPER_SLOT) as *const u32).read_unaligned() as usize,
        );
        step(field);
        lf_checker_rt::callee_thiscall!(FINISH, u32, this)
    }
});
