// original: 0x00D01290 CTaskComplexStandGuard::vf1

/// Clone a stand-guard task through the task manager.
///
/// Null from the manager (patched callee 1) means no clone. Otherwise forwards
/// the guard descriptor to the clone routine (patched thiscall callee): a
/// pointer to the five-word coordinate frame, the stance word twice, the
/// radius word, the sign-extended 3-bit mode from `+0x90`, and one word the
/// original reads from its own uninitialised frame (skipped in the contract).
/// Returns the clone answer.
///
/// Original: 0x00D01290 (thiscall, no stack words; aligns its frame).
lf_checker_rt::export!(thiscall, rw_00d01290(this: u32) -> u32 {
    unsafe {
        const ALLOC: u32 = 1;
        const CLONE: u32 = 2;
        const TASK_MANAGER_SLOT: u32 = 0x0167E2A0;
        let f74 = ((this + 0x74) as *const u32).read_unaligned();
        let f68 = ((this + 0x68) as *const u32).read_unaligned();
        let f60 = ((this + 0x60) as *const u32).read_unaligned();
        let f24 = ((this + 0x24) as *const u32).read_unaligned();
        let f28 = ((this + 0x28) as *const u32).read_unaligned();
        let stance = ((this + 0x90) as *const u32).read_unaligned();
        let manager = lf_checker_rt::global::<u32>(TASK_MANAGER_SLOT).read();
        let target = lf_checker_rt::callee_thiscall!(ALLOC, u32, manager);
        if target == 0 {
            return 0;
        }
        let mode = (((stance as i32) << 29) >> 29) as u32;
        let frame: [u32; 5] = [f74, f68, f60, f24, f28];
        let at = frame.as_ptr() as u32 + 8;
        lf_checker_rt::callee_thiscall!(CLONE, u32, target, at, stance, f74, mode, 0, stance)
    }
});
