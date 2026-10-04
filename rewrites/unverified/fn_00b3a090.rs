// original: 0x00b3a090 task_scaled_pick (proposed)

/// Pick a task-scaled value: sum the two seed globals, ask the scaler callee
/// for a candidate, and keep the sum when it is ordered-below the candidate
/// (signed); otherwise ask again and keep the second answer. Subtract the
/// base global and return the result (wrapping). The callee takes no
/// arguments. Original: 0x00b3a090 (cdecl, no stack arguments).
lf_checker_rt::export!(cdecl, rw_00b3a090() -> u32 {
    unsafe {
        const SEED_A: u32 = 0x0169e408;
        const SEED_B: u32 = 0x0169e40c;
        const BASE: u32 = 0x016624c0;
        const SCALER: u32 = 1;
        let mut picked = lf_checker_rt::global::<u32>(SEED_A)
            .read()
            .wrapping_add(lf_checker_rt::global::<u32>(SEED_B).read());
        let first: u32 = lf_checker_rt::callee_cdecl!(SCALER, u32,);
        if !((picked as i32) < (first as i32)) {
            picked = lf_checker_rt::callee_cdecl!(SCALER, u32,);
        }
        picked.wrapping_sub(lf_checker_rt::global::<u32>(BASE).read())
    }
});
