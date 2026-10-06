// original: 0x0094E7B0 link_child_objects (proposed)

/// Link two looked-up objects, flag the parent, and run three callees.
///
/// Resolves `a` and `b` through callee 1 (thiscall, the context word `CTX`
/// in ECX, the key on the stack) into `left` and `right`, and stores
/// `right` at `left + LINK_OFF`. Then sets bit 30 of the word at
/// `left + FLAG_OFF` to 1 (`right` is never null in the contract; the
/// original's null path faults and is never taken): the rewrite sets the
/// bit directly where the original flips it with xor-and-mask, which is
/// the same value. Bumps the byte at `right + USE_OFF`, runs callee 2
/// with (`right`, 0), callee 3 as a thiscall on `right`, writes 2 to
/// `right + KIND_OFF`, and returns callee 4's answer for (`right`, 0).
///
/// Original: 0x0094E7B0 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_0094E7B0(a: u32, b: u32) -> u32 {
    unsafe {
        const CTX: u32 = 0x1632C60;
        const LINK_OFF: u32 = 0x4C;
        const FLAG_OFF: u32 = 0x24;
        const FLAG_BIT: u32 = 30;
        const USE_OFF: u32 = 0x61;
        const KIND_OFF: u32 = 0x41;
        const LOOKUP: u32 = 1;
        const PREP: u32 = 2;
        const ACTIVATE: u32 = 3;
        const EMIT: u32 = 4;
        let ctx = (lf_checker_rt::global::<u32>(CTX) as *const u32).read();
        let left = lf_checker_rt::callee_thiscall!(LOOKUP, u32, ctx, a);
        let right = lf_checker_rt::callee_thiscall!(LOOKUP, u32, ctx, b);
        (left.wrapping_add(LINK_OFF) as *mut u32).write_unaligned(right);
        let old = (left.wrapping_add(FLAG_OFF) as *const u32).read_unaligned();
        let newbit = if right != 0 { 1u32 } else { (old >> FLAG_BIT) & 1 };
        (left.wrapping_add(FLAG_OFF) as *mut u32)
            .write_unaligned((old & !(1 << FLAG_BIT)) | (newbit << FLAG_BIT));
        let rc = (right.wrapping_add(USE_OFF) as *const u8).read_unaligned();
        (right.wrapping_add(USE_OFF) as *mut u8).write(rc.wrapping_add(1));
        lf_checker_rt::callee_cdecl!(PREP, u32, right, 0);
        lf_checker_rt::callee_thiscall!(ACTIVATE, u32, right);
        (right.wrapping_add(KIND_OFF) as *mut u8).write(2);
        lf_checker_rt::callee_cdecl!(EMIT, u32, right, 0)
    }
});
