// original: 0x00A6CEA0 CTaskComplexPlayerPlaceCarBomb::CTaskComplexPlayerPlaceCarBomb

/// Initializes a task object, an embedded subobject, and an optional
/// reference field.
///
/// `this` is the object and `reference` is copied to `+0x14`. The base
/// constructor and embedded-subobject constructor are intercepted thiscalls.
/// If `reference` is nonzero, a third thiscall receives `reference` in ECX
/// and the address of the field at `this + 0x14` as its sole stack argument.
/// The vtable is installed at `+0`; `+0x18` (a 16-bit field) and `+0x40` are
/// cleared. The routine returns `this` and pops one stack argument.
lf_checker_rt::export!(thiscall, rw_00a6cea0(this: u32, reference: u32) -> u32 {
    unsafe {
        const BASE_CTOR: u32 = 1;
        const EMBEDDED_CTOR: u32 = 2;
        const ADD_REFERENCE: u32 = 3;
        const VTABLE: u32 = 0x00E9_F064;

        let _ = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((this + 0x18) as *mut u16).write_unaligned(0);
        let _ = lf_checker_rt::callee_thiscall!(EMBEDDED_CTOR, u32, this + 0x1C);
        ((this + 0x14) as *mut u32).write_unaligned(reference);
        ((this + 0x40) as *mut u32).write_unaligned(0);
        if reference != 0 {
            let _ = lf_checker_rt::callee_thiscall!(
                ADD_REFERENCE,
                u32,
                reference,
                this + 0x14,
            );
        }
        this
    }
});
