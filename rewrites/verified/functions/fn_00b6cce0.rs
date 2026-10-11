// original: 0x00B6CCE0 heli_notify_field_e48

/// Invoke the attached-field update helper with `this + 0xe48`, read the
/// status byte at `this + 0x1f05`, and return false. The status comparison
/// does not affect the result; the thiscall has no stack arguments.
lf_checker_rt::export!(thiscall, rw_00B6CCE0(this: u32) -> u32 {
    unsafe {
        const ATTACHED_FIELD: u32 = 0xe48;
        const STATUS_BYTE: u32 = 0x1f05;
        const UPDATE_HELPER: u32 = 1;
        let field = this.wrapping_add(ATTACHED_FIELD);
        let _ = lf_checker_rt::callee_thiscall!(UPDATE_HELPER, u32, field);
        let _status = (this.wrapping_add(STATUS_BYTE) as *const u8).read_volatile();
        0
    }
});
