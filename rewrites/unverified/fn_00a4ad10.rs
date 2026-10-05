// original: 0x00A4AD10 vehicle_call_with_global_this (proposed)

/// Forwards this object and an address constant to a helper called with a
/// global object.
///
/// Loads the callee's `this` from the global at `GLOBAL_THIS`, then calls the
/// helper with `this` in `ecx`, stack arguments the relocated address `ADDR`
/// (file VA 0x401690: the original's pushed immediate carries a relocation,
/// so it arrives relocated) and the caller's own `this`, and returns the
/// helper's answer. The incoming stack word is ignored. The argument order on
/// the stack (address first) matches the original's push order.
///
/// Original: 0x00A4AD10 (thiscall, one ignored stack word), one callee.
lf_checker_rt::export!(thiscall, rw_00A4AD10(this: u32, _unused: u32) -> u32 {
    unsafe {
        const GLOBAL_THIS: u32 = 0x018B6F1C;
        const ADDR_FILE_VA: u32 = 0x00401690;
        const CALLEE: u32 = 1;
        let gthis = lf_checker_rt::global::<u32>(GLOBAL_THIS).read_unaligned();
        let addr = lf_checker_rt::relocated(ADDR_FILE_VA);
        lf_checker_rt::callee_thiscall!(CALLEE, u32, gthis, addr, this)
    }
});
