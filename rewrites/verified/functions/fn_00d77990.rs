// original: 0x00D77990 render_phase_ctor_link_args (proposed)

/// Construct a render-phase object, storing two link arguments.
///
/// Forwards `arg0` with `this` to the base constructor, stores `arg1` at
/// `+0x940` and `arg2` at `+0x3c`, then installs the vtable pointer and
/// two slot constants (0, 3). Returns `this`. Thiscall: object in `ecx`,
/// three stack words.
use lf_checker_rt::{export, relocated};
use lf_checker_rt::callee_thiscall;

const BASE_CTOR: u32 = 1;

export!(thiscall, rw_00d77990(this: u32, arg0: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00eec6c4;
        const LINK_OFF: u32 = 0x940;
        const AUX_OFF: u32 = 0x3c;
        const SLOT_A: u32 = 0x8d0;
        const SLOT_B: u32 = 0x8f4;
        callee_thiscall!(BASE_CTOR, u32, this, arg0);
        ((this + LINK_OFF) as *mut u32).write_unaligned(arg1);
        ((this + AUX_OFF) as *mut u32).write_unaligned(arg2);
        (this as *mut u32).write_unaligned(relocated(VTABLE));
        ((this + SLOT_A) as *mut u32).write_unaligned(0);
        ((this + SLOT_B) as *mut u32).write_unaligned(3);
        this
    }
});
