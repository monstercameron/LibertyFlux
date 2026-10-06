// original: 0x00D76650 touch_five_slots_resolve_named (proposed)

/// Touch the five slot headers, then resolve and register the name.
///
/// Calls the slot helper on `this + 0, 4, 8, 0xc, 0x10`, resolves the
/// shared name, and unless the answer is -1 registers it; clears the
/// byte at `+0x49`. Returns the register answer, or -1 when skipped.
/// Thiscall: object in `ecx`, no stack words.
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, relocated};

const TOUCH: u32 = 1;
const RESOLVE: u32 = 2;
const REGISTER: u32 = 3;

export!(thiscall, rw_00d76650(this: u32) -> u32 {
    unsafe {
        const NAME_REF: u32 = 0x00eebd58;
        const DONE_OFF: u32 = 0x49;
        const MISSING: u32 = 0xffff_ffff;
        callee_thiscall!(TOUCH, u32, this);
        callee_thiscall!(TOUCH, u32, this + 4);
        callee_thiscall!(TOUCH, u32, this + 8);
        callee_thiscall!(TOUCH, u32, this + 0x0c);
        callee_thiscall!(TOUCH, u32, this + 0x10);
        let h = callee_cdecl!(RESOLVE, u32, relocated(NAME_REF));
        let r = if h == MISSING {
            MISSING
        } else {
            callee_cdecl!(REGISTER, u32, h)
        };
        ((this + DONE_OFF) as *mut u8).write(0);
        r
    }
});
