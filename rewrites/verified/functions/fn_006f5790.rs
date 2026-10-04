// original: 0x006f5790 arm_from_state
/// Arm the state block from an argument.
///
/// Only states 0 and 1 are accepted; anything else returns 0. Stores the
/// argument's low word and one less at `+0x82`/`+0x84`, runs a setup
/// helper from state 0, moves to state 2 and returns 1.
rt::export!(thiscall, rw_006f5790(this: *mut u8, a1: u32) -> u8 {
    unsafe {
        let state = *this.add(0x20).cast::<u32>();
        if state != 0 && state != 1 {
            return 0;
        }
        *this.add(0x82).cast::<u16>() = a1 as u16;
        *this.add(0x84).cast::<u16>() = a1.wrapping_sub(1) as u16;
        if state == 0 {
            rt::callee_thiscall!(1, u32, this as u32, 0, 0, 1, 0);
        }
        *this.add(0x20).cast::<u32>() = 2;
        1
    }
});
