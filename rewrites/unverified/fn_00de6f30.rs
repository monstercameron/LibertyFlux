// original: 0x00DE6F30 UITimeOverview::UITimeOverview

/// Construct a time-overview object by forwarding its two stack arguments to
/// the intercepted frame constructor, then install the relocated vtable and
/// clear the timing, child, transition, and auxiliary state words from
/// `+0x1e0` through `+0x228`. Return the object pointer. This `thiscall`
/// constructor removes its two incoming stack arguments on return.
lf_checker_rt::export!(thiscall, rw_00DE6F30(this: u32, arg0: u32, arg1: u32) -> u32 {
    const FRAME_CONSTRUCTOR: u32 = 1;
    const VTABLE_WORD: u32 = 0;
    const LOW_BYTE_STATE: u32 = 0x228;
    const FIRST_CHILD: u32 = 0x1E0;
    const LEFT_STATE: u32 = 0x200;
    const SECOND_CHILD: u32 = 0x1E4;
    const RIGHT_STATE: u32 = 0x204;
    const TIME_VALUE: u32 = 0x1E8;
    const TIME_STATE: u32 = 0x208;
    const TRANSITION_FLAG: u32 = 0x1EC;
    const TRANSITION_STATE: u32 = 0x20C;
    const LOW_STATE: u32 = 0x1F0;
    const LOW_STATE_COPY: u32 = 0x210;
    const MID_STATE: u32 = 0x1F4;
    const MID_STATE_COPY: u32 = 0x214;
    const HIGH_STATE: u32 = 0x1F8;
    const HIGH_STATE_COPY: u32 = 0x218;
    const FINAL_STATE: u32 = 0x1FC;
    const FINAL_STATE_COPY: u32 = 0x21C;
    const ITEM_STATE: u32 = 0x220;
    const AUX_STATE: u32 = 0x224;

    let _ = lf_checker_rt::callee_thiscall!(FRAME_CONSTRUCTOR, u32, this, arg0, arg1);
    unsafe {
        (this.wrapping_add(VTABLE_WORD) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(0x00EFE864));
        (this.wrapping_add(FIRST_CHILD) as *mut u32).write_unaligned(0);
        (this.wrapping_add(LEFT_STATE) as *mut u32).write_unaligned(0);
        (this.wrapping_add(SECOND_CHILD) as *mut u32).write_unaligned(0);
        (this.wrapping_add(RIGHT_STATE) as *mut u32).write_unaligned(0);
        (this.wrapping_add(TIME_VALUE) as *mut u32).write_unaligned(0);
        (this.wrapping_add(TIME_STATE) as *mut u32).write_unaligned(0);
        (this.wrapping_add(TRANSITION_FLAG) as *mut u32).write_unaligned(0);
        (this.wrapping_add(TRANSITION_STATE) as *mut u32).write_unaligned(0);
        (this.wrapping_add(LOW_STATE) as *mut u32).write_unaligned(0);
        (this.wrapping_add(LOW_STATE_COPY) as *mut u32).write_unaligned(0);
        (this.wrapping_add(MID_STATE) as *mut u32).write_unaligned(0);
        (this.wrapping_add(MID_STATE_COPY) as *mut u32).write_unaligned(0);
        (this.wrapping_add(HIGH_STATE) as *mut u32).write_unaligned(0);
        (this.wrapping_add(HIGH_STATE_COPY) as *mut u32).write_unaligned(0);
        (this.wrapping_add(FINAL_STATE) as *mut u32).write_unaligned(0);
        (this.wrapping_add(FINAL_STATE_COPY) as *mut u32).write_unaligned(0);
        (this.wrapping_add(ITEM_STATE) as *mut u32).write_unaligned(0);
        (this.wrapping_add(AUX_STATE) as *mut u32).write_unaligned(0);
        (this.wrapping_add(LOW_BYTE_STATE) as *mut u8).write(0);
    }
    this
});
