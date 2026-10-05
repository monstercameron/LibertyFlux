// original: 0x00a935c0 stream_check_bit_set

/// Sets a bit for a record that passes its check.
///
/// Calls the check (callee 1, thiscall) with (`b+0x30`, `a`); a zero low
/// answer byte ends the call and is returned. Otherwise registers `b`
/// (callee 2, thiscall) with the manager at file VA 0x12FB258, whose
/// answer indexes the bit array at file VA 0x12FB26C, and sets bit 0 of
/// that byte. Returns the register answer. Two calls.
/// Original: 0x00A935C0 (cdecl, two stack words), 45 bytes.
lf_checker_rt::export!(cdecl, rw_00a935c0(a: u32, b: u32) -> u32 {
    unsafe {
        const MGR: u32 = 0x12FB258;
        const BITS: u32 = 0x12FB26C;
        const SUB_OFF: u32 = 0x30;
        const CHECK: u32 = 1;
        const REGISTER: u32 = 2;
        let ok: u32 =
            lf_checker_rt::callee_thiscall!(CHECK, u32, b.wrapping_add(SUB_OFF), a);
        if (ok & 0xFF) == 0 {
            return ok;
        }
        let mgr = lf_checker_rt::global::<u32>(MGR).read();
        let idx: u32 = lf_checker_rt::callee_thiscall!(REGISTER, u32, mgr, b);
        let base = lf_checker_rt::global::<u32>(BITS).read();
        let cell = base.wrapping_add(idx) as *mut u8;
        cell.write(cell.read() | 1);
        idx
    }
});
