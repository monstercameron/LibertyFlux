// original: 0x00CE0A30 init_blend_from_nm_state

/// Reset the state fields initialized for a blend-from-NaturalMotion task.
///
/// `this` is the task object in ECX. The method writes fixed defaults to the
/// status, timing, target and blend fields at offsets `0x14` through `0x68`.
/// The object pointer is a 32-bit game pointer because the original layout is
/// an x86 task object. It takes no stack arguments, makes no calls, and has no
/// meaningful return value.
///
/// Original: 0x00CE0A30 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00ce0a30(this: u32) -> u32 {
    unsafe {
        const STATUS_WORD: u32 = 0x14;
        const STATUS_BYTE: u32 = 0x16;
        const TIMER_DEFAULT: u32 = 0x18;
        const STATE_A: u32 = 0x1c;
        const STATE_B: u32 = 0x20;
        const STATE_C: u32 = 0x24;
        const TARGET_A: u32 = 0x28;
        const TARGET_B: u32 = 0x2c;
        const TARGET_C: u32 = 0x30;
        const TARGET_D: u32 = 0x34;
        const TARGET_E: u32 = 0x38;
        const COUNTER_WORD: u32 = 0x3c;
        const COUNTER_BYTE: u32 = 0x3e;
        const FLAGS: u32 = 0x40;
        const FLAG_BYTE: u32 = 0x44;
        const BLEND_ONE: u32 = 0x48;
        const BLEND_A: u32 = 0x4c;
        const BLEND_B: u32 = 0x50;
        const BLEND_C: u32 = 0x60;
        const BLEND_D: u32 = 0x64;
        const BLEND_E: u32 = 0x68;
        const NO_TARGET: u32 = u32::MAX;

        ((this.wrapping_add(STATUS_WORD)) as *mut u16).write_unaligned(0);
        ((this.wrapping_add(STATUS_BYTE)) as *mut u8).write(0);
        ((this.wrapping_add(TIMER_DEFAULT)) as *mut u32).write_unaligned(2000);
        ((this.wrapping_add(STATE_A)) as *mut u32).write_unaligned(0);
        ((this.wrapping_add(STATE_B)) as *mut u32).write_unaligned(0);
        ((this.wrapping_add(STATE_C)) as *mut u32).write_unaligned(0);
        ((this.wrapping_add(TARGET_A)) as *mut u32).write_unaligned(NO_TARGET);
        ((this.wrapping_add(TARGET_B)) as *mut u32).write_unaligned(NO_TARGET);
        ((this.wrapping_add(TARGET_C)) as *mut u32).write_unaligned(NO_TARGET);
        ((this.wrapping_add(TARGET_D)) as *mut u32).write_unaligned(0);
        ((this.wrapping_add(TARGET_E)) as *mut u32).write_unaligned(NO_TARGET);
        ((this.wrapping_add(COUNTER_WORD)) as *mut u16).write_unaligned(0);
        ((this.wrapping_add(COUNTER_BYTE)) as *mut u8).write(0);
        ((this.wrapping_add(FLAGS)) as *mut u32).write_unaligned(0);
        ((this.wrapping_add(FLAG_BYTE)) as *mut u8).write(0);
        ((this.wrapping_add(BLEND_ONE)) as *mut u32).write_unaligned(0x3f80_0000);
        ((this.wrapping_add(BLEND_A)) as *mut u32).write_unaligned(0);
        ((this.wrapping_add(BLEND_B)) as *mut u32).write_unaligned(0);
        ((this.wrapping_add(BLEND_C)) as *mut u32).write_unaligned(0);
        ((this.wrapping_add(BLEND_D)) as *mut u32).write_unaligned(0);
        ((this.wrapping_add(BLEND_E)) as *mut u32).write_unaligned(0);
        0
    }
});
