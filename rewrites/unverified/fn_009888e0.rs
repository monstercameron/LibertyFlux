// original: 0x009888E0 audCutscene_wait_ready (proposed)

/// Cutscene readiness wait: polls the acknowledge callee until it answers
/// nonzero, pumping the audio tasks between polls.
///
/// Takes two ids (`first`, `second`). Remembers the flag byte at `FLAG`, sets
/// it to 0, then polls the acknowledge callee (id 1, thiscall on the shared
/// object at `SHARED_ENTITY` with `(first, second)`): while it answers zero,
/// each round reads the sequence value from `SEQ`, runs the three task
/// callees (ids 2-4, cdecl) and the manager poke (id 5, thiscall on the object
/// read from `MANAGER_PTR`) on it, then the delay callee (id 6, cdecl) with
/// `DELAY_ARG`, and polls again. Returns the flag byte found at the end, or 1
/// when the remembered flag was nonzero, and stores the answer back to `FLAG`.
/// Original: stdcall, two stack words, callee pops 8, full `eax` returned.
lf_checker_rt::export!(stdcall, rw_009888E0(first: u32, second: u32) -> u32 {
    const FLAG: u32 = 0x115dbfc;
    const SHARED_ENTITY: u32 = 0x1282fa8;
    const SEQ: u32 = 0x11735b4;
    const MANAGER_PTR: u32 = 0x115f848;
    const DELAY_ARG: u32 = 0x64;
    const ACK: u32 = 1;
    const TASK_A: u32 = 2;
    const TASK_B: u32 = 3;
    const TASK_C: u32 = 4;
    const POKE: u32 = 5;
    const DELAY: u32 = 6;
    unsafe {
        let flag = lf_checker_rt::global::<u8>(FLAG);
        let saved = flag.read();
        flag.write(0);
        let ent = lf_checker_rt::relocated(SHARED_ENTITY);
        let mut ans: u32 = lf_checker_rt::callee_thiscall!(ACK, u32, ent, first, second);
        while (ans as u8) == 0 {
            let seq = *lf_checker_rt::global::<u32>(SEQ);
            let _: u32 = lf_checker_rt::callee_cdecl!(TASK_A, u32, seq);
            let _: u32 = lf_checker_rt::callee_cdecl!(TASK_B, u32, seq);
            let _: u32 = lf_checker_rt::callee_cdecl!(TASK_C, u32, seq);
            let mgr = *lf_checker_rt::global::<u32>(MANAGER_PTR);
            let _: u32 = lf_checker_rt::callee_thiscall!(POKE, u32, mgr, seq);
            let _: u32 = lf_checker_rt::callee_cdecl!(DELAY, u32, DELAY_ARG);
            ans = lf_checker_rt::callee_thiscall!(ACK, u32, ent, first, second);
        }
        let mut out = flag.read() as u32;
        if saved != 0 {
            out = 1;
        }
        flag.write(out as u8);
        out
    }
});
