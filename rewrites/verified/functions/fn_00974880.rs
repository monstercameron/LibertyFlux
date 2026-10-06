// original: 0x00974880 audio_guarded_pair_forward (proposed)

/// Forward a value pair while a guard flag is set: raises the byte at
/// +0x110, calls the worker with the global word and the argument (pushed in
/// that order: the global is the first stack word), then lowers the flag.
/// Original: 0x00974880 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00974880(this: u32, arg: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x110;
        const GLOBAL: u32 = 0x11618FC;
        const WORKER: u32 = 1;
        ((this.wrapping_add(FLAG)) as *mut u8).write(1);
        let g = lf_checker_rt::global::<u32>(GLOBAL).read_unaligned();
        lf_checker_rt::callee_thiscall!(WORKER, u32, this, g, arg);
        ((this.wrapping_add(FLAG)) as *mut u8).write(0);
        0
    }
});
