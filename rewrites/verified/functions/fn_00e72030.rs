// original: 0x00e72030 audio_shutdown_2030
/// Release the block at 0x0103ae88, clear two slots, set the ready byte.
///
/// Pushes the pointer from the global slot and calls the release callee
/// (cdecl/1, id 1), zeroes the slot and the dword after it, and writes 1
/// to the flag byte at 0x0103ae84. Returns 0 (the original zeroes EAX for
/// the stores and returns it). Takes no arguments (cdecl/0).
export!(cdecl, rw_00e72030() -> u32 {
    unsafe {
        const PTR_SLOT: u32 = 0x0103AE88;
        const NEXT_SLOT: u32 = 0x0103AE8C;
        const READY_FLAG: u32 = 0x0103AE84;
        let p = lf_checker_rt::global::<u32>(PTR_SLOT).read_unaligned();
        lf_checker_rt::callee_cdecl!(1, u32, p);
        lf_checker_rt::global::<u32>(PTR_SLOT).write_unaligned(0);
        lf_checker_rt::global::<u32>(NEXT_SLOT).write_unaligned(0);
        lf_checker_rt::global::<u8>(READY_FLAG).write(1);
        0
    }
});
