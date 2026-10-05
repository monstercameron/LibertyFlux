// original: 0x00c80110 scenario_ped_link (proposed)

/// Link a scenario ped into the task graph through four object calls.
///
/// `a1` points at the scenario record (object pointer at `+0x224`), `a2` is an
/// opaque value forwarded to the third and last calls. Issues, in order:
/// `c1(obj, 1)` and keeps its result as `h`; `c2(h)` whose result is ignored;
/// `c3(obj, a2, 0)` whose result is ignored; and returns `c4(h, a2)`.
/// Straight-line code: all four calls fire on every run. The object pointers
/// come from the record, so both sides pass identical addresses.
///
/// Original: stdcall, two stack words (the callee pops 8 bytes).
lf_checker_rt::export!(stdcall, rw_00c80110(a1: u32, a2: u32) -> u32 {
    unsafe {
        const OBJ_OFF: u32 = 0x224;
        const C1: u32 = 1;
        const C2: u32 = 2;
        const C3: u32 = 3;
        const C4: u32 = 4;
        let obj = ((a1 + OBJ_OFF) as *const u32).read_unaligned();
        let h: u32 = lf_checker_rt::callee_thiscall!(C1, u32, obj, 1u32);
        lf_checker_rt::callee_thiscall!(C2, u32, h);
        lf_checker_rt::callee_thiscall!(C3, u32, obj, a2, 0u32);
        lf_checker_rt::callee_thiscall!(C4, u32, h, a2)
    }
});
