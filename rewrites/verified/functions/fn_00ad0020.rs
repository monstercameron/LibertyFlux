// original: 0x00AD0020 audio_drain_pending (proposed)

/// Drain the pending audio queue when its count is positive.
///
/// If the pending-count global is not positive, returns at once (eax is the
/// caller's leftover there, pinned to 0 in the proof). Otherwise notifies
/// the queue owner (thiscall/1) with the queue address, submits the five
/// queue words (cdecl/5: queue base, three words, length 0x40), then zeroes
/// the count and the trailing flag word. Takes no arguments (cdecl/0);
/// returns 0 on the drain path.
lf_checker_rt::export!(cdecl, rw_00ad0020() -> u32 {
    unsafe {
        const NOTIFY: u32 = 1;
        const SUBMIT: u32 = 2;
        const COUNT: u32 = 0x0154E140;
        const OWNER: u32 = 0x017F583C;
        const QUEUE: u32 = 0x01110090;
        const W0: u32 = 0x0154E148;
        const W1: u32 = 0x0154E14C;
        const W2: u32 = 0x0154E150;
        const BASE: u32 = 0x0154E154;
        const FLAG: u32 = 0x0154E158;
        const LEN: u32 = 0x40;
        if lf_checker_rt::global::<i32>(COUNT).read() <= 0 {
            return 0;
        }
        let owner = lf_checker_rt::global::<u32>(OWNER).read();
        lf_checker_rt::callee_thiscall!(NOTIFY, u32, owner, lf_checker_rt::relocated(QUEUE));
        let w0 = lf_checker_rt::global::<u32>(W0).read();
        let w1 = lf_checker_rt::global::<u32>(W1).read();
        let w2 = lf_checker_rt::global::<u32>(W2).read();
        lf_checker_rt::callee_cdecl!(SUBMIT, u32, lf_checker_rt::relocated(BASE), w0, w1, w2, LEN);
        lf_checker_rt::global::<u32>(COUNT).write(0);
        lf_checker_rt::global::<u16>(FLAG).write(0);
        0
    }
});
