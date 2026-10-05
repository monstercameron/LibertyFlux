// original: 0x009427d0 streaming_system_reset (proposed)

/// Reset the streaming globals, then re-initialise through two workers.
///
/// Copies the mode word into the state word, clears the state words and
/// flag bytes to idle values, runs the two initialisation workers (cdecl,
/// no arguments) and tail-jumps to the finishing routine, whose result is
/// returned.
///
/// Original: 0x009427d0 (cdecl, no arguments; ends in a tail jump).
lf_checker_rt::export!(cdecl, rw_009427d0() -> u32 {
    unsafe {
        const MODE: u32 = 0x011D6FC8;
        const STATE: u32 = 0x011D6FC4;
        const W0: u32 = 0x011D6FA0;
        const W1: u32 = 0x011D6FA4;
        const W2: u32 = 0x011D6FBC;
        const W3: u32 = 0x011D6FC0;
        const W4: u32 = 0x011D6FA8;
        const W5: u32 = 0x011D6FAC;
        const FLAG_A: u32 = 0x011D6FB7;
        const FLAG_B: u32 = 0x011D6FCC;
        const FLAG_C: u32 = 0x011D6FCD;
        const INIT_A: u32 = 1;
        const INIT_B: u32 = 2;
        const FINISH: u32 = 3;
        let mode = lf_checker_rt::global::<u32>(MODE).read();
        lf_checker_rt::global::<u8>(FLAG_A).write(0);
        lf_checker_rt::global::<u32>(W0).write(0);
        lf_checker_rt::global::<u32>(W1).write(0);
        lf_checker_rt::global::<u32>(W2).write(0);
        lf_checker_rt::global::<u32>(W3).write(0);
        lf_checker_rt::global::<u32>(W4).write(0);
        lf_checker_rt::global::<u32>(W5).write(0);
        lf_checker_rt::global::<u32>(STATE).write(mode);
        lf_checker_rt::global::<u8>(FLAG_B).write(0);
        lf_checker_rt::global::<u8>(FLAG_C).write(0);
        let _: u32 = lf_checker_rt::callee_cdecl!(INIT_A, u32,);
        let _: u32 = lf_checker_rt::callee_cdecl!(INIT_B, u32,);
        lf_checker_rt::callee_cdecl!(FINISH, u32,)
    }
});
