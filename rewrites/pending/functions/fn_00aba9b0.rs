// original: 0x00aba9b0 resolve_and_emit
/// Validate object `a0` and emit it through two callees with `a1`-`a6`.
///
/// Validates `a0` through vtable slot 0xD0, builds the row at
/// `answer+0x80` with callee 2 over `(a1..a6)`, revalidates twice, then
/// emits `(a0, second+0x80, first)` through callee 3, returning its
/// answer. All three validations go through the same fabricated vtable.
lf_checker_rt::export!(cdecl, rw_00aba9b0(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    // SAFETY: the object and vtable are checker-backed heap on every trial.
    let vtab = unsafe { (a0 as *const u32).read_unaligned() };
    let slot = unsafe { (vtab.wrapping_add(0xD0) as *const u32).read_unaligned() };
    let validate: extern "thiscall" fn(u32) -> u32 = unsafe { core::mem::transmute(slot as usize) };
    let v0 = validate(a0);
    let _ = lf_checker_rt::callee_thiscall!(2, u32, v0.wrapping_add(0x80), a1, a2, a3, a4, a5, a6);
    let v1 = validate(a0);
    let v2 = validate(a0);
    lf_checker_rt::callee_cdecl!(3, u32, a0, v2.wrapping_add(0x80), v1)
});
