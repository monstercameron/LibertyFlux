// original: 0x00954540 pool_fit_check (proposed)

/// Check that a pooled allocation still fits its block.
///
/// Returns 0 unless the mode global is 1, the flag byte is 0 and the
/// state dword differs from `BUSY` (0x12). Otherwise takes a base size
/// from callee 1, resolves an entry through the thiscall callee 2 and a
/// size through callee 3 (or `EMPTY_SIZE` (0x21) when the alt global is
/// 0), and returns whether `size + base` (wrapping) reaches the limit
/// dword at entry+0x34 (UNSIGNED comparison: 0 only when strictly
/// below). Original is cdecl/0, returns AL.
lf_checker_rt::export!(cdecl, rw_00954540() -> u32 {
    const G_MODE: u32 = 0x011F7060;
    const G_FLAG: u32 = 0x01037758;
    const G_STATE: u32 = 0x01037720;
    const G_ARG: u32 = 0x011F6F34;
    const G_THIS: u32 = 0x011F6954;
    const G_ALT: u32 = 0x011F70CC;
    const BUSY: u32 = 0x12;
    const EMPTY_SIZE: u32 = 0x21;
    const LIMIT_OFF: u32 = 0x34;
    const C_BASE: u32 = 1;
    const C_ENTRY: u32 = 2;
    const C_SIZE: u32 = 3;
    if unsafe { lf_checker_rt::global::<u32>(G_MODE).read() } != 1 {
        return 0;
    }
    if unsafe { lf_checker_rt::global::<u8>(G_FLAG).read() } != 0 {
        return 0;
    }
    if unsafe { lf_checker_rt::global::<u32>(G_STATE).read() } == BUSY {
        return 0;
    }
    let base = lf_checker_rt::callee_cdecl!(C_BASE, u32,);
    let arg = unsafe { lf_checker_rt::global::<u32>(G_ARG).read() };
    let this = unsafe { lf_checker_rt::global::<u32>(G_THIS).read() };
    let entry = lf_checker_rt::callee_thiscall!(C_ENTRY, u32, this, arg);
    let alt = unsafe { lf_checker_rt::global::<u32>(G_ALT).read() };
    let v = lf_checker_rt::callee_cdecl!(C_SIZE, u32, alt);
    let size = if alt == 0 { EMPTY_SIZE } else { v };
    let sum = size.wrapping_add(base);
    let limit = unsafe { (entry.wrapping_add(LIMIT_OFF) as *const u32).read_unaligned() };
    (sum >= limit) as u32
});
