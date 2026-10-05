// original: 0x00942af0 streaming_startup_step (proposed)

/// Run one streaming start-up step when the gate worker allows it.
///
/// Polls the gate worker (cdecl, no arguments); a zero low byte skips the
/// step. Otherwise resolves the worker for the configured slot, runs the
/// fulfilling call with a zero argument and clears bits 1 and 2 of the
/// fulfilled object's status word at `+0x264`. Always records stage 2 in
/// the progress word. Returns the fulfilling call's answer, or the gate's
/// answer when the step was skipped.
///
/// Original: 0x00942af0 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_00942af0() -> u32 {
    unsafe {
        const SLOT: u32 = 0x01036F18;
        const PROGRESS: u32 = 0x010375A0;
        const STATUS: u32 = 0x264;
        const KEEP: u32 = 0xFFFF_FFF9;
        const STAGE: u32 = 2;
        const GATE: u32 = 1;
        const RESOLVE: u32 = 2;
        const FULFIL: u32 = 3;
        let gate: u32 = lf_checker_rt::callee_cdecl!(GATE, u32,);
        let mut answer = gate;
        if gate & 0xFF != 0 {
            let slot = lf_checker_rt::global::<u32>(SLOT).read();
            let _: u32 = lf_checker_rt::callee_cdecl!(RESOLVE, u32, slot);
            let fulfilled: u32 = lf_checker_rt::callee_cdecl!(FULFIL, u32, 0u32);
            let status = (fulfilled + STATUS) as *mut u32;
            status.write_unaligned(status.read_unaligned() & KEEP);
            answer = fulfilled;
        }
        lf_checker_rt::global::<u32>(PROGRESS).write(STAGE);
        answer
    }
});
