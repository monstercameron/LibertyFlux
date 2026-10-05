// original: 0x00c49130 ccamcinematic_push_17 (proposed)
/// Append a kind-0x17 entry to the table at `this + TABLE`.
///
/// Writes `KIND` at index `this + COUNT`, bumps the count, then
/// notifies the owner (callee 1). Returns 1 in the low byte.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c49130(this: u32) -> u32 {
    const TABLE: u32 = 0x140;
    const COUNT: u32 = 0x200;
    const KIND: u32 = 0x17;
    const NOTIFY: u32 = 1;
    unsafe {
        let n = ((this + COUNT) as *const u32).read_unaligned();
        ((this + TABLE + n.wrapping_mul(4)) as *mut u32).write_unaligned(KIND);
        ((this + COUNT) as *mut u32).write_unaligned(n.wrapping_add(1));
        lf_checker_rt::callee_thiscall!(NOTIFY, u32, this);
    }
    1
});
