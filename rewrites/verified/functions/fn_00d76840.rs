// original: 0x00D76840 render_phase_ctor_00d76840 (proposed)

/// Construct a render-phase object: base constructor plus vtable install.
///
/// Forwards `arg0` with `this` to the base constructor, then sets flag bit 1 at `+0x19`, installs the vtable pointer and three slot constants (0x232d20, 3, 2).
/// Returns `this`. Thiscall: object in `ecx`, one stack word.
use lf_checker_rt::{export, relocated};
use lf_checker_rt::callee_thiscall;

const BASE_CTOR: u32 = 1;

export!(thiscall, rw_00d76840(this: u32, arg0: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00eec3f4;
        const SLOT_A: u32 = 0x8d0;
        const SLOT_B: u32 = 0x8f4;
        const FLAG_OFF: u32 = 0x19;
        const FLAG_BIT: u8 = 2;
        const SLOT_A_VAL: u32 = 0x232d20;
        const SLOT_C: u32 = 0x40;
        callee_thiscall!(BASE_CTOR, u32, this, arg0);
        let f = ((this + FLAG_OFF) as *const u8).read();
        ((this + FLAG_OFF) as *mut u8).write(f | FLAG_BIT);
        (this as *mut u32).write_unaligned(relocated(VTABLE));
        ((this + SLOT_A) as *mut u32).write_unaligned(SLOT_A_VAL);
        ((this + SLOT_B) as *mut u32).write_unaligned(3);
        ((this + SLOT_C) as *mut u32).write_unaligned(2);
        this
    }
});
