// original: 0x005BBB70 apply_streaming_mode_updates
/// For each of two global state bytes whose low bit is clear, call the
/// streaming update helper with the corresponding dword from the shared
/// object. Then set bit 6 in the shared flags word while preserving every
/// other bit. The original is a no-argument cdecl routine; helper calls are
/// direct thiscall calls with one 32-bit stack argument.
lf_checker_rt::export!(cdecl, rw_005bbb70() -> u32 {
    const OBJECT_PTR_VA: u32 = 0x018b_6eb0;
    const FLAGS_VA: u32 = 0x018b_6ea4;
    const FIRST_GATE_VA: u32 = 0x019d_302e;
    const SECOND_GATE_VA: u32 = 0x019d_3022;
    const FIRST_TARGET_VA: u32 = 0x019d_3024;
    const SECOND_TARGET_VA: u32 = 0x019d_3018;
    const FIRST_FIELD_OFFSET: u32 = 0x18;
    const SECOND_FIELD_OFFSET: u32 = 0x1c;
    const UPDATE_HELPER_ID: u32 = 1;
    const UPDATED_FLAG: u32 = 0x40;

    let object = unsafe { lf_checker_rt::global::<u32>(OBJECT_PTR_VA).read() };
    let first_gate = unsafe { lf_checker_rt::global::<u8>(FIRST_GATE_VA).read() };
    if first_gate & 1 == 0 {
        let field = unsafe {
            core::ptr::read_unaligned(object.wrapping_add(FIRST_FIELD_OFFSET) as *const u32)
        };
        let this_pointer = lf_checker_rt::relocated(FIRST_TARGET_VA) as u32;
        lf_checker_rt::callee_thiscall!(UPDATE_HELPER_ID, u32, this_pointer, field);
    }

    let second_gate = unsafe { lf_checker_rt::global::<u8>(SECOND_GATE_VA).read() };
    if second_gate & 1 == 0 {
        let field = unsafe {
            core::ptr::read_unaligned(object.wrapping_add(SECOND_FIELD_OFFSET) as *const u32)
        };
        let this_pointer = lf_checker_rt::relocated(SECOND_TARGET_VA) as u32;
        lf_checker_rt::callee_thiscall!(UPDATE_HELPER_ID, u32, this_pointer, field);
    }

    unsafe {
        let flags = lf_checker_rt::global::<u32>(FLAGS_VA);
        flags.write(flags.read() | UPDATED_FLAG);
    }
    0
});
