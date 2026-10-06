// original: 0x00D76330 reset_extents_tail_continue (proposed)

/// Reset the extent block to defaults, then tail-jump to the continuer.
///
/// Zeroes the words at `+0x20`, `+0x24` and `+0x28`, writes the 30.0
/// default (0x41f00000) at `+0x30` and the word 1 at `+0x48`, then
/// tail-jumps to the next stage with `this`. Returns its answer.
/// Thiscall: object in `ecx`, no stack words.
use lf_checker_rt::{callee_thiscall, export};

const NEXT: u32 = 1;

export!(thiscall, rw_00d76330(this: u32) -> u32 {
    unsafe {
        const ZERO0: u32 = 0x20;
        const ZERO1: u32 = 0x24;
        const ZERO2: u32 = 0x28;
        const DEFAULT_OFF: u32 = 0x30;
        const DEFAULT_BITS: u32 = 0x41f0_0000; // 30.0
        const FLAG_OFF: u32 = 0x48;
        ((this + ZERO0) as *mut u32).write_unaligned(0);
        ((this + ZERO1) as *mut u32).write_unaligned(0);
        ((this + ZERO2) as *mut u32).write_unaligned(0);
        ((this + DEFAULT_OFF) as *mut u32).write_unaligned(DEFAULT_BITS);
        ((this + FLAG_OFF) as *mut u16).write_unaligned(1);
        callee_thiscall!(NEXT, u32, this)
    }
});
