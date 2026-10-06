// original: 0x00872BA0 parent_dtor_body_two_pass
/// Destructor body of a parent motion-tree node: two teardown passes.
///
/// First pass: stamp the parent table, run the direct child-list pass,
/// then the direct teardown helper, and clear the link words at `+0x0C`
/// and `+0x10` (plus `+8` when non-null). Second pass: stamp the node
/// table and repeat the teardown helper with the same link clearing.
/// No return value.
///
/// Original: 0x00872BA0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00872BA0(this: u32) -> u32 {
    unsafe {
        const PARENT_VTABLE: u32 = 0xFE7F64;
        const NODE_VTABLE: u32 = 0xFE7F28;
        const LIST_CALLEE: u32 = 1;
        const TEARDOWN_CALLEE: u32 = 2;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(PARENT_VTABLE));
        lf_checker_rt::callee_thiscall!(LIST_CALLEE, u32, this);
        lf_checker_rt::callee_thiscall!(TEARDOWN_CALLEE, u32, this);
        // Link words: `+0x0C` and `+0x10` always, `+8` when set.
        let has_link = ((this + 8) as *const u32).read_unaligned() != 0;
        ((this + 0x0C) as *mut u32).write_unaligned(0);
        ((this + 0x10) as *mut u32).write_unaligned(0);
        if has_link {
            ((this + 8) as *mut u32).write_unaligned(0);
        }
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(NODE_VTABLE));
        lf_checker_rt::callee_thiscall!(TEARDOWN_CALLEE, u32, this);
        let has_link = ((this + 8) as *const u32).read_unaligned() != 0;
        ((this + 0x0C) as *mut u32).write_unaligned(0);
        ((this + 0x10) as *mut u32).write_unaligned(0);
        if has_link {
            ((this + 8) as *mut u32).write_unaligned(0);
        }
    }
    0
});
