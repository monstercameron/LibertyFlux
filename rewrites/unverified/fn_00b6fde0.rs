// original: 0x00b6fde0 release_slot_cond_call_init (proposed)
/// Mark a slot done, then conditionally initialise-and-release its child.
///
/// Takes two stack words; the first is ignored and the second (`s`) points to
/// the slot record. Byte at `s+0x14` is set to 1 first. When the child dword
/// at `s+0x18` is null the function returns at once (EAX keeps its entry
/// value, so the contract compares no return channel). Otherwise, when bit 15
/// of the dword at `c+4` is set, the initialiser (callee 1, thiscall/1: `c`
/// in ECX, the float -8.0) runs and `s+0x18` is cleared to 0.
///
/// Original: 0x00b6fde0 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00b6fde0(_ignored: u32, s: u32) -> u32 {
    unsafe {
        const CHILD_OFF: u32 = 0x18;
        const DONE_OFF: u32 = 0x14;
        const FLAGS_OFF: u32 = 4;
        const TEST_SHIFT: u32 = 15;
        const INIT_VALUE: u32 = 0xc1000000; // -8.0f
        let c = ((s + CHILD_OFF) as *const u32).read_unaligned();
        ((s + DONE_OFF) as *mut u8).write(1);
        if c == 0 {
            return 0;
        }
        let flags = ((c + FLAGS_OFF) as *const u32).read_unaligned();
        if ((flags >> TEST_SHIFT) & 1) != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, c, INIT_VALUE);
            ((s + CHILD_OFF) as *mut u32).write_unaligned(0);
        }
        0
    }
});

