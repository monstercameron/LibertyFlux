// original: 0x00DA6AF0 task_vt_eefa04_ctor (proposed)

/// Constructor of the task class whose virtual table is 0x00EEFA04: run
/// the shared five-argument builder with the two caller floats, a zero, a
/// constant and a one, then install the table.
///
/// The builder receives (`a`, `b`, 0, HALF, 1) where HALF is the constant
/// 0.5 kept in the read-only data. Returns `this`.
/// Original: thiscall, two stack words, callee cleanup.
lf_checker_rt::export!(thiscall, rw_00da6af0(this: u32, a: u32, b: u32) -> u32 {
    unsafe {
        const BUILDER: u32 = 1;
        const VTABLE: u32 = 0x00EEFA04;
        const HALF_SLOT: u32 = 0x00ED7EE4;

        let half = (lf_checker_rt::relocated(HALF_SLOT) as *const u32).read();
        lf_checker_rt::callee_thiscall!(BUILDER, u32, this, a, b, 0, half, 1);
        (this as *mut u32).write(lf_checker_rt::relocated(VTABLE));
        this
    }
});
