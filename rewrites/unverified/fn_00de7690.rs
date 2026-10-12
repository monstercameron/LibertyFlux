// original: 0x00DE7690 UIMontageEditorLayout::UIMontageEditorLayout

/// Construct a montage-editor layout by forwarding two constructor words to
/// the intercepted frame constructor, installing the relocated layout vtable,
/// and clearing the child, transition, and auxiliary state fields through
/// `+0x232`. Return `this`; the `thiscall` constructor removes both incoming
/// stack arguments.
lf_checker_rt::export!(thiscall, rw_00DE7690(this: u32, arg0: u32, arg1: u32) -> u32 {
    const FRAME_CONSTRUCTOR: u32 = 1;
    const VTABLE_WORD: u32 = 0;
    const BYTE_STATE: u32 = 0x232;
    const LAYOUT_VTABLE: u32 = 0x00EFEFCC;
    const FIRST_CHILD: u32 = 0x1E0;
    const LOW_STATE: u32 = 0x1F0;
    const SECOND_CHILD: u32 = 0x1E4;
    const MIDDLE_STATE: u32 = 0x1F4;
    const TRANSITION_FLAG: u32 = 0x1EC;
    const STATE_210: u32 = 0x210;
    const STATE_214: u32 = 0x214;
    const STATE_218: u32 = 0x218;
    const STATE_21C: u32 = 0x21C;
    const STATE_20C: u32 = 0x20C;
    const STATE_1F8: u32 = 0x1F8;

    let _ = lf_checker_rt::callee_thiscall!(FRAME_CONSTRUCTOR, u32, this, arg0, arg1);
    unsafe {
        (this.wrapping_add(VTABLE_WORD) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(LAYOUT_VTABLE));
        (this.wrapping_add(FIRST_CHILD) as *mut u32).write_unaligned(0);
        (this.wrapping_add(LOW_STATE) as *mut u32).write_unaligned(0);
        (this.wrapping_add(SECOND_CHILD) as *mut u32).write_unaligned(0);
        (this.wrapping_add(MIDDLE_STATE) as *mut u32).write_unaligned(0);
        (this.wrapping_add(TRANSITION_FLAG) as *mut u32).write_unaligned(0);
        (this.wrapping_add(STATE_210) as *mut u32).write_unaligned(0);
        (this.wrapping_add(STATE_214) as *mut u32).write_unaligned(0);
        (this.wrapping_add(STATE_218) as *mut u32).write_unaligned(0);
        (this.wrapping_add(STATE_21C) as *mut u32).write_unaligned(0);
        (this.wrapping_add(STATE_20C) as *mut u32).write_unaligned(0);
        (this.wrapping_add(STATE_1F8) as *mut u32).write_unaligned(0);
        (this.wrapping_add(BYTE_STATE) as *mut u8).write(0);
    }
    this
});
