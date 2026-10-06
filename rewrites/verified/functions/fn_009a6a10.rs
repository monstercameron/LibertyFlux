// original: 0x009A6A10 audio_setup_voice (proposed)

/// Sets up a voice slot when the mode, readiness and helpers all agree.
///
/// thiscall, two stack words (`arg0`, `level_bits`). Reads the mode
/// global at `MODE`: any mode but 2 or 4 (equality compares) returns the
/// mode at once, as does a non-zero readiness word at `READY` (+0x3AB8)
/// of `this`. Otherwise `level_bits` is offered to the level check
/// (callee 1, cdecl, one word): a zero low byte in its answer returns
/// that answer. Then the handle source (callee 2, cdecl, one zero word)
/// must answer non-null, and the status dword at `STATUS` (+0x1304) of
/// that handle must be zero, or their values are returned. When every
/// gate passes, the 17-word voice installer (callee 3, thiscall on
/// `this`) runs with (`arg0`, the readiness address, eight zero words,
/// 1, three zero words, -1, two zero words) and its answer is returned.
/// All comparisons are equalities; the float arg is forwarded by bits,
/// never computed on.
lf_checker_rt::export!(thiscall, rw_009a6a10(this: u32, arg0: u32, level_bits: u32) -> u32 {
    unsafe {
        const LEVEL_CHECK: u32 = 1;
        const HANDLE_SRC: u32 = 2;
        const INSTALLER: u32 = 3;
        const MODE: u32 = 0x00128463C;
        const READY: u32 = 0x3AB8;
        const STATUS: u32 = 0x1304;
        let mode = lf_checker_rt::global::<u32>(MODE).read_unaligned();
        if mode != 2 && mode != 4 {
            return mode;
        }
        if (this.wrapping_add(READY) as *const u32).read_unaligned() != 0 {
            return mode;
        }
        let check: u32 = lf_checker_rt::callee_cdecl!(LEVEL_CHECK, u32, level_bits);
        if (check as u8) == 0 {
            return check;
        }
        let handle: u32 = lf_checker_rt::callee_cdecl!(HANDLE_SRC, u32, 0);
        if handle == 0 {
            return 0;
        }
        let status = (handle.wrapping_add(STATUS) as *const u32).read_unaligned();
        if status != 0 {
            return handle;
        }
        lf_checker_rt::callee_thiscall!(
            INSTALLER, u32, this, arg0, this.wrapping_add(READY),
            0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0xFFFF_FFFFu32, 0, 0
        )
    }
});
