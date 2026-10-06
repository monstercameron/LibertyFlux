// original: 0x00955C10 emit_pool_entry_4 (proposed)

/// Emit one queued entry into the shared pool buffer (family member 4 of 5).
///
/// Returns immediately unless the enable byte is set, the disable byte is
/// clear, the version dword matches the read-only reference, and (when the
/// alt-mode byte is set) the alt value is 2. Returns when the entry id at
/// `ID_OFF` (0x64) in the record is `EMPTY` (-1). When the mode dword is
/// nonzero, marks `AUX_OFF` (0x68) and the id `EMPTY` and returns.
/// Otherwise, when `used + 8` exceeds `POOL_LIMIT` (UNSIGNED 32-bit
/// comparison) it calls the grow callee and returns if that answers 0;
/// then emits the id through the member-specific thiscall callee with
/// `this = used + pool`, advances `used` by 8, zero-terminates the pool
/// at the new end, calls the done callee with (0, id, record) and marks
/// the id `EMPTY`. This member's emit callee is at file VA 0x00BEEA60.
/// Original is cdecl/1, no return value.
lf_checker_rt::export!(cdecl, rw_00955C10(rec: u32) -> u32 {
    const G_ENABLED: u32 = 0x010376E9;
    const G_DISABLED: u32 = 0x011F7076;
    const G_VERSION: u32 = 0x012088B4;
    const G_ROM_VERSION: u32 = 0x00F1C040;
    const G_ALT_MODE: u32 = 0x01037868;
    const G_ALT_VALUE: u32 = 0x011F6FFA;
    const G_MODE: u32 = 0x011F7060;
    const G_USED: u32 = 0x0120F290;
    const G_POOL: u32 = 0x0120F294;
    const POOL_LIMIT: u32 = 0x895430;
    const ID_OFF: u32 = 0x64;
    const AUX_OFF: u32 = 0x68;
    const EMPTY: u32 = 0xFFFFFFFF;
    const C_GROW: u32 = 1;
    const C_EMIT: u32 = 2;
    const C_DONE: u32 = 3;
    if unsafe { lf_checker_rt::global::<u8>(G_ENABLED).read() } == 0 {
        return 0;
    }
    if unsafe { lf_checker_rt::global::<u8>(G_DISABLED).read() } != 0 {
        return 0;
    }
    let version = unsafe { lf_checker_rt::global::<u32>(G_VERSION).read() };
    let rom = unsafe { lf_checker_rt::global::<u32>(G_ROM_VERSION).read() };
    if version != rom {
        return 0;
    }
    if unsafe { lf_checker_rt::global::<u8>(G_ALT_MODE).read() } != 0
        && unsafe { lf_checker_rt::global::<u8>(G_ALT_VALUE).read() } != 2
    {
        return 0;
    }
    let id = unsafe { (rec.wrapping_add(ID_OFF) as *const u32).read_unaligned() };
    if id == EMPTY {
        return 0;
    }
    if unsafe { lf_checker_rt::global::<u32>(G_MODE).read() } != 0 {
        unsafe { (rec.wrapping_add(AUX_OFF) as *mut u32).write_unaligned(EMPTY) };
        unsafe { (rec.wrapping_add(ID_OFF) as *mut u32).write_unaligned(EMPTY) };
        return 0;
    }
    let used = unsafe { lf_checker_rt::global::<u32>(G_USED).read() };
    if used.wrapping_add(8) > POOL_LIMIT {
        let ok = lf_checker_rt::callee_cdecl!(C_GROW, u32,);
        if (ok as u8) == 0 {
            return 0;
        }
    }
    let used2 = unsafe { lf_checker_rt::global::<u32>(G_USED).read() };
    let pool = unsafe { lf_checker_rt::global::<u32>(G_POOL).read() };
    lf_checker_rt::callee_thiscall!(C_EMIT, u32, used2.wrapping_add(pool), id);
    let used3 = unsafe { lf_checker_rt::global::<u32>(G_USED).read() };
    let pool3 = unsafe { lf_checker_rt::global::<u32>(G_POOL).read() };
    let new_used = used3.wrapping_add(8);
    unsafe { lf_checker_rt::global::<u32>(G_USED).write(new_used) };
    unsafe { (pool3.wrapping_add(new_used) as *mut u8).write(0) };
    lf_checker_rt::callee_cdecl!(C_DONE, u32, 0, id, rec);
    unsafe { (rec.wrapping_add(ID_OFF) as *mut u32).write_unaligned(EMPTY) };
    0
});
