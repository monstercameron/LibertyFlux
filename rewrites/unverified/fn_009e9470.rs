// original: 0x009e9470 peds_tasks_init_all
/// Initialises the peds-tasks subsystems: runs the four subsystem
/// initialisers in order and publishes the ready flags (byte 1, words
/// -1/0/0). Returns the last initialiser's answer. (cdecl, no args.)
lf_checker_rt::export!(cdecl, rw_009e9470() -> u32 {
    unsafe {
        const READY_BYTE: u32 = 0x1046AC5;
        const READY_A: u32 = 0x103B1B0;
        const READY_B: u32 = 0x166838C;
        const READY_C: u32 = 0x16660F0;
        let _: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        let _: u32 = lf_checker_rt::callee_cdecl!(2, u32,);
        lf_checker_rt::global::<u8>(READY_BYTE).write(1);
        let _: u32 = lf_checker_rt::callee_cdecl!(3, u32,);
        let ans: u32 = lf_checker_rt::callee_cdecl!(4, u32,);
        lf_checker_rt::global::<u32>(READY_A).write_unaligned(0xFFFFFFFF);
        lf_checker_rt::global::<u32>(READY_B).write_unaligned(0);
        lf_checker_rt::global::<u32>(READY_C).write_unaligned(0);
        ans
    }
});
