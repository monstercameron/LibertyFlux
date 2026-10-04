// original: 0x00bc1190 ped_flag_bit24 (proposed)

/// Set bit 24 of a ped's flag word from a flag byte, or allocate a task.
///
/// Calls the gate (cdecl, no arguments): when its low byte is non-zero the
/// function returns that answer unchanged. With a null handle it allocates a
/// kind-0x4E task instead: the allocator from `G_ALLOC` (thiscall, no stack
/// arguments), initialised on success (ecx-only call), stamped with vtable
/// `0xEB7424` and the low byte of `arg1` at `+0x14`, then `(0, object, 0x4E)`
/// reported to the result sink (cdecl, three words); a failed allocation
/// reports `(0, null, 0x4E)`. With a non-null handle the ped is looked up
/// through the ped manager (`G_PEDMGR`, thiscall with the handle) and bit 24
/// of the word at `+0x260` is set to bit 0 of the low byte of `arg1` by an
/// exclusive-or mask, leaving other bits unchanged; the ped pointer is
/// returned.
///
/// Original: 0x00BC1190 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00bc1190(handle: u32, arg1: u32) -> u32 {
    const G_ALLOC: u32 = 0x0167E2A0;
    const G_PEDMGR: u32 = 0x018B6F1C;
    const OFF_FLAGS: u32 = 0x260;
    const BIT: u32 = 0x0100_0000;
    const VTABLE: u32 = 0x00EB7424;
    const KIND: u32 = 0x4E;
    let gate: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
    if gate & 0xFF != 0 {
        return gate;
    }
    if handle == 0 {
        let alloc: u32 =
            unsafe { (lf_checker_rt::relocated(G_ALLOC) as *const u32).read_unaligned() };
        let obj: u32 = lf_checker_rt::callee_thiscall!(3, u32, alloc);
        if obj == 0 {
            return lf_checker_rt::callee_cdecl!(5, u32, 0, 0, KIND);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, obj);
        unsafe {
            (obj as *mut u32).write_unaligned(VTABLE);
            ((obj + 0x14) as *mut u8).write((arg1 & 0xFF) as u8);
        }
        return lf_checker_rt::callee_cdecl!(5, u32, 0, obj, KIND);
    }
    let mgr: u32 =
        unsafe { (lf_checker_rt::relocated(G_PEDMGR) as *const u32).read_unaligned() };
    let ped: u32 = lf_checker_rt::callee_thiscall!(2, u32, mgr, handle);
    unsafe {
        let slot: *mut u32 = ((ped + OFF_FLAGS) as *mut u32);
        let mem: u32 = slot.read_unaligned();
        let mask: u32 = (((arg1 & 0xFF) << 24) ^ mem) & BIT;
        slot.write_unaligned(mem ^ mask);
    }
    ped
});
