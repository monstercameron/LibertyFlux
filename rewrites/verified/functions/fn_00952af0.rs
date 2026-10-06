// original: 0x00952AF0 vtable_probe_value (proposed)

/// Call a vtable slot, probe its answer, and add a conditional bonus.
///
/// Calls the function at slot 0xA0 of `obj`'s table (callee 1, thiscall
/// with `obj` in ECX; the contract plants the stub address in the slot and
/// the rewrite calls the stub directly — the two pointer reads are
/// unobservable either way). A null answer returns 0. Otherwise callee 2
/// probes the answer (thiscall, no stack words) and callee 3 values `obj`
/// (one stack word); the return is callee 3's answer plus 0x18 when callee
/// 2's answer is non-zero (full-word test), else unchanged.
///
/// Original: 0x00952AF0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00952AF0(obj: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 1;
        const PROBE: u32 = 2;
        const VALUE: u32 = 3;
        const BONUS: u32 = 0x18;
        let first = lf_checker_rt::callee_thiscall!(SLOT, u32, obj);
        if first == 0 {
            0
        } else {
            let r = lf_checker_rt::callee_thiscall!(PROBE, u32, first);
            let v = lf_checker_rt::callee_cdecl!(VALUE, u32, obj);
            v.wrapping_add(if r != 0 { BONUS } else { 0 })
        }
    }
});
