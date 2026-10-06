// original: 0x00D76760 render_phase_ctor_00d76760 (proposed)

/// Construct a render-phase object: base constructor plus vtable install.
///
/// Forwards `arg0` with `this` to the base constructor, then installs the vtable pointer and two slot constants (0, 2).
/// Returns `this`. Thiscall: object in `ecx`, one stack word.
use lf_checker_rt::{export, relocated};
use lf_checker_rt::callee_thiscall;

const BASE_CTOR: u32 = 1;

export!(thiscall, rw_00d76760(this: u32, arg0: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00eec4cc;
        const SLOT_A: u32 = 0x8d0;
        const SLOT_B: u32 = 0x8f4;
        callee_thiscall!(BASE_CTOR, u32, this, arg0);
        (this as *mut u32).write_unaligned(relocated(VTABLE));
        ((this + SLOT_A) as *mut u32).write_unaligned(0);
        ((this + SLOT_B) as *mut u32).write_unaligned(2);
        this
    }
});
