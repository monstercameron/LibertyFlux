// original: 0x00b6fda0 cond_init_then_release_slot (proposed)
/// Conditionally initialise a slot, then release it.
///
/// Takes two stack words; the first is ignored and the second (`s`) points to
/// the slot record. `c` is the dword at `s+0x18`. When bit 15 of the dword at
/// `c+4` is clear, the initialiser (callee 1, thiscall/1: `c` in ECX, the
/// float -4.0) runs and bit 14 of `c+4` is set. Either way the slot is then
/// released: byte at `s+0x14` becomes 1 and `s+0x18` becomes 0.
///
/// Returns `c` on the initialise path and the shifted flag word on the skip
/// path; both are deterministic, so the contract compares EAX.
///
/// Original: 0x00b6fda0 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00b6fda0(_ignored: u32, s: u32) -> u32 {
    unsafe {
        const CHILD_OFF: u32 = 0x18;
        const DONE_OFF: u32 = 0x14;
        const FLAGS_OFF: u32 = 4;
        const TEST_SHIFT: u32 = 15;
        const SET_BIT: u32 = 0x4000;
        const INIT_VALUE: u32 = 0xc0800000; // -4.0f
        let c = ((s + CHILD_OFF) as *const u32).read_unaligned();
        let flags = ((c + FLAGS_OFF) as *const u32).read_unaligned();
        if ((flags >> TEST_SHIFT) & 1) == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, c, INIT_VALUE);
            let p = (c + FLAGS_OFF) as *mut u32;
            p.write_unaligned(p.read_unaligned() | SET_BIT);
            ((s + DONE_OFF) as *mut u8).write(1);
            ((s + CHILD_OFF) as *mut u32).write_unaligned(0);
            return c;
        }
        ((s + DONE_OFF) as *mut u8).write(1);
        ((s + CHILD_OFF) as *mut u32).write_unaligned(0);
        flags >> TEST_SHIFT
    }
});

