// original: 0x00CD9D70 CTaskSimpleBlendFromNM::CTaskSimpleBlendFromNM

/// Initialize the two-word `CTaskSimpleBlendFromNM` constructor form. After
/// the base and common task initializer calls, it stores the first argument
/// at offset 0x24, the second at offset 0x38, installs the derived vtable and
/// returns `this`.
///
/// Calling convention: thiscall with two 32-bit stack words. Both initializer
/// calls are thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00cd9d70(this: u32, type_value: u32, time_value: u32) -> u32 {
    const BASE_CONSTRUCTOR: u32 = 1;
    const COMMON_INITIALIZER: u32 = 2;
    const TYPE_FIELD: u32 = 0x24;
    const TIME_FIELD: u32 = 0x38;
    const VTABLE: u32 = 0x00ED_D68C;

    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let _ = lf_checker_rt::callee_thiscall!(COMMON_INITIALIZER, u32, this);
        ((this + TYPE_FIELD) as *mut u32).write_unaligned(type_value);
        ((this + TIME_FIELD) as *mut u32).write_unaligned(time_value);
        this
    }
});
